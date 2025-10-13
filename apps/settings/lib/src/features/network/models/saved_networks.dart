import 'package:nm/nm.dart';

class SavedNetworks {
  String ssid; 
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
  final String? macAddress; 
  final String? ipv4Method;
  final String? autoConnect;       
  // final String? type;        // wireless or something else type

  SavedWirelessNetwork({
    this.ssid,
    this.security,
    this.macAddress,
    this.ipv4Method,
    this.autoConnect,
    // this.type,
  });
}