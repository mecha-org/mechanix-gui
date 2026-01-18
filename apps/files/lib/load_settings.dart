import 'dart:async';

import 'package:dbus/dbus.dart';
import 'package:logger/web.dart';
import 'package:flutter/material.dart';
import 'package:widgets/constants.dart';
import 'package:widgets/mechanix.dart';

Future<Map<String, dynamic>> connectToMxconf() async {
  final logger = Logger();

  final result = <String, dynamic>{};

  final client = DBusClient.session();

  final object = DBusRemoteObject(
    client,
    name: 'org.mechanix.MxConf',
    path: DBusObjectPath('/org/mechanix/MxConf'),
  );

  try {
    final filesHomePageSettings = await object.callMethod(
      'org.mechanix.MxConf', // Valid interface name
      'GetSetting', // Method name
      [const DBusString('org.mechanix.files.files_home_page.*')], // Parameters
    );
    result['filesHomePage'] = filesHomePageSettings;

    final generalSettings = await object.callMethod(
      'org.mechanix.MxConf',
      'GetSetting',
      [const DBusString('org.mechanix.files.general.*')],
    );
    result['generalSettings'] = generalSettings;
  } catch (e) {
    logger.e('Error connecting to mxconf: $e');
  } finally {
    await client.close();
  }
  return result;
}

class ThemeSettingsService {
  static const String _busName = 'org.mechanix.MxConf';
  static const String _busPath = '/org/mechanix/MxConf';
  static const String _busInterface = 'org.mechanix.MxConf';
  static const String _themeKey = 'org.mechanix.desktop.settings.active_theme.theme_colors';
  static const String _schemaName = 'org.mechanix.desktop';
  static const String _schemaKey = 'settings.active_theme.theme_colors';

  final DBusClient _bus;
  StreamSubscription<DBusSignal>? _signalSubscription;

  ThemeSettingsService(this._bus);

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
    final logger = Logger();
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
      logger.e('Error fetching theme from DBus: $e');
    }
    return null;
  }

  /// Parse theme colors from DBus response string
  Map<String, String>? _parseThemeColors(String description) {
    final logger = Logger();

    final accentMatch = RegExp(r'accent\s*=\s*"([^"]+)"').firstMatch(description);
    final backgroundMatch = RegExp(r'background\s*=\s*"([^"]+)"').firstMatch(description);
    final foregroundMatch = RegExp(r'foreground\s*=\s*"([^"]+)"').firstMatch(description);

    if (accentMatch == null || backgroundMatch == null || foregroundMatch == null) {
      logger.w('Failed to parse theme colors from: $description');
      return null;
    }

    return {
      'accent': accentMatch.group(1)!,
      'background': backgroundMatch.group(1)!,
      'foreground': foregroundMatch.group(1)!,
    };
  }

  /// Convert color map to MechanixThemeData
  MechanixThemeData colorsToThemeData(Map<String, String> colors) {
    final accent = colors['accent']?.toOKLCHStringToColor() ?? Colors.amber;
    final background = colors['background']?.toOKLCHStringToColor() ?? defaultBackgroundColor;
    final foreground = colors['foreground']?.toOKLCHStringToColor() ?? defaultForegroundColor;

    return MechanixThemeData(
      mechanixVariant: MechanixVariant.custom(accent),
      mechanixBackgroundVariant: MechanixVariant.custom(background),
      mechanixForegroundVariant: MechanixVariant.custom(foreground),
    );
  }

  /// Clean up resources
  void dispose() {
    _signalSubscription?.cancel();
  }
}