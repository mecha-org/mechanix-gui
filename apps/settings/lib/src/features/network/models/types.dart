import 'package:nm/nm.dart';

enum WifiStatus {
  connected,
  connecting,
  disconnected,
  disconnecting,
  saved,
  unknown
}

Map<WifiStatus, String> wifiStatusToString = {
  WifiStatus.unknown: '-',
  WifiStatus.connected: 'Connected',
  WifiStatus.connecting: 'Connecting',
  WifiStatus.disconnected: 'Disconnected',
  WifiStatus.disconnecting: 'Disconnecting',
  WifiStatus.saved: 'Saved'
};

class StreamAndDevice {
  Stream<List<String>> props;
  NetworkManagerDevice device;
  StreamAndDevice(this.props, this.device);
}

enum IpModes { static, dhcp }

Map<IpModes, String> ipModesList = {
  IpModes.dhcp: "Automatic (DHCP)",
  IpModes.static: "Static",
};

