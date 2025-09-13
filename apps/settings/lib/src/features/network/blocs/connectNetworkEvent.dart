import 'package:nm/nm.dart';

abstract class ConnectNetworkEvent {}

class PasswordChanged extends ConnectNetworkEvent {
  final String password;
  PasswordChanged(this.password);
}

class TogglePasswordVisibility extends ConnectNetworkEvent {}

class ConnectToNetwork extends ConnectNetworkEvent {
  final NetworkManagerAccessPoint accessPoint;
  ConnectToNetwork(this.accessPoint);
}

class ConnectToUnknownNetwork extends ConnectNetworkEvent {
  final String ssid;
  final String password;
  ConnectToUnknownNetwork(this.ssid, this.password);
}
class Error extends ConnectNetworkEvent {
  final String error;
  Error(this.error);
}
