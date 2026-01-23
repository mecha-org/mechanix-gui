import 'dart:async';

import 'package:dbus/dbus.dart';
import 'package:flutter/material.dart';
import 'package:widgets/constants.dart';
import 'package:widgets/mechanix.dart';

class ThemeSettingsService {
  static const String _busName = 'org.mechanix.MxConf';
  static const String _busPath = '/org/mechanix/MxConf';
  static const String _busInterface = 'org.mechanix.MxConf';
  static const String _themeKey =
      'org.mechanix.desktop.settings.active_theme.theme_colors';
  static const String _schemaName = 'org.mechanix.desktop';
  static const String _schemaKey = 'settings.active_theme.theme_colors';
  static const String _wallpaperKey =
      'org.mechanix.desktop.settings.lockscreen.wallpaper';

  static const String _fixedWallpaperPath =
      '/usr/share/backgrounds/lock-screen';

  final DBusClient _bus;
  StreamSubscription<DBusSignal>? _signalSubscription;

  ThemeSettingsService(this._bus);

  Future<String?> getWallpaper() async {
    try {
      final remoteObj = DBusRemoteObject(
        _bus,
        name: _busName,
        path: DBusObjectPath(_busPath),
      );

      final response = await remoteObj.callMethod(
        _busInterface,
        'GetSetting',
        [const DBusString(_wallpaperKey)],
      );

      if (response.returnValues.isNotEmpty &&
          response.returnValues[0] is DBusDict) {
        final dict = response.returnValues[0] as DBusDict;
        final dBusValue = dict.children[const DBusString(_wallpaperKey)];

        if (dBusValue is DBusString) {
          return dBusValue.value.replaceAll(_fixedWallpaperPath, '');
        }
      }
      return null;
    } catch (e) {
      print("Error getting up wallpaper $e");
      return null;
    }
  }

  void listenForThemeChanges(Function(Map<String, String>) onThemeChanged) {
    final remoteObj = DBusRemoteObject(
      _bus,
      name: _busName,
      path: DBusObjectPath(_busPath),
    );

    _signalSubscription = DBusRemoteObjectSignalStream(
      object: remoteObj,
      interface: _busInterface,
      name: 'SchemaKeyChanged',
    ).listen((signal) {
      if (signal.values.length >= 3) {
        final schema = (signal.values[0] as DBusString).value;
        final key = (signal.values[1] as DBusString).value;
        final rawValue = (signal.values[2] as DBusString).value;

        if (schema == _schemaName && key == _schemaKey) {
          final colors = _parseThemeColors(rawValue);
          if (colors != null) {
            onThemeChanged(colors);
          }
        }
      }
    });
  }

  Future<Map<String, String>?> fetchCurrentTheme() async {
    try {
      final remoteObj = DBusRemoteObject(
        _bus,
        name: _busName,
        path: DBusObjectPath(_busPath),
      );

      final response = await remoteObj.callMethod(
        _busInterface,
        'GetSetting',
        [const DBusString(_themeKey)],
      );

      if (response.returnValues.isNotEmpty &&
          response.returnValues[0] is DBusDict) {
        final dict = response.returnValues[0] as DBusDict;
        final dbusValue = dict.children[const DBusString(_themeKey)];

        if (dbusValue is DBusString) {
          return _parseThemeColors(dbusValue.value);
        }
      }
    } catch (e) {
      print('Error fetching theme from DBus: $e');
    }
    return null;
  }

  Future<Map<String, String>?> setCurrentTheme(MechanixVariant variant) async {
    try {
      final remoteObj = DBusRemoteObject(
        _bus,
        name: _busName,
        path: DBusObjectPath(_busPath),
      );

      final currentTheme = await fetchCurrentTheme();

      final String accentString = variant.color.toOklchString();

      final String backgroundString = currentTheme?["background"] ?? '';

      final String foregroundString = currentTheme?["foreground"] ?? '';

      final String body =
          '{type = "object", default = { accent = "$accentString", background = "$backgroundString", foreground = "$foregroundString" }, description = "Theme color palette"}';

      print("SET THEME ========>>>> $body");

      await remoteObj.callMethod(
        _busInterface,
        'SetSetting',
        [
          DBusStruct([const DBusString(_themeKey), DBusString(body)])
        ],
      );
    } catch (e) {
      print('Error updating theme from app: $e');
    }
    return null;
  }

  Future<void> setWallpaper(String filename) async {
    try {
      final remoteObj = DBusRemoteObject(
        _bus,
        name: _busName,
        path: DBusObjectPath(_busPath),
      );

      await remoteObj.callMethod(
        _busInterface,
        'SetSetting',
        [
          DBusStruct([
            const DBusString(_wallpaperKey),
            DBusString(_fixedWallpaperPath + filename)
          ])
        ],
      );
    } catch (e) {
      print("Error setting up wallpaper $e");
    }
  }

  /// Parse theme colors from DBus response string
  Map<String, String>? _parseThemeColors(String description) {
    final accentMatch =
        RegExp(r'accent\s*=\s*"([^"]+)"').firstMatch(description);
    final backgroundMatch =
        RegExp(r'background\s*=\s*"([^"]+)"').firstMatch(description);
    final foregroundMatch =
        RegExp(r'foreground\s*=\s*"([^"]+)"').firstMatch(description);

    print("accentMatch - ${accentMatch}");
    print("backgroundMatch - ${backgroundMatch}");
    print("foregroundMatch - ${foregroundMatch}");

    if (accentMatch == null) {
      print('Failed to parse theme colors from: $description');
      return null;
    }

    return {
      'accent': accentMatch.group(1)!,
      // 'background': backgroundMatch.group(1)!,
      // 'foreground': foregroundMatch.group(1)!,
    };
  }

  /// Convert color map to MechanixThemeData
  MechanixThemeData colorsToThemeData(Map<String, String> colors) {
    final accent = colors['accent']?.toOKLCHStringToColor() ?? Colors.amber;
    final background =
        colors['background']?.toOKLCHStringToColor() ?? defaultBackgroundColor;
    final foreground =
        colors['foreground']?.toOKLCHStringToColor() ?? defaultForegroundColor;

    print("accent - $accent");
    print("background - $background");
    print("foreground - $foreground");

    return MechanixThemeData(
      mechanixVariant: MechanixVariant.custom(accent),
      mechanixBackgroundVariant: MechanixVariant.custom(defaultBackgroundColor),
      mechanixForegroundVariant: MechanixVariant.custom(defaultForegroundColor),
    );
  }

  /// Clean up resources
  void dispose() {
    _signalSubscription?.cancel();
  }
}
