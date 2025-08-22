import 'package:dbus/dbus.dart';
import 'package:logger/web.dart';

class DbusService {
  static final DbusService _instance = DbusService._internal();
  factory DbusService() => _instance;

  DbusService._internal();

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
}
