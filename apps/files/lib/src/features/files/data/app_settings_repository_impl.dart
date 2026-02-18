import 'package:hive/hive.dart';
import 'package:logger/web.dart';
import 'package:mechanix_files/src/features/files/data/app_settings_repository.dart';
import 'package:mechanix_files/src/models/app_settings.dart';
import 'package:mechanix_files/src/commons/constants.dart';

class AppSettingsRepositoryImpl extends AppSettingsRepository {
  final logger = Logger();

  Future<void> ensureAppSettingsConnected() async {
    if (!Hive.isBoxOpen(TableName.appSettingsTable)) {
      await Hive.openBox<AppSettings>(TableName.appSettingsTable);
    }
  }

  AppSettings _defaults() => AppSettings(
    sortMode: 'mod_time',
    ascending: false,
    showHiddenFiles: false,
  );

  Box<AppSettings> _box() => Hive.box<AppSettings>(TableName.appSettingsTable);

  @override
  Future<AppSettings> getSettings() async {
    await ensureAppSettingsConnected();

    final settings = _box().get(0, defaultValue: _defaults())!;

    logger.d("Loaded app settings: $settings");
    return settings;
  }

  Future<void> saveSettings(AppSettings settings) async {
    await ensureAppSettingsConnected();

    await _box().put(0, settings);
    logger.d("Saved app settings: $settings");
  }

  @override
  Future<void> updateSort(String sortMode, bool ascending) async {
    final current = await getSettings();

    await saveSettings(
      current.copyWith(sortMode: sortMode, ascending: ascending),
    );
  }

  @override
  Future<void> updateShowHidden(bool value) async {
    final current = await getSettings();

    await saveSettings(current.copyWith(showHiddenFiles: value));
  }

  Future<void> resetToDefaults() async {
    await saveSettings(_defaults());
  }
}
