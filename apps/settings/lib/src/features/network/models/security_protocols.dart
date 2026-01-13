import 'package:mechanix_settings/src/features/network/models/types.dart';
import 'package:nm/nm.dart';

WirelessProtocol getWirelessProtocol(
    List<NetworkManagerWifiAccessPointSecurityFlag> flags) {
  final Set<NetworkManagerWifiAccessPointSecurityFlag> setFlags = flags.toSet();
  return _getSecurityType(setFlags);
}

WirelessProtocol _getSecurityType(
    Set<NetworkManagerWifiAccessPointSecurityFlag> flags) {
  // WEP
  if (flags.any((f) =>
      f == NetworkManagerWifiAccessPointSecurityFlag.pairWep40 ||
      f == NetworkManagerWifiAccessPointSecurityFlag.pairWep104 ||
      f == NetworkManagerWifiAccessPointSecurityFlag.groupWep40 ||
      f == NetworkManagerWifiAccessPointSecurityFlag.groupWep104)) {
    return WirelessProtocol.wep;
  }

  // Check for Enterprise first (802.1X)
  bool isEnterprise = flags
      .contains(NetworkManagerWifiAccessPointSecurityFlag.keyManagement802_1X);

  // WPA3 Enterprise
  if (isEnterprise &&
      flags.contains(
          NetworkManagerWifiAccessPointSecurityFlag.keyManagementSae)) {
    return WirelessProtocol.wpa3Enterprise;
  }

  // WPA3 Personal
  if (flags
      .contains(NetworkManagerWifiAccessPointSecurityFlag.keyManagementSae)) {
    return WirelessProtocol.wpa3;
  }

  // WPA3 OWE (treated as WPA3)
  if (flags.contains(
          NetworkManagerWifiAccessPointSecurityFlag.keyManagementOwe) ||
      flags.contains(
          NetworkManagerWifiAccessPointSecurityFlag.keyManagementOweTm)) {
    return WirelessProtocol.wpa3;
  }

  // Mixed WPA2 + WPA3 (e.g., PSK + SAE + CCMP)
  if (flags.contains(
          NetworkManagerWifiAccessPointSecurityFlag.keyManagementPsk) &&
      flags.contains(
          NetworkManagerWifiAccessPointSecurityFlag.keyManagementSae) &&
      flags.contains(NetworkManagerWifiAccessPointSecurityFlag.pairCcmp)) {
    return isEnterprise
        ? WirelessProtocol.wpa3Enterprise
        : WirelessProtocol.wpa2Wpa3;
  }

  // Enterprise (WPA/WPA2)
  if (isEnterprise) {
    bool hasCcmp =
        flags.contains(NetworkManagerWifiAccessPointSecurityFlag.pairCcmp) ||
            flags.contains(NetworkManagerWifiAccessPointSecurityFlag.groupCcmp);
    bool hasTkip =
        flags.contains(NetworkManagerWifiAccessPointSecurityFlag.pairTkip) ||
            flags.contains(NetworkManagerWifiAccessPointSecurityFlag.groupTkip);

    if (hasCcmp && !hasTkip) {
      return WirelessProtocol.wpa2Enterprise;
    } else if (hasTkip && !hasCcmp) {
      return WirelessProtocol.wpaEnterprise;
    } else {
      // Both or neither: Default to WPA2
      return WirelessProtocol.wpa2Enterprise;
    }
  }

  // WPA/WPA2 Personal (PSK)
  if (flags
      .contains(NetworkManagerWifiAccessPointSecurityFlag.keyManagementPsk)) {
    bool hasCcmp =
        flags.contains(NetworkManagerWifiAccessPointSecurityFlag.pairCcmp) ||
            flags.contains(NetworkManagerWifiAccessPointSecurityFlag.groupCcmp);
    bool hasTkip =
        flags.contains(NetworkManagerWifiAccessPointSecurityFlag.pairTkip) ||
            flags.contains(NetworkManagerWifiAccessPointSecurityFlag.groupTkip);

    // Both CCMP and TKIP (mixed WPA + WPA2) - prioritize WPA2
    if (hasCcmp && hasTkip) {
      return WirelessProtocol.wpa2;
    }

    // Only CCMP (WPA2)
    if (hasCcmp && !hasTkip) {
      return WirelessProtocol.wpa2;
    }

    // Only TKIP (WPA)
    if (hasTkip && !hasCcmp) {
      return WirelessProtocol.wpa;
    }

    // Default to WPA2 if no cipher info
    return WirelessProtocol.wpa2;
  }

  // No security flags
  return WirelessProtocol.none;
}

// NEW APPROACH


// WirelessProtocol getWirelessProtocol(
//     List<NetworkManagerWifiAccessPointSecurityFlag> flags) {
//   final Set<NetworkManagerWifiAccessPointSecurityFlag> setFlags = flags
//           .where((flag) => flag is NetworkManagerWifiAccessPointSecurityFlag)
//           .map((flag) => flag as NetworkManagerWifiAccessPointSecurityFlag)
//           .toSet() ??
//       {};

