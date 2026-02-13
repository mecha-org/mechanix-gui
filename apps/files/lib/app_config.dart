import 'package:dbus/dbus.dart';

class AppConfig {
  static final AppConfig _instance = AppConfig._internal();
  late Map<String, dynamic> _config;

  factory AppConfig() => _instance;

  AppConfig._internal();

  // Call this with the parsed config result from mxconf
  void loadFromMap(Map<String, dynamic> configMap) {
    _config = {};

    for (final entry in configMap.entries) {
      final value = entry.value;

      if (value is DBusMethodSuccessResponse &&
          value.returnValues.isNotEmpty &&
          value.returnValues.first is DBusDict) {
        final dict = value.returnValues.first as DBusDict;
        for (final e in dict.children.entries) {
          final key = (e.key as DBusString).value;
          final val = (e.value as DBusString).value;
          _config[key] = val;
        }
      }
    }
  }

  String? get(String key) => _config[key];
  String get homeDir =>
      get('org.mechanix.files.files_home_page.home_dir') ?? '/home/mecha';
  String get downloadsDir =>
      get('org.mechanix.files.files_home_page.downloads_dir') ??
      '/home/mecha/Downloads';
  String get documentsDir =>
      get('org.mechanix.files.files_home_page.documents_dir') ??
      '/home/mecha/Documents';
  String get recentDir => '/recent';
  int get recentFilesCount {
    final value = get('org.mechanix.files.general.recent_files_count');
    final parsed = int.tryParse(value ?? '');
    return (parsed == null || parsed == 0) ? 50 : parsed;
  }

  String get pdfiumModulePath => '/usr/lib64/libpdfium.so';
}
