import 'dart:async';

import 'package:bluez/bluez.dart';
import 'package:logger/web.dart';

import 'bluetooth_repository.dart';

class BluetoothRepositoryImpl implements BluetoothRepository {
  bool connected = false; // for client connection
  final logger = Logger();

  late BlueZClient _client;
  late BlueZAdapter _adapter;

  @override
  Stream<bool> get bluezAdapterPowerStream => _adapterPowerController.stream;
  final _adapterPowerController = StreamController<bool>.broadcast();

  @override
  Stream<bool> get bluezAdapterDiscoverableStream =>
      _adapterDiscoverableController.stream;
  final _adapterDiscoverableController = StreamController<bool>.broadcast();

  @override
  Future<void> init() async {
    try {
      _client = BlueZClient();
      await _client.connect();
      connected = true;
      logger.i('IMPL:init::  Connected to BlueZClient');

      // start listening for adapter changes
      final adapters = _client.adapters;
      if (adapters.isNotEmpty) {
        _adapter = adapters.first;

        _adapter.propertiesChanged.listen((props) {
          if (props.contains('Powered')) {
            logger.i(
                'IMPL:init:: Using adapter: power change ${_adapter.powered}');
            _adapterPowerController.add(_adapter.powered);
          } else if (props.contains('Discoverable')) {
            logger.i(
                'IMPL:init:: Using adapter: discoverable change ${_adapter.discoverable}');

            _adapterDiscoverableController.add(_adapter.discoverable);
          }
        });
      } else {
        logger.w('IMPL:init:: No Bluetooth adapter found!');
      }
    } catch (e) {
      logger.e('IMPL:init:: Failed to connect to BlueZClient: $e');
      rethrow;
    }
  }

  @override
  Future<BlueZAdapter> getBluezAdapter() async {
    return _client.adapters.first;
  }

  @override
  Future<bool> isBluetoothEnabled() async {
    if (connected) {
      final adapter = await getBluezAdapter();
      return adapter.powered;
    } else {
      var client = BlueZClient();
      await client.connect();
      connected = true;
      final adapter = await getBluezAdapter();
      return adapter.powered;
    }
  }

  @override
  Future<bool> setPower(bool enable) async {
    final adapter = await getBluezAdapter();
    await adapter.setPowered(enable);
    return enable;
  }

  @override
  Future<String> setAdapterAlias(String alias) async {
    final adapter = await getBluezAdapter();
    await adapter.setAlias(alias);
    return alias;
  }

  @override
  Future<void> startDiscovery() async {
    final adapter = await getBluezAdapter();
    final checkPowered = adapter.powered;
    if (!checkPowered) await adapter.setPowered(true);

    if (!adapter.discovering) await adapter.startDiscovery();
  }

  @override
  Future<void> stopDiscovery() async {
    final adapter = await getBluezAdapter();
    if (adapter.discovering) await adapter.stopDiscovery();
  }

  @override
  Future<List<BlueZDevice>> getDevices() async {
    try {
      logger.i("IMPL:  getDevices - CHECK CLIENT $_client");
      var devices = _client.devices;
      logger.i("Devices: $devices");
      return devices;
    } catch (e) {
      logger.e("Error getting devices $e");
      return <BlueZDevice>[]; // Return an empty list on error
    }
  }

  @override
  Future<void> pair(String address) async {
    var device = _client.devices.firstWhere((d) => d.address == address);
    // await device.setTrusted(true);
    await device.pair();
    logger.i("Device paired: $address");
  }

  @override
  Future<void> connect(String address) async {
    var device = _client.devices.firstWhere((d) => d.address == address);
    await device.connect();
    logger.i("Device connected: $address");
  }

  @override
  Future<void> disconnect(String address) async {
    var device = _client.devices.firstWhere((d) => d.address == address);
    await device.disconnect();
    logger.i("Device disconnected: $address");
  }

  @override
  Future<void> remove(String address) async {
    var device = _client.devices.firstWhere((d) => d.address == address);
    final adapter = await getBluezAdapter();

    await adapter.removeDevice(device);
    logger.i("Device removed: $address");
  }

  @override
  Future<Stream<BlueZDevice>> onDeviceAdded() async {
    // return client.deviceAdded.map((device) {
    // logger.i("Device added: ${device.name} --- ${device.address}");
    //   return true;
    // });
    return _client.deviceAdded;
  }

  @override
  Future<Stream<BlueZDevice>> onDeviceRemoved() async {
    // return client.deviceRemoved.map((device) {
    // logger.i("Device removed: ${device.name} ${device.icon}");
    //   return true;
    // });
    return _client.deviceRemoved;
  }

  @override
  Future<void> setDiscoverable(bool value) async {
    final adapter = await getBluezAdapter();
    final checkPowered = adapter.powered;
    if (!checkPowered) await adapter.setPowered(true);
    await adapter.setDiscoverable(value);
  }

  // @override
  // Future<BlueZAdapter> onAdapterClick(BlueZDevice device) async {
  //   await _ensureConnected();
  //   final deviceInfo = client.adapters.firstWhere(
  //     (element) => element.address == device.address,
  //   );
  //   return deviceInfo;
  // }

  @override
  Future<void> close() async {
    await _client.close();
  }
}