//   final securityFlag = _getSecurityType(setFlags);

//   return securityFlag;
// }

// WirelessProtocol _getSecurityType(
//     Set<NetworkManagerWifiAccessPointSecurityFlag> flags) {
//   // WEP
//   if (flags.any((f) => f.name.contains('Wep'))) {
//     return WirelessProtocol.wep;
//   }

//   // Check for Enterprise first (802.1X)
//   bool isEnterprise = flags
//       .contains(NetworkManagerWifiAccessPointSecurityFlag.keyManagement802_1X);

//   // WPA3 Enterprise
//   if (isEnterprise &&
//       flags.contains(
//           NetworkManagerWifiAccessPointSecurityFlag.keyManagementSae)) {
//     return WirelessProtocol.wpa3Enterprise;
//   }

//   // WPA3 Personal
//   if (flags
//       .contains(NetworkManagerWifiAccessPointSecurityFlag.keyManagementSae)) {
//     return WirelessProtocol.wpa3;
//   }

//   // WPA3 OWE (treated as WPA3)
//   if (flags.contains(
//           NetworkManagerWifiAccessPointSecurityFlag.keyManagementOwe) ||
//       flags.contains(
//           NetworkManagerWifiAccessPointSecurityFlag.keyManagementOweTm)) {
//     return WirelessProtocol.wpa3;
//   }

//   // Enterprise (WPA/WPA2)
//   if (isEnterprise) {
//     // Check if CCMP is present (indicates WPA2)
//     bool hasCcmp =
//         flags.contains(NetworkManagerWifiAccessPointSecurityFlag.pairCcmp) ||
//             flags.contains(NetworkManagerWifiAccessPointSecurityFlag.groupCcmp);
//     bool hasTkip =
//         flags.contains(NetworkManagerWifiAccessPointSecurityFlag.pairTkip) ||
//             flags.contains(NetworkManagerWifiAccessPointSecurityFlag.groupTkip);

//     if (hasCcmp && !hasTkip) {
//       return WirelessProtocol.wpa2Enterprise;
//     } else if (hasTkip && !hasCcmp) {
//       return WirelessProtocol.wpaEnterprise;
//     } else {
//       // Both TKIP and CCMP present, or neither
//       return WirelessProtocol.wpa2Enterprise; // Default to WPA2 for mixed mode
//     }
//   }

//   // WPA/WPA2 Personal (PSK)
//   if (flags
//       .contains(NetworkManagerWifiAccessPointSecurityFlag.keyManagementPsk)) {
//     bool hasCcmp =
//         flags.contains(NetworkManagerWifiAccessPointSecurityFlag.pairCcmp) ||
//             flags.contains(NetworkManagerWifiAccessPointSecurityFlag.groupCcmp);
//     bool hasTkip =
//         flags.contains(NetworkManagerWifiAccessPointSecurityFlag.pairTkip) ||
//             flags.contains(NetworkManagerWifiAccessPointSecurityFlag.groupTkip);

//     // Both CCMP and TKIP (mixed mode WPA + WPA2)
//     if (hasCcmp && hasTkip) {
//       return WirelessProtocol.wpa2Wpa3; // Or could be WPA+WPA2
//     }

//     // Only CCMP (WPA2)
//     if (hasCcmp && !hasTkip) {
//       return WirelessProtocol.wpa2;
//     }

//     // Only TKIP (WPA)
//     if (hasTkip && !hasCcmp) {
//       return WirelessProtocol.wpa;
//     }

//     // Default to WPA2 if no cipher info
//     return WirelessProtocol.wpa2;
//   }

//   // No security flags
//   return WirelessProtocol.none;
// }

// NEW APPROACH

// import 'package:mechanix_settings/src/features/network/models/types.dart';
// import 'package:nm/nm.dart';

// WirelessProtocol getSecurityType(
//     Set<NetworkManagerWifiAccessPointSecurityFlag> flags) {
//   // WEP
//   if (flags.any((f) => f.name.contains('Wep'))) {
//     // return 'WEP (Insecure)';
//     return WirelessProtocol.wep;
//   }

//   // WPA3 Personal
//   if (flags
//       .contains(NetworkManagerWifiAccessPointSecurityFlag.keyManagementSae)) {
//     // return 'WPA3 Personal';
//     return WirelessProtocol.wpa3;
//   }

//   // WPA3 OWE
//   if (flags
//       .contains(NetworkManagerWifiAccessPointSecurityFlag.keyManagementOwe)) {
//     // return 'WPA3 OWE';
//     return WirelessProtocol.wpa3;
//   }

//   // WPA3 OWE Transition Mode
//   if (flags
//       .contains(NetworkManagerWifiAccessPointSecurityFlag.keyManagementOweTm)) {
//     // return 'WPA3 OWE-TM';
//     return WirelessProtocol.wpa3;
//   }

