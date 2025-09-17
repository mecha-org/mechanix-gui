import 'package:bluez/bluez.dart';
import 'package:logger/web.dart';

import 'bluetooth_repository.dart';

class BluetoothRepositoryImpl implements BluetoothRepository {
  final BlueZClient client =
      BlueZClient(); // BlueZ client for managing Bluetooth connections
  late BlueZAdapter adapter; // Bluetooth adapter - central device

  bool connected = false; // for client connection
  final logger = Logger();

  BluetoothRepositoryImpl() {
    _init();
  }

  Future<void> _init() async {
    if (!connected) {
      await client.connect();
      connected = true;
    }

    if (client.adapters.isNotEmpty) {
      adapter = client.adapters.first;
    } else {
      logger.w("No Bluetooth adapters found. Ensure Bluetooth is enabled.");
      // throw Exception("No Bluetooth adapters found. Ensure Bluetooth is enabled.");
    }

    logger.i("Bluetooth Repository Initialized with adapter: ${adapter.name}");
  }

  Future<void> _ensureConnected() async {
    if (!connected) {
      await client.connect();
      connected = true;
    }
  }

  @override
  Future<Stream<List<String>>> streamBluetoothEvents() async {
    logger.i('Subscribing to Bluetooth events');
    var client = BlueZClient();
    await client.connect();

    return adapter.propertiesChanged;
  }

  @override
  Future<bool> isBluetoothEnabled() async {
    var client = BlueZClient();
    await client.connect();

    var result = client.adapters.first.powered;
    logger.i("Bluetooth enable status: $result");

    return result;
  }

  @override
  Future<bool> setPower(bool enable) async {
    await _ensureConnected();

    await adapter.setPowered(enable);
    return enable;
  }

  @override
  Future<String> getAdapterName() async {
    _ensureConnected();
    return adapter.name;
  }

  @override
  Future<String> getAdapterAlias() async {
    await _ensureConnected();
    return adapter.alias;
  }

  @override
  Future<String> setAdapterAlias(String alias) async {
    _ensureConnected();
    await adapter.setAlias(alias);
    return alias;
  }

  @override
  Future<void> startDiscovery() async {
    if (!adapter.discovering) await adapter.startDiscovery();
  }

  @override
  Future<void> stopDiscovery() async {
    if (adapter.discovering) await adapter.stopDiscovery();
  }

  @override
  Future<List<BlueZDevice>> getDevices() async {
    await _ensureConnected();
    var devices = client.devices;
    logger.i("Devices: $devices");
    return devices;
  }

  @override
  Future<void> pair(String address) async {
    var device = client.devices.firstWhere((d) => d.address == address);
    // await device.setTrusted(true);
    await device.pair();
    logger.i("Device paired: $address");
  }

  @override
  Future<void> connect(String address) async {
    var device = client.devices.firstWhere((d) => d.address == address);
    await device.connect();
    logger.i("Device connected: $address");
  }

  @override
  Future<void> disconnect(String address) async {
    var device = client.devices.firstWhere((d) => d.address == address);
    await device.disconnect();
    logger.i("Device disconnected: $address");
  }

  @override
  Future<void> remove(String address) async {
    var device = client.devices.firstWhere((d) => d.address == address);
    await adapter.removeDevice(device);
    logger.i("Device removed: $address");
  }

  @override
  Future<Stream<bool>> onDeviceAdded() async {
    await _ensureConnected();
    return client.deviceAdded.map((device) {
      logger.i("Device added: ${device.name} --- ${device.address}");
      return true;
    });
  }

  @override
  Future<Stream<bool>> onDeviceRemoved() async {
    await _ensureConnected();

    return client.deviceRemoved.map((device) {
      logger.i("Device removed: ${device.name} ${device.icon}");
      return true;
    });
  }

  @override
  Future<bool> isDeviceDiscoverable() async {
    return adapter.discoverable;
  }

  @override
  Future<void> setDiscoverable(bool value) async {
    return adapter.setDiscoverable(value);
  }

  // @override
  // Future<BlueZAdapter> onAdapterClick(BlueZDevice device) async {
  //   await _ensureConnected();
  //   final deviceInfo = client.adapters.firstWhere(
  //     (element) => element.address == device.address,
  //   );
  //   return deviceInfo;
  // }
}
