import 'package:equatable/equatable.dart';
import 'package:nm/nm.dart';

class ConnectNetworkState extends Equatable {
  final String username;
  final String password;
  final bool obscurePassword;
  final bool isConnecting;
  final bool isConnected;
  final String? error;
  final NetworkManagerDeviceState? deviceState;

  const ConnectNetworkState({
    this.username = '',
    this.password = '',
    this.obscurePassword = true,
    this.isConnecting = false,
    this.isConnected = false,
    this.error,
    this.deviceState,
  });

  ConnectNetworkState copyWith({
    String? username,
    String? password,
    bool? obscurePassword,
    bool? isConnecting,
    bool? isConnected,
    NetworkManagerAccessPoint? accessPoint,
    String? error,
    NetworkManagerDeviceState? deviceState,
  }) {
    return ConnectNetworkState(
      username: username ?? this.username,
      password: password ?? this.password,
      obscurePassword: obscurePassword ?? this.obscurePassword,
      isConnecting: isConnecting ?? this.isConnecting,
      isConnected: isConnected ?? this.isConnected,
      error: error ?? this.error,
      deviceState: deviceState ?? this.deviceState,
    );
  }

  @override
  List<Object?> get props => [
        username,
        password,
        obscurePassword,
        isConnecting,
        isConnected,
        error,
        deviceState,
      ];
}