//   // Enterprise
//   if (flags.contains(
//       NetworkManagerWifiAccessPointSecurityFlag.keyManagement802_1X)) {
//     // return 'WPA/WPA2 Enterprise';
//     return WirelessProtocol.wpa3Enterprise;
//   }

//   // WPA/WPA2 Personal (PSK)
//   if (flags
//       .contains(NetworkManagerWifiAccessPointSecurityFlag.keyManagementPsk)) {
//     // Check cipher type to distinguish WPA from WPA2
//     if (flags.contains(NetworkManagerWifiAccessPointSecurityFlag.pairCcmp) ||
//         flags.contains(NetworkManagerWifiAccessPointSecurityFlag.groupCcmp)) {
//       // return 'WPA2 Personal';
//       return WirelessProtocol.wpa2;
//     }
//     if (flags.contains(NetworkManagerWifiAccessPointSecurityFlag.pairTkip) ||
//         flags.contains(NetworkManagerWifiAccessPointSecurityFlag.groupTkip)) {
//       // return 'WPA Personal';
//       return WirelessProtocol.wpa;
//     }
//     // return 'WPA/WPA2 Personal';
//     return WirelessProtocol.wpa2Wpa3;
//   }

//   return WirelessProtocol.none;
// }




// NEW APPROACH


// import 'package:nm/nm.dart';  // Import the nm.dart package

// enum WirelessProtocol {
//   none,
//   wep,
//   wpa,  // WPA1
//   wpa2Wpa3,  // Mixed WPA2/WPA3
//   wpa2,
//   wpa3,
//   wpaEnterprise,  // WPA1 Enterprise
//   wpa2Enterprise,
//   wpa3Enterprise,
// }

// // Function to map flags to your enum
// WirelessProtocol mapToWirelessProtocol(Set<NetworkManagerWifiAccessPointSecurityFlag> flags) {
//   // Helper to check if any flag in a list is present
//   bool hasAny(List<NetworkManagerWifiAccessPointSecurityFlag> flagList) {
//     return flagList.any((flag) => flags.contains(flag));
//   }

//   // No flags = no security
//   if (flags.isEmpty) return WirelessProtocol.none;

//   // WEP: Any WEP flags
//   if (hasAny([NetworkManagerWifiAccessPointSecurityFlag.pairWep40,
//               NetworkManagerWifiAccessPointSecurityFlag.pairWep104,
//               NetworkManagerWifiAccessPointSecurityFlag.groupWep40,
//               NetworkManagerWifiAccessPointSecurityFlag.groupWep104])) {
//     return WirelessProtocol.wep;
//   }

//   // Check for 802.1X (Enterprise)
//   bool isEnterprise = flags.contains(NetworkManagerWifiAccessPointSecurityFlag.keyManagement802_1X);

//   // Check for WPA3 indicators
//   bool hasWpa3 = hasAny([NetworkManagerWifiAccessPointSecurityFlag.keyManagementSae,
//                          NetworkManagerWifiAccessPointSecurityFlag.keyManagementOwe,
//                          NetworkManagerWifiAccessPointSecurityFlag.keyManagementOweTm]);

//   // Check for WPA2 (CCMP + PSK, no TKIP)
//   bool hasWpa2 = flags.contains(NetworkManagerWifiAccessPointSecurityFlag.pairCcmp) &&
//                  flags.contains(NetworkManagerWifiAccessPointSecurityFlag.keyManagementPsk) &&
//                  !flags.contains(NetworkManagerWifiAccessPointSecurityFlag.pairTkip);

//   // Check for WPA1 (TKIP + PSK, no CCMP)
//   bool hasWpa1 = flags.contains(NetworkManagerWifiAccessPointSecurityFlag.pairTkip) &&
//                  flags.contains(NetworkManagerWifiAccessPointSecurityFlag.keyManagementPsk) &&
//                  !flags.contains(NetworkManagerWifiAccessPointSecurityFlag.pairCcmp);

//   // Determine protocol
//   if (hasWpa3 && hasWpa2) {
//     // Mixed WPA2/WPA3
//     return isEnterprise ? WirelessProtocol.wpa3Enterprise : WirelessProtocol.wpa2Wpa3;
//   } else if (hasWpa3) {
//     return isEnterprise ? WirelessProtocol.wpa3Enterprise : WirelessProtocol.wpa3;
//   } else if (hasWpa2) {
//     return isEnterprise ? WirelessProtocol.wpa2Enterprise : WirelessProtocol.wpa2;
//   } else if (hasWpa1) {
//     return isEnterprise ? WirelessProtocol.wpaEnterprise : WirelessProtocol.wpa;
//   }

//   // Fallback: If PSK or other flags but no clear match, assume WPA2 (common default)
//   if (flags.contains(NetworkManagerWifiAccessPointSecurityFlag.keyManagementPsk)) {
//     return isEnterprise ? WirelessProtocol.wpa2Enterprise : WirelessProtocol.wpa2;
//   }

//   // If nothing matches, default to none
//   return WirelessProtocol.none;
// }