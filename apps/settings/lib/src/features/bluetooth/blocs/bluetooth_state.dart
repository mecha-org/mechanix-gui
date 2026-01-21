import 'package:bluez/bluez.dart';
import 'package:equatable/equatable.dart';
import 'package:mechanix_settings/src/features/bluetooth/models/types.dart';

class BluetoothState extends Equatable {
  final bool isPowered;
  final bool loading;
  final BluetoothConnection? connection;
  final List<BlueZDevice> devices;
  final BlueZDevice? selectedDevice;
  final String? deviceState;
  final String? error;
  final bool isDiscoveryEnabled;
  final BluetoothAdapter? bluetoothAdapter;

  const BluetoothState({
    required this.isPowered,
    this.loading = false,
    this.connection,
    required this.devices,
    this.selectedDevice,
    this.deviceState,
    this.error,
    this.bluetoothAdapter,
    this.isDiscoveryEnabled = false,
  });

  BluetoothState copyWith({
    bool? isPowered,
    bool? loading,
    String? error,
    List<BlueZDevice>? devices,
    String? deviceState,
    BlueZDevice? selectedDevice,
    BluetoothAdapter? bluetoothAdapter,
    bool? isDiscoveryEnabled,
    BluetoothConnection? connection,
  }) {
    return BluetoothState(
      isPowered: isPowered ?? this.isPowered,
      loading: loading ?? this.loading,
      connection: connection ?? this.connection,
      devices: devices ?? this.devices,
      error: error,
      deviceState: deviceState ?? this.deviceState,
      selectedDevice: selectedDevice ?? this.selectedDevice,
      bluetoothAdapter: bluetoothAdapter ?? this.bluetoothAdapter,
      isDiscoveryEnabled: isDiscoveryEnabled ?? this.isDiscoveryEnabled,
    );
  }

  @override
  List<Object?> get props => [
        isPowered,
        loading,
        connection,
        error,
        selectedDevice,
        deviceState,
        bluetoothAdapter,
        devices,
        isDiscoveryEnabled,
      ];
}
