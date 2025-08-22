
import 'package:bluez/bluez.dart';

abstract class BluetoothRepository {
  Future<bool> isBluetoothEnabled();
  Future<bool> setPower(bool enable);

  Future<String> getAdapterName();
  Future<String> getAdapterAlias();
  Future<String> setAdapterAlias(String alias);

  Future<void> startDiscovery();
  Future<void> stopDiscovery();
  Future<List<BlueZDevice>> getDevices();
  Future<void> pair(String address);
  Future<void> connect(String address);
  Future<void> disconnect(String address);
  Future<void> remove(String address);

  Future<Stream<List<String>>> streamBluetoothEvents();
  Future<Stream<bool>> onDeviceAdded();
  Future<Stream<bool>> onDeviceRemoved();
}
