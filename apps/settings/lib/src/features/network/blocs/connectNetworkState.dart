import 'package:nm/nm.dart';

class ConnectNetworkState {
  final String password;
  final bool obscurePassword;
  final bool isConnecting;
  final bool isConnected;
  final String? error;

  ConnectNetworkState({
    this.password = '',
    this.obscurePassword = true,
    this.isConnecting = false,
    this.isConnected = false,
    this.error,
  });

  ConnectNetworkState copyWith({
    String? password,
    bool? obscurePassword,
    bool? isConnecting,
    bool? isConnected,
    NetworkManagerAccessPoint? accessPoint,
    String? error,
  }) {
    return ConnectNetworkState(
      password: password ?? this.password,
      obscurePassword: obscurePassword ?? this.obscurePassword,
      isConnecting: isConnecting ?? this.isConnecting,
      isConnected: isConnected ?? this.isConnected,
      error: error ?? this.error,
    );
  }
}
