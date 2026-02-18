import 'package:mechanix_files/src/models/app_settings.dart';

abstract class AppSettingsRepository {
  Future<AppSettings> getSettings();

  Future<void> updateSort(String sortMode, bool ascending);

  Future<void> updateShowHidden(bool showHidden);
}
