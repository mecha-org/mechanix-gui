import 'package:hive/hive.dart';
import 'package:mechanix_files/app_config.dart';

class RecentFilesManager {
  static const String _boxName = 'recent_files';
  static const String _key = 'items';

  Future<Box<List>> _getBox() async {
    if (!Hive.isBoxOpen(_boxName)) {
      await Hive.openBox<List>(_boxName);
    }
    return Hive.box<List>(_boxName);
  }

  Future<List<String>> getRecentFiles() async {
    final box = await _getBox();
    return (box.get(_key) ?? []).cast<String>();
  }

  Future<void> setRecentFiles(List<String> paths) async {
    final box = await _getBox();
    await box.put(_key, paths);
  }

  Future<void> addRecentFile(String filePath) async {
    final box = await _getBox();

    final List<String> recentFiles = (box.get(_key) ?? []).cast<String>();

    recentFiles.remove(filePath);
    recentFiles.insert(0, filePath);

    final limit = AppConfig().recentFilesCount;
    if (recentFiles.length > limit) {
      recentFiles.removeRange(limit, recentFiles.length);
    }

    await box.put(_key, recentFiles);
  }

  Future<void> clear() async {
    final box = await _getBox();
    await box.delete(_key);
  }
}
