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

class InitializeBluetooth extends BluetoothEvent {}

class StartDiscovery extends BluetoothEvent {}

class StopDiscovery extends BluetoothEvent {}

class RefreshDeviceList extends BluetoothEvent {}

class GetAdapterAlias extends BluetoothEvent {}

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
  // SelectDevice(this.selectedDevice);
  SelectDevice(this.selectedDevice) {}
}