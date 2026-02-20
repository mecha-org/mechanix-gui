import 'package:mechanix_files/src/models/recent_files.dart';

abstract class RecentFilesRepository {
  Future<List<RecentFile>> getRecentFiles();

  Future<void> addRecentFile(String path);

  Future<void> clear();

  Future<void> removeRecentFile(List<String> entitiesPath);
}
