import 'package:dbus/dbus.dart';
import 'package:logger/web.dart';
import 'package:mechanix_settings/src/features/display/models/types.dart';

class DBusDisplayService {
// Object /org/mechanix/services/Display
// Interface: org.mechanix.services.Display
// method: SetBrightness

  // static const String _busName = 'org.mechanix.MxConf';
  static const String _busName = 'org.mechanix.services.Display';
  // static const String _objectPath = '/org/mechanix/MxConf';
  static const String _objectPath = '/org/mechanix/services/Display';
  // static const String _interfaceName = 'org.mechanix.MxConf';
  // static const String _propertyName = 'brightness';
  static const brightnessKey =
      "org.mechanix.settings.brightness.value"; // org.mechanix.desktop.settings.display.brightness

  static const displayTimeoutKey =
      "org.mechanix.desktop.settings.display.timeout.value"; // org.mechanix.desktop.settings.display.timeout

  static const autoBrightnessKey =
      "org.mechanix.desktop.settings.display.enable_auto_brightness.value"; // org.mechanix.desktop.settings.display.enable_auto_brightness

  static const lockScreenTimeoutKey =
      "org.mechanix.desktop.settings.security.lock_timeout.value"; // org.mechanix.desktop.settings.security.lock-timeout

  final logger = Logger();

  Future<DBusDefaultSettings> getDisplaySettings() async {
    final client = DBusClient.system();
    int? brightnessValue = 40;
    bool? autoBrightnessValue = true;
    int? displayTimeoutValue = 10;
    int? lockScreenTimeoutValue = 30;

    try {
      final object = DBusRemoteObject(
        client,
        name: _busName,
        path: DBusObjectPath(_objectPath),
      );

      final brightness = await object.callMethod(
        _busName,
        'GetBrightness',
        [],
      );

      final byteValue = brightness.values[0] as DBusByte;
      brightnessValue = byteValue.value;

      // TODO: Re-visit this code later

      // final displayTimeout = await object.callMethod(
      //   _busName,
      //   'GetSetting',
      //   [DBusString(displayTimeoutKey)],
      // );

      // final autoBrightness = await object.callMethod(
      //   _busName,
      //   'GetSetting',
      //   [DBusString(autoBrightnessKey)],
      // );

      // final lockScreenTimeout = await object.callMethod(
      //   _busName,
      //   'GetSetting',
      //   [DBusString(lockScreenTimeoutKey)],
      // );

      // if (displayTimeout.returnValues.isNotEmpty) {
      //   final dict = displayTimeout.returnValues.first;
      //   if (dict is DBusDict) {
      //     final value = dict.children[DBusString(displayTimeoutKey)];
      //     if (value is DBusString) {
      //       logger.i('default display timeout value ${value.value}');
      //       displayTimeoutValue = int.tryParse(value.value);
      //     }
      //   }
      // }

      // if (autoBrightness.returnValues.isNotEmpty) {
      //   final dict = autoBrightness.returnValues.first;
      //   if (dict is DBusDict) {
      //     final value = dict.children[DBusString(autoBrightnessKey)];
      //     if (value is DBusString) {
      //       logger.i('default autoBrightness value ${value.value}');
      //       autoBrightnessValue = bool.parse(value.value);
      //     }
      //   }
      // }

      // if (lockScreenTimeout.returnValues.isNotEmpty) {
      //   final dict = lockScreenTimeout.returnValues.first;
      //   if (dict is DBusDict) {
      //     final value = dict.children[DBusString(lockScreenTimeoutKey)];
      //     if (value is DBusString) {
      //       logger.i('default lockScreenTimeout value ${value.value}');
      //       lockScreenTimeoutValue = int.tryParse(value.value);
      //       // return int.tryParse(value.value);
      //     }
      //   }
      // }

      final DBusDefaultSettings defaultSetting = DBusDefaultSettings(
        brightness: brightnessValue ?? 40,
        autoBrightness: autoBrightnessValue,
        displayTimeout: displayTimeoutValue ?? 10,
        lockScreenTimeout: lockScreenTimeoutValue ?? 30,
      );
      return defaultSetting;
    } catch (e) {
      print('Error calling get_settings: $e');
      return DBusDefaultSettings(
        brightness: 40,
        autoBrightness: true,
        displayTimeout: 10,
        lockScreenTimeout: 30,
      );
    } finally {
      await client.close();
    }
  }

  Future<void> setBrightness(double brightness) async {
    final client = DBusClient.system();
    try {
      final object = DBusRemoteObject(
        client,
        name: _busName,
        path: DBusObjectPath(_objectPath),
      );

      final int brightnessValue = convertToRange(brightness);

      print('brightnessValue - $brightnessValue');

      var args = [DBusByte(brightnessValue)];

      await object.callMethod(
        'org.mechanix.services.Display',
        'SetBrightness',
        args,
      );

      logger.i('Brightness set to $brightness%');
    } catch (e) {
      logger.e('Error setting brightness: $e');
    } finally {
      await client.close();
    }
  }

  Future<void> setAutoBrightness(bool isAutoBrightness) async {
    final client = DBusClient.session();

    try {
      final object = DBusRemoteObject(
        client,
        name: _busName,
        path: DBusObjectPath(_objectPath),
      );

      final result = await object.callMethod(
        _busName,
        'SetSetting',
        [
          DBusStruct([
            DBusString(autoBrightnessKey),
            DBusString(isAutoBrightness.toString())
          ])
        ],
      );
      if (result.returnValues.isNotEmpty) {}
    } catch (e) {
      logger.e('$e');
    } finally {
      await client.close();
    }
  }

  Future<void> setScreenTimeout(int timeout) async {
    final client = DBusClient.session();
    logger.i('setting display timeout $timeout');
    try {
      final object = DBusRemoteObject(
        client,
        name: _busName,
        path: DBusObjectPath(_objectPath),
      );

      final result = await object.callMethod(
        _busName,
        'SetSetting',
        [
          DBusStruct(
              [DBusString(displayTimeoutKey), DBusString(timeout.toString())])
        ],
      );
      if (result.returnValues.isNotEmpty) {}
    } catch (e) {
      logger.e('$e');
    } finally {
      await client.close();
    }
  }

  Future<void> setLockScreenTimeout(int timeout) async {
    final client = DBusClient.session();

    try {
      final object = DBusRemoteObject(
        client,
        name: _busName,
        path: DBusObjectPath(_objectPath),
      );

      final result = await object.callMethod(
        _busName,
        'SetSetting',
        [
          DBusStruct([
            DBusString(lockScreenTimeoutKey),
            DBusString(timeout.toString())
          ])
        ],
      );
      if (result.returnValues.isNotEmpty) {}
    } catch (e) {
      logger.e('$e');
    } finally {
      await client.close();
    }
  }

  int convertToRange(double input) {
    return (input * 254).round();
  }

  double convertToUnitRange(int input) {
    assert(input >= 0 && input <= 254);
    return input / 254;
  }
}
