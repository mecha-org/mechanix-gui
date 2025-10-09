import 'package:nm/nm.dart';

class SavedNetworks {
  String ssid; // status - connected/connecting/disconnected/saved..
  String icon;
  bool connected;
  NetworkManagerAccessPoint? accessPoint;
  SavedNetworks(
      {this.ssid = '',
      required this.icon,
      required this.connected,
      this.accessPoint});
}

class SavedWirelessNetwork {
  final String? ssid;            
  final String? security;        

  SavedWirelessNetwork({
    this.ssid,
    this.security,
  });
}