import 'package:mechanix_files/app_config.dart';
import 'package:shared_preferences/shared_preferences.dart';

class RecentFilesManager {
  static const String _recentFilesKey = 'recentFiles';

  Future<List<String>> getRecentFiles() async {
    final prefs = await SharedPreferences.getInstance();
    return prefs.getStringList(_recentFilesKey) ?? [];
  }

  Future<void> addRecentFile(String filePath) async {
    final prefs = await SharedPreferences.getInstance();
    List<String> recentFiles = prefs.getStringList(_recentFilesKey) ?? [];

    recentFiles.remove(filePath);
    recentFiles.insert(0, filePath);

    final limit = AppConfig().recentFilesCount;
    if (recentFiles.length > limit) {
      recentFiles = recentFiles.sublist(0, limit);
    }

    await prefs.setStringList(_recentFilesKey, recentFiles);
  }

  Future<void> clear() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.remove(_recentFilesKey);
  }

  Future<void> setRecentFiles(List<String> paths) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setStringList(_recentFilesKey, paths);
  }
}
