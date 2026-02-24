import 'dart:async';

import 'package:bluez/bluez.dart';
import 'package:mechanix_settings/src/features/bluetooth/models/bluetooth_device_classifier.dart';
import 'package:mechanix_settings/src/features/bluetooth/models/types.dart';

import 'bluetooth_repository.dart';

class BluetoothRepositoryImpl implements BluetoothRepository {
  late BlueZClient _client;
  late BlueZAdapter _adapter;
  StreamSubscription<List<String>>? _adapterPropsSub;

  bool _connected = false;

  final _adapterPowerController = StreamController<bool>.broadcast();
  final _adapterDiscoverableController = StreamController<bool>.broadcast();

  @override
  Stream<bool> get bluezAdapterPowerStream => _adapterPowerController.stream;

  @override
  Stream<bool> get bluezAdapterDiscoverableStream =>
      _adapterDiscoverableController.stream;

  @override
  Future<void> init() async {
    await _ensureConnected();
  }

  Future<void> _ensureConnected() async {
    try {
      if (_connected) return;

      _client = BlueZClient();

      if (!_connected) {
        await _client.connect();
        _connected = true;
      }

      if (_client.adapters.isEmpty) {
        return;
      }

      _adapter = _client.adapters.first;

      _adapterPropsSub ??= _adapter.propertiesChanged.listen((props) {
        if (props.contains('Powered')) {
          _adapterPowerController.add(_adapter.powered);
        }
        if (props.contains('Discoverable')) {
          _adapterDiscoverableController.add(_adapter.discoverable);
        }
      });
    } catch (e) {
      print("EnsureConnected error: $e");
    }
  }

  @override
  Future<BlueZAdapter> getBluezAdapter() async {
    await _ensureConnected();
    return _adapter;
  }

  @override
  Future<bool> isBluetoothEnabled() async {
    final adapter = await getBluezAdapter();
    return adapter.powered;
  }

  @override
  Future<bool> setPower(bool enable) async {
    try {
      final adapter = await getBluezAdapter();

      await adapter.setPowered(enable);
      return enable;
    } catch (e) {
      print("Error toggling bluetooth impl - $e");
      return false;
    }
  }

  @override
  Future<String> setAdapterAlias(String alias) async {
    try {
      final adapter = await getBluezAdapter();

      await adapter.setAlias(alias);
      return alias;
    } catch (e) {
      print("setAdapterAlias error: $e");
      return '';
    }
  }

  @override
  Future<void> startDiscovery() async {
    try {
      final adapter = await getBluezAdapter();

      if (!adapter.powered) await adapter.setPowered(true);

      if (!adapter.discovering) await adapter.startDiscovery();
    } catch (e) {
      print("startDiscovery error: $e");
    }
  }

  @override
  Future<void> stopDiscovery() async {
    try {
      final adapter = await getBluezAdapter();

      if (adapter.discovering) await adapter.stopDiscovery();
    } catch (e) {
      print("stopDiscovery error: $e");
    }
  }

  @override
  Future<List<BluetoothDeviceDetails>> getDevices() async {
    try {
      await _ensureConnected();

      final List<BluetoothDeviceDetails> devices = [];

      for (final device in _client.devices) {
        final deviceType = BluetoothDeviceClassifier.classify(
          deviceClass: device.deviceClass,
          uuids: device.uuids,
        );

        devices.add(
          BluetoothDeviceDetails(device: device, deviceType: deviceType),
        );
      }

      return devices;
    } catch (e) {
      return [];
    }
  }

  BlueZDevice? _findDevice(String address) {
    try {
      return _client.devices.firstWhere((d) => d.address == address);
    } catch (_) {
      return null;
    }
  }

  @override
  Future<void> pair(String address) async {
    try {
      final device = _findDevice(address);
      if (device == null) return;

      await device.pair();
    } catch (e) {
      print("pair error: $e");
    }
  }

  @override
  Future<bool> connect(String address) async {
    try {
      final device = _findDevice(address);
      if (device == null) return false;

      await device.connect();
      return true;
    } catch (e) {
      return false;
    }
  }

  @override
  Future<void> disconnect(String address) async {
    try {
      final device = _findDevice(address);
      if (device == null) return;

      await device.disconnect();
    } catch (e) {
      print("disconnect error: $e");
    }
  }

  @override
  Future<void> remove(String address) async {
    try {
      final device = _findDevice(address);
      final adapter = await getBluezAdapter();

      if (device == null) return;

      await adapter.removeDevice(device);
    } catch (e) {
      print("remove error: $e");
    }
  }

  @override
  Future<Stream<BlueZDevice>> onDeviceAdded() async {
    await _ensureConnected();
    return _client.deviceAdded;
  }

  @override
  Future<Stream<BlueZDevice>> onDeviceRemoved() async {
    await _ensureConnected();
    return _client.deviceRemoved;
  }

  @override
  Future<void> setDiscoverable(bool value) async {
    try {
      final adapter = await getBluezAdapter();

      if (!adapter.powered) {
        await adapter.setPowered(true);
      }

      await adapter.setDiscoverable(value);
    } catch (e) {
      print("setDiscoverable error: $e");
    }
  }

  @override
  Future<void> close() async {
    await _adapterPropsSub?.cancel();
    await _adapterPowerController.close();
    await _adapterDiscoverableController.close();
    await _client.close();
  }
}
