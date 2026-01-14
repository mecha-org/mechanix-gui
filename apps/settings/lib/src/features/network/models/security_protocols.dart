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
