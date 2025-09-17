import 'dart:io';

import 'package:dbus/dbus.dart';
import 'package:logger/web.dart';

class DBusService {
  static final DBusService _instance = DBusService._internal();
  factory DBusService() => _instance;

  DBusService._internal();

  final Logger _logger = Logger();

  static const service = 'org.freedesktop.timedate1';
  static const path = '/org/freedesktop/timedate1';
  static const interface = 'org.freedesktop.timedate1';

  DBusClient? _client;
  DBusRemoteObject? _object;

  Future<void> initialize() async {
    if (_client != null && _object != null) return; // Already initialized

    _logger.w("Initializing DBus client");
    try {
      _client = DBusClient.system();
      _object = DBusRemoteObject(
        _client!,
        name: service,
        path: DBusObjectPath(path),
      );
    } catch (e) {
      _logger.e("Error initializing DBus client: $e");
    }
  }

  DBusRemoteObject? get object => _object;
  DBusClient? get client => _client;

  Future<void> dispose() async {
    try {
      await _client?.close();
      _client = null;
      _object = null;
    } catch (e) {
      _logger.e("Error closing DBus client: $e");
    }
  }

  Future<String> getCurrentTimezones() async {
    final client = DBusClient.system();
    try {
      final object = DBusRemoteObject(
        client,
        name: service,
        path: DBusObjectPath(path),
      );

      final result = await object.callMethod(
        'org.freedesktop.DBus.Properties',
        'Get',
        [
          DBusString(interface),
          DBusString('Timezone'),
        ],
      );

      if (result.returnValues.isNotEmpty) {
        final dict = result.returnValues.first;
        if (dict is DBusVariant) {
          final variant = dict.asVariant();
          if (variant is DBusString) {
            return variant.value;
          }
        }
      }

      return (result.values[0] as DBusString).value;
    } on DBusMethodResponseException catch (e) {
      _logger.e('error while getting current timezone $e');
      rethrow;
    } catch (e) {
      _logger.e('error while getting current timezones $e');
      rethrow;
    }
  }

  Future<List<String>> getAvailableTimezones() async {
    final List<String> timeZones = [];
    final client = DBusClient.system();
    try {
      final object = DBusRemoteObject(
        client,
        name: service,
        path: DBusObjectPath(path),
      );

      final result = await object.callMethod(
        interface,
        'ListTimezones',
        [],
      );

      final values = result.returnValues.first;
      final arr = values.asArray();
      for (var element in arr) {
        if (element is DBusString) {
          timeZones.add(element.value);
        }
      }

      return timeZones;
    } catch (e) {
      _logger.e('error while getting timezones $e');
      return timeZones;
    } finally {
      client.close();
    }
  }

  Future<String> getClockFormat() async {
    final client = DBusClient.session();
    try {
      final result = await Process.run(
          'gsettings', ['get', 'org.gnome.desktop.interface', 'clock-format']);

      return result.stdout.toString().trim();
    } catch (e) {
      print('Error getting clock format: $e');
      rethrow;
    } finally {
      client.close();
    }
  }

  void setClockFormat(String format) async {
    final client = DBusClient.session();
    try {
      await Process.run('gsettings',
          ['set', 'org.gnome.desktop.interface', 'clock-format', format]);
    } catch (e) {
      print('Error getting clock format: $e');
      rethrow;
    } finally {
      client.close();
    }
  }
}
