import 'package:file/src/interface/file_system_entity.dart';
import 'package:mechanix_files/src/controllers/file_manager_controller.dart';
import 'package:mechanix_files/src/models/recent_files.dart';

abstract class RecentFilesRepository {
  Future<List<RecentFile>> getRecentFiles();

  Future<void> addRecentFile(String path);

  Future<void> clear();

  Future<void> removeRecentFile(List<String> entitiesPath);

  Future getSortedFiles({
    required List<FileSystemEntity> files,
    required SortBy sortBy,
    required bool ascending,
  });
}
