import 'package:equatable/equatable.dart';
import 'package:nm/nm.dart';

abstract class ConnectNetworkEvent extends Equatable {
  @override
  List<Object> get props => [];
}

class PasswordChanged extends ConnectNetworkEvent {
  final String password;
  PasswordChanged(this.password);
}

class UsernameChanged extends ConnectNetworkEvent {
  final String username;
  UsernameChanged(this.username);
}

class TogglePasswordVisibility extends ConnectNetworkEvent {}

class ConnectToNetwork extends ConnectNetworkEvent {
  final NetworkManagerAccessPoint accessPoint;
  ConnectToNetwork(this.accessPoint);
}

class ConnectToHiddenNetwork extends ConnectNetworkEvent {
  final String ssid;
  final String password;
  ConnectToHiddenNetwork(this.ssid, this.password);
}

class Error extends ConnectNetworkEvent {
  final String error;
  Error(this.error);
}

class DeviceConnectionStateEvent extends ConnectNetworkEvent {
  final NetworkManagerDeviceState deviceSate;
  DeviceConnectionStateEvent(this.deviceSate);

  @override
  List<Object> get props => [deviceSate];
}
