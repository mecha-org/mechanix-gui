import 'package:bluez/bluez.dart';

abstract class BluetoothRepository {
  Future<void> init(); // connect BlueZ client
  Stream<bool> get bluezAdapterPowerStream;
  Stream<bool> get bluezAdapterDiscoverableStream;

  Future<bool> isBluetoothEnabled();
  Future<bool> setPower(bool enable);

  Future<BlueZAdapter> getBluezAdapter();  
  Future<String> setAdapterAlias(String alias);

  Future<void> startDiscovery();
  Future<void> stopDiscovery();
  Future<List<BlueZDevice>> getDevices();
  Future<void> pair(String address);
  Future<void> connect(String address);
  Future<void> disconnect(String address);
  Future<void> remove(String address);

  Future<Stream<BlueZDevice>> onDeviceAdded();
  Future<Stream<BlueZDevice>> onDeviceRemoved();
  Future<void> setDiscoverable(bool value);

  Future<void> close();

}
