import 'package:mechanix_settings/src/features/network/models/access_points.dart';
import 'package:nm/nm.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/select/select_type.dart';

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

enum ConfigureDNS {
  automaticDhcp,
  static,
}

final List<SelectOption<ConfigureDNS>> dnsOptions = [
  ConfigureDNS.automaticDhcp.toSelectOption('Automatic (DHCP)'),
  ConfigureDNS.static.toSelectOption('Static'),
];

enum DnsProxyConfiguration {
  off,
  automatic,
  static,
}

final List<SelectOption<DnsProxyConfiguration>> dnsProxyOptions = [
  DnsProxyConfiguration.off.toSelectOption('Off'),
  DnsProxyConfiguration.automatic.toSelectOption('Automatic'),
  DnsProxyConfiguration.static.toSelectOption('Static'),
];

enum WirelessProtocol {
  none,
  wep,
  wpa,
  wpa2Wpa3,
  wpa2,
  wpa3,
  wpaEnterprise,
  wpa2Enterprise,
  wpa3Enterprise,
}

final List<SelectOption<WirelessProtocol>> wirelessProtocolOptions = [
  WirelessProtocol.none.toSelectOption('None'),
  WirelessProtocol.wep.toSelectOption('WEP'),
  WirelessProtocol.wpa.toSelectOption('WPA'),
  WirelessProtocol.wpa2Wpa3.toSelectOption('WPA2/WPA3'),
  WirelessProtocol.wpa2.toSelectOption('WPA2'),
  WirelessProtocol.wpa3.toSelectOption('WPA3'),
  WirelessProtocol.wpaEnterprise.toSelectOption('WPA Enterprise'),
  WirelessProtocol.wpa2Enterprise.toSelectOption('WPA2 Enterprise'),
  WirelessProtocol.wpa3Enterprise.toSelectOption('WPA3 Enterprise'),
];

enum SignalLevel { high, medium, low, none }

class ActivatingNetwork {
  final List<int> ssid;
  final bool isActivate;
  final AccessPoints? accessPoint;
  final NetworkManagerActiveConnectionState deviceState;

  const ActivatingNetwork({
    required this.ssid,
    required this.isActivate,
    required this.deviceState,
    this.accessPoint,
  });
}
