import 'dart:async';
import 'dart:developer';

import 'package:dbus/dbus.dart';
import 'package:logger/web.dart';
import 'package:mechanix_settings/src/features/battery/models/battery_info.dart';
import 'package:upower/upower.dart';

import 'battery_repository.dart';

class BatteryRepositoryImpl implements BatteryRepository {
  final logger = Logger();
  bool _connected = false;
  final UPowerClient _client = UPowerClient();

  BatteryRepositoryImpl() {
    _init();
  }

  Future<void> _init() async {
    if (!_connected) {
      await _client.connect();
      _connected = true;
    }
  }

  Future<void> _ensureConnected() async {
    if (!_connected) {
      await _client.connect();
      _connected = true;
    }
  }

  @override
  Future<BatteryInfo> getBatteryInfo() async {
    await _ensureConnected();
    // short delay (or wait for updates)
    await Future.delayed(const Duration(milliseconds: 150));

    String? batteryMode = await getBatteryModeViaDBus();

    final modes = await getAvailableBatteryModes();

    final device = _client.displayDevice;
    double percentage = 0.0;

    if (device.type == UPowerDeviceType.battery) {
      percentage = device.percentage;
      log('Battery percentage: $percentage');
    }

    return BatteryInfo(
      batteryPercentage: percentage,
      status: device.state,
      mode: batteryMode ?? '',
      batteryChargingTime: device.timeToFull,
      batteryRemainingTime: device.timeToEmpty,
      availableBatteryModes: modes,
    );
  }

  @override
  Future<String> setBatteryMode(String mode) async {
    final client = DBusClient.system();
    try {
      final object = DBusRemoteObject(
        client,
        name:
            'net.hadess.PowerProfiles', // service name for power-profiles-daemon
        path: DBusObjectPath('/net/hadess/PowerProfiles'),
      );

      // Set ActiveProfile property to the desired mode
      await object.setProperty(
        'net.hadess.PowerProfiles',
        'ActiveProfile',
        DBusString(mode),
      );

      // Verify change
      final response = await object.getProperty(
        'net.hadess.PowerProfiles',
        'ActiveProfile',
      );

      return (response as DBusString).value;
    } catch (e) {
      return 'Error: $e';
    } finally {
      await client.close();
    }
  }

  Future<String?> getBatteryModeViaDBus() async {
    final client = DBusClient.system();

    try {
      final object = DBusRemoteObject(
        client,
        name: 'org.freedesktop.UPower.PowerProfiles',
        path: DBusObjectPath('/org/freedesktop/UPower/PowerProfiles'),
      );

      dynamic prop = await object.getProperty(
        'org.freedesktop.UPower.PowerProfiles',
        'ActiveProfile',
      );
      if (prop is DBusString) {
        return prop.value;
      }

      return null;
    } catch (e) {
      return null;
    } finally {
      await client.close();
    }
  }

  Future<List<String>> getAvailableBatteryModes() async {
    final client = DBusClient.system();

    try {
      final object = DBusRemoteObject(
        client,
        name: 'org.freedesktop.UPower.PowerProfiles',
        path: DBusObjectPath('/org/freedesktop/UPower/PowerProfiles'),
      );

      final prop = await object.getProperty(
        'org.freedesktop.UPower.PowerProfiles',
        'Profiles',
      );

      List<String> modes = [];

      if (prop is DBusArray) {
        for (var element in prop.children) {
          if (element is DBusDict) {
            final value = element.children[DBusString('Profile')];
            if (value is DBusVariant) {
              final variant = value.asVariant();
              if (variant is DBusString) {
                modes.add(variant.value);
              }
            }
          }
        }
      }

      return modes;
    } catch (e) {
      return [];
    } finally {
      await client.close();
    }
  }

  @override
  Future<Stream<List<String>>> streamBatteryEvents() async {
    await _ensureConnected();
    final device = _client.displayDevice;
    return device.propertiesChanged;
  }
}
