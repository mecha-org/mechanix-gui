import 'package:dbus/dbus.dart';
import 'package:logger/web.dart';
import 'package:mechanix_settings/src/features/display/models/types.dart';

class DBusDisplayService {
  static const String _busName = 'org.mechanix.MxConf';
  static const String _objectPath = '/org/mechanix/MxConf';
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
    final client = DBusClient.session();
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
        'GetSetting',
        [DBusString(brightnessKey)],
      );

      final displayTimeout = await object.callMethod(
        _busName,
        'GetSetting',
        [DBusString(displayTimeoutKey)],
      );

      final autoBrightness = await object.callMethod(
        _busName,
        'GetSetting',
        [DBusString(autoBrightnessKey)],
      );

      final lockScreenTimeout = await object.callMethod(
        _busName,
        'GetSetting',
        [DBusString(lockScreenTimeoutKey)],
      );

      if (brightness.returnValues.isNotEmpty) {
        final dict = brightness.returnValues.first;
        if (dict is DBusDict) {
          final value = dict.children[DBusString(brightnessKey)];
          if (value is DBusString) {
            logger.i('default brightness value ${value.value}');
            brightnessValue = int.tryParse(value.value);
            // return int.tryParse(value.value);
          }
        }
      }

      if (displayTimeout.returnValues.isNotEmpty) {
        final dict = displayTimeout.returnValues.first;
        if (dict is DBusDict) {
          final value = dict.children[DBusString(displayTimeoutKey)];
          if (value is DBusString) {
            logger.i('default display timeout value ${value.value}');
            displayTimeoutValue = int.tryParse(value.value);
          }
        }
      }

      if (autoBrightness.returnValues.isNotEmpty) {
        final dict = autoBrightness.returnValues.first;
        if (dict is DBusDict) {
          final value = dict.children[DBusString(autoBrightnessKey)];
          if (value is DBusString) {
            logger.i('default autoBrightness value ${value.value}');
            autoBrightnessValue = bool.parse(value.value);
          }
        }
      }

      if (lockScreenTimeout.returnValues.isNotEmpty) {
        final dict = lockScreenTimeout.returnValues.first;
        if (dict is DBusDict) {
          final value = dict.children[DBusString(lockScreenTimeoutKey)];
          if (value is DBusString) {
            logger.i('default lockScreenTimeout value ${value.value}');
            lockScreenTimeoutValue = int.tryParse(value.value);
            // return int.tryParse(value.value);
          }
        }
      }

      final DBusDefaultSettings defaultSetting = DBusDefaultSettings(
        brightness: brightnessValue ?? 40,
        autoBrightness: autoBrightnessValue,
        displayTimeout: displayTimeoutValue ?? 10,
        lockScreenTimeout: lockScreenTimeoutValue ?? 30,
      );
      return defaultSetting;
    } catch (e) {
      logger.e('Error calling get_settings: $e');
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

  Future<void> setBrightness(int brightness) async {
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
          DBusStruct(
              [DBusString(brightnessKey), DBusString(brightness.toString())])
        ],
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
}
