class SettingsMenuItem {
  String title;
  String? value;
  String? route; // temp optional
  String icon;
  bool isBreak;
  SettingsMenuItem({
    required this.title,
    this.value = '',
    this.route = '',
    required this.icon,
    this.isBreak = false,
  });
}

// TODO: add details property
class NetworkListItem {
  String title;
  String? subTitle; // status - connected/connecting/disconnected/saved..
  String? infoRoute; // temp optional
  String? connectNetworkRoute;
  String icon;
  NetworkDetailsType networkDetails;
  NetworkListItem(
      {required this.title,
      this.subTitle = '',
      this.infoRoute = '',
      this.connectNetworkRoute = '',
      required this.icon,
      required this.networkDetails});
}

// connected
class NetworkDetailsType {
  String networkName;
  String status;
  String frequency;
  String ipAddress;
  String macAddress;
  String securityType; // e.g., WPA2, WPA3, etc.
  int signalStrength;
  NetworkDetailsType(
      {required this.networkName,
      required this.status,
      required this.frequency,
      required this.ipAddress,
      required this.macAddress,
      required this.securityType,
      required this.signalStrength});
}

enum WifiStatus {
  connected,
  connecting,
  disconnected,
  disconnecting,
  saved,
  unknown
}

Map<WifiStatus, String> wifiStatusToString = {
  WifiStatus.unknown: '',
  WifiStatus.connected: 'Connected',
  WifiStatus.connecting: 'Connecting',
  WifiStatus.disconnected: 'Disconnected',
  WifiStatus.disconnecting: 'Disconnecting',
  WifiStatus.saved: 'Saved'
};

class RouteMeta {
  final String backTitle;

  const RouteMeta({required this.backTitle});
}
