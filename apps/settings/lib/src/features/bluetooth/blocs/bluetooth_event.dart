import 'package:bluez/bluez.dart';
import 'package:equatable/equatable.dart';

abstract class BluetoothEvent extends Equatable {
  @override
  List<Object?> get props => [];
}

class ToggleBluetooth extends BluetoothEvent {
  final bool enabled;
  ToggleBluetooth(this.enabled);

  @override
  List<Object?> get props => [enabled];
}

class BluetoothPowerChanged extends BluetoothEvent {
  final bool enabled;
  BluetoothPowerChanged(this.enabled);

  @override
  List<Object?> get props => [enabled];
}

class DiscoverableChanged extends BluetoothEvent {
  final bool enabled;
  DiscoverableChanged(this.enabled);

  @override
  List<Object?> get props => [enabled];
}

class InitBluetooth extends BluetoothEvent {}

class InitializeBluetooth extends BluetoothEvent {}

class GetAdapterDetails extends BluetoothEvent {}

class StartDiscovery extends BluetoothEvent {}

class StopDiscovery extends BluetoothEvent {}

class GetDeviceList extends BluetoothEvent {}

class RefreshDeviceList extends BluetoothEvent {}

class RenameAdapterEvent extends BluetoothEvent {
  final String newName;
  RenameAdapterEvent(this.newName);
}

class PairDevice extends BluetoothEvent {
  final String address;
  PairDevice(this.address);
}

class ConnectDevice extends BluetoothEvent {
  final String address;
  ConnectDevice(this.address);
}

class DisconnectDevice extends BluetoothEvent {
  final String address;
  DisconnectDevice(this.address);
}

class RemoveDevice extends BluetoothEvent {
  final String address;
  RemoveDevice(this.address);
}

class SelectDevice extends BluetoothEvent {
  final BlueZDevice selectedDevice;
  SelectDevice(this.selectedDevice);

  @override
  List<Object?> get props => [selectedDevice];
}

class DiscoveryEnabled extends BluetoothEvent {
  final bool isDiscoverable;
  DiscoveryEnabled(this.isDiscoverable);
}

class BluetoothDevicesAdded extends BluetoothEvent {
  final BlueZDevice device;
  BluetoothDevicesAdded(this.device);

  @override
  List<Object?> get props => [device];
}

class BluetoothDevicesRemoved extends BluetoothEvent {
  final BlueZDevice device;
  BluetoothDevicesRemoved(this.device);

  @override
  List<Object?> get props => [device];
}

class BluetoothConnectingEvent extends BluetoothEvent {
  final String address;
  final bool connectionLoading;

  BluetoothConnectingEvent(
      {required this.address, required this.connectionLoading});

  @override
  List<Object?> get props => [address, connectionLoading];
}
