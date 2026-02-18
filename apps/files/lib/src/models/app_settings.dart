import 'package:hive/hive.dart';

part 'app_settings.g.dart';

@HiveType(typeId: 0)
class AppSettings extends HiveObject {
  @HiveField(0)
  String sortMode;

  @HiveField(1)
  bool ascending;

  @HiveField(2)
  bool showHiddenFiles;

  AppSettings({
    required this.sortMode,
    required this.ascending,
    required this.showHiddenFiles,
  });

  @override
  String toString() {
    return 'AppSettings(sortMode: $sortMode, ascending: $ascending, showHiddenFiles: $showHiddenFiles)';
  }

  AppSettings copyWith({
    String? sortMode,
    bool? ascending,
    bool? showHiddenFiles,
  }) {
    return AppSettings(
      sortMode: sortMode ?? this.sortMode,
      ascending: ascending ?? this.ascending,
      showHiddenFiles: showHiddenFiles ?? this.showHiddenFiles,
    );
  }
}
