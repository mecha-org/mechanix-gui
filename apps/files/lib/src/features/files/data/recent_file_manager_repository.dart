import 'package:hive/hive.dart';
import 'package:mechanix_files/app_config.dart';

class RecentFilesManager {
  static const String _boxName = 'recent_files';
  static const String _key = 'items';

  Box<List> get _box => Hive.box<List>(_boxName);

  Future<void> ensureHiveConnected() async {
    if (!Hive.isBoxOpen(_boxName)) {
      await Hive.openBox<List>(_boxName);
    }
  }

  Future<List<String>> getRecentFiles() async {
    await ensureHiveConnected();
    return (_box.get(_key) ?? []).cast<String>();
  }

  Future<void> addRecentFile(String filePath) async {
    await ensureHiveConnected();

    final List<String> recentFiles = (_box.get(_key) ?? []).cast<String>();

    recentFiles.remove(filePath);
    recentFiles.insert(0, filePath);

    final limit = AppConfig().recentFilesCount;
    if (recentFiles.length > limit) {
      recentFiles.removeRange(limit, recentFiles.length);
    }

    await _box.put(_key, recentFiles);
  }

  Future<void> setRecentFiles(List<String> paths) async {
    await ensureHiveConnected();
    await _box.put(_key, paths);
  }

  Future<void> clear() async {
    await ensureHiveConnected();
    await _box.delete(_key);
  }
}
