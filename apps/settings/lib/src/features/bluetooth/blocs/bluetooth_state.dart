import 'package:bluez/bluez.dart';
import 'package:equatable/equatable.dart';

class BluetoothState extends Equatable {
  final bool isPowered;
  final bool loading;
  final List<BlueZDevice> devices;
  final BlueZDevice? selectedDevice;
  final String? deviceState;
  final String? error;
  final bool isDiscoveryEnabled;

  final String? adapterAlias;

  const BluetoothState({
    required this.isPowered,
    this.loading = false,
    required this.devices,
    this.selectedDevice,
    this.deviceState,
    this.error,
    this.adapterAlias,
    this.isDiscoveryEnabled = false,
  });

  BluetoothState copyWith(
      {bool? isPowered,
      bool? loading,
      String? error,
      List<BlueZDevice>? devices,
      String? deviceState,
      BlueZDevice? selectedDevice,
      String? adapterAlias,
      bool? isDiscoveryEnabled}) {
    return BluetoothState(
        isPowered: isPowered ?? this.isPowered,
        loading: loading ?? this.loading,
        devices: devices ?? this.devices,
        error: error,
        deviceState: deviceState ?? this.deviceState,
        selectedDevice: selectedDevice ?? this.selectedDevice,
        adapterAlias: adapterAlias ?? this.adapterAlias,
        isDiscoveryEnabled: isDiscoveryEnabled ?? this.isDiscoveryEnabled);
  }

  @override
  List<Object?> get props => [
        isPowered,
        loading,
        error,
        deviceState,
        adapterAlias,
        devices,
        isDiscoveryEnabled
      ];
}
