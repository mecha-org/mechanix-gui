import 'package:dbus/dbus.dart';
import 'package:logger/web.dart';
import 'package:mechanix_settings/src/features/sound/data/types.dart';

class DBusSoundService {
  static const String _busName = 'org.mechanix.MxConf';
  static const String _objectPath = '/org/mechanix/MxConf';
  // static const String _interfaceName = 'org.mechanix.MxConf';
  // static const String _propertyName = 'brightness';
  static const enableLauncherSoundKey =
      "org.mechanix.desktop.settings.launcher.enable_sounds.value"; // org.mechanix.desktop.settings.launcher.enable_sounds

  static const enableVibrationKey =
      "org.mechanix.desktop.settings.haptics.enable_vibration.value"; // org.mechanix.desktop.settings.haptics.enable_vibration

  static const vibrationLevelKey =
      "org.mechanix.desktop.settings.haptics.vibration_level.value"; // org.mechanix.desktop.settings.haptics.vibration_level

  static const notificationSoundKey =
      "org.mechanix.desktop.notification.sound.value"; // org.mechanix.desktop.notification.sound

  final logger = Logger();

  Future<DBusSoundSettings> getSoundSettings() async {
    final client = DBusClient.session();
    bool? enableSoundsValue = true;
    bool? enableVibrationValue = true;
    String? vibrationLevelValue = "medium";
    String? notificationSoundValue = "space";

    try {
      final object = DBusRemoteObject(
        client,
        name: _busName,
        path: DBusObjectPath(_objectPath),
      );

      final enableSounds = await object.callMethod(
        _busName,
        'GetSetting',
        [DBusString(enableLauncherSoundKey)],
      );

      final enableVibration = await object.callMethod(
        _busName,
        'GetSetting',
        [DBusString(enableVibrationKey)],
      );

      final vibrationLevel = await object.callMethod(
        _busName,
        'GetSetting',
        [DBusString(vibrationLevelKey)],
      );

      final notificationSound = await object.callMethod(
        _busName,
        'GetSetting',
        [DBusString(notificationSoundKey)],
      );

      if (enableSounds.returnValues.isNotEmpty) {
        final dict = enableSounds.returnValues.first;
        if (dict is DBusDict) {
          final value = dict.children[DBusString(enableLauncherSoundKey)];
          if (value is DBusString) {
            logger.i('enable sounds value ${value.value}');
            enableSoundsValue = bool.parse(value.value);
          }
        }
      }

      if (enableVibration.returnValues.isNotEmpty) {
        final dict = enableVibration.returnValues.first;
        if (dict is DBusDict) {
          final value = dict.children[DBusString(enableVibrationKey)];
          if (value is DBusString) {
            logger.i('enable vibration value ${value.value}');
            enableVibrationValue = bool.parse(value.value);
          }
        }
      }

      if (vibrationLevel.returnValues.isNotEmpty) {
        final dict = vibrationLevel.returnValues.first;
        if (dict is DBusDict) {
          final value = dict.children[DBusString(vibrationLevelKey)];
          if (value is DBusString) {
            logger.i('vibration level value ${value.value}');
            vibrationLevelValue = value.value;
          }
        }
      }

      if (notificationSound.returnValues.isNotEmpty) {
        final dict = notificationSound.returnValues.first;
        if (dict is DBusDict) {
          final value = dict.children[DBusString(notificationSoundKey)];
          if (value is DBusString) {
            logger.i('notification sound value ${value.value}');
            notificationSoundValue = value.value;
          }
        }
      }

      final DBusSoundSettings soundSettings = DBusSoundSettings(
        enableSounds: enableSoundsValue,
        enableVibration: enableVibrationValue,
        vibrationLevel: vibrationLevelValue,
        notificationSound: notificationSoundValue,
      );
      return soundSettings;
    } catch (e) {
      logger.e('Error calling get sound settings: $e');
      return DBusSoundSettings(
        enableSounds: true,
        enableVibration: true,
        vibrationLevel: "medium",
        notificationSound: "default",
      );
    } finally {
      await client.close();
    }
  }

  Future<void> setEnableSounds(bool enableSounds) async {
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
            DBusString(enableLauncherSoundKey),
            DBusString(enableSounds.toString())
          ])
        ],
      );

      logger.i('Enable sounds set to $enableSounds');
    } catch (e) {
      logger.e('Error setting enable sounds: $e');
    } finally {
      await client.close();
    }
  }

  Future<void> setEnableVibration(bool enableVibration) async {
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
            DBusString(enableVibrationKey),
            DBusString(enableVibration.toString())
          ])
        ],
      );

      logger.i('Enable vibration set to $enableVibration');
    } catch (e) {
      logger.e('Error setting enable vibration: $e');
    } finally {
      await client.close();
    }
  }

  Future<void> setVibrationLevel(String vibrationLevel) async {
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
              [DBusString(vibrationLevelKey), DBusString(vibrationLevel)])
        ],
      );

      logger.i('Vibration level set to $vibrationLevel');
    } catch (e) {
      logger.e('Error setting vibration level: $e');
    } finally {
      await client.close();
    }
  }

  Future<void> setNotificationSound(String notificationSound) async {
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
              [DBusString(notificationSoundKey), DBusString(notificationSound)])
        ],
      );

      logger.i('Notification sound set to $notificationSound');
    } catch (e) {
      logger.e('Error setting notification sound: $e');
    } finally {
      await client.close();
    }
  }
}
