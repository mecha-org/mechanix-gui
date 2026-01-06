import 'package:mechanix_settings/src/features/network/models/types.dart';
import 'package:nm/nm.dart';

WirelessProtocol getWirelessProtocol(
    List<NetworkManagerWifiAccessPointSecurityFlag> flags) {
  final Set<NetworkManagerWifiAccessPointSecurityFlag> setFlags = flags
          .where((flag) => flag is NetworkManagerWifiAccessPointSecurityFlag)
          .map((flag) => flag as NetworkManagerWifiAccessPointSecurityFlag)
          .toSet() ??
      {};

  final securityFlag = _getSecurityType(setFlags);

  return securityFlag;
}

WirelessProtocol _getSecurityType(
    Set<NetworkManagerWifiAccessPointSecurityFlag> flags) {
  // WEP
  if (flags.any((f) => f.name.contains('Wep'))) {
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

  // Enterprise (WPA/WPA2)
  if (isEnterprise) {
    // Check if CCMP is present (indicates WPA2)
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
      // Both TKIP and CCMP present, or neither
      return WirelessProtocol.wpa2Enterprise; // Default to WPA2 for mixed mode
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

    // Both CCMP and TKIP (mixed mode WPA + WPA2)
    if (hasCcmp && hasTkip) {
      return WirelessProtocol.wpa2Wpa3; // Or could be WPA+WPA2
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
