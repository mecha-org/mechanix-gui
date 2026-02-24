import 'package:file/src/interface/file_system_entity.dart';
import 'package:hive/hive.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/controllers/file_manager_controller.dart';
import 'package:mechanix_files/src/models/recent_files.dart';

import 'recent_files_repository.dart';

class RecentFilesRepositoryImpl extends RecentFilesRepository {
  Future<void> ensureRecentFilesConnected() async {
    if (!Hive.isBoxOpen(HiveTables.recentFilesTable)) {
      await Hive.openBox<RecentFile>(HiveTables.recentFilesTable);
    }
  }

  Box<RecentFile> _box() => Hive.box<RecentFile>(HiveTables.recentFilesTable);

  @override
  Future<List<RecentFile>> getRecentFiles() async {
    await ensureRecentFilesConnected();

    return _box().values.toList()
      ..sort((a, b) => b.openedAt.compareTo(a.openedAt));
  }

  @override
  Future<void> addRecentFile(String path) async {
    await ensureRecentFilesConnected();

    dynamic existingKey;

    for (final key in _box().keys) {
      final item = _box().get(key);
      if (item?.path == path) {
        existingKey = key;
        break;
      }
    }

    if (existingKey != null) {
      await _box().delete(existingKey);
    }

    await _box().add(RecentFile(path: path, openedAt: DateTime.now()));

    final limit = AppLimits.recentFilesCount;

    final items =
        _box().values.toList()
          ..sort((a, b) => b.openedAt.compareTo(a.openedAt));

    if (items.length > limit) {
      for (final extra in items.sublist(limit)) {
        final key = _box().keys.firstWhere((k) => _box().get(k) == extra);
        await _box().delete(key);
      }
    }
  }

  @override
  Future<void> clear() async {
    await ensureRecentFilesConnected();
    await _box().clear();
  }

  @override
  Future<void> removeRecentFile(List<String> entitiesPath) async {
    await ensureRecentFilesConnected();

    final box = _box();

    // Convert to Set for fast lookup
    final pathsToRemove = entitiesPath.toSet();

    final keysToDelete = <dynamic>[];

    for (final key in box.keys) {
      final file = box.get(key);

      if (file != null && pathsToRemove.contains(file.path)) {
        keysToDelete.add(key);
      }
    }

    if (keysToDelete.isNotEmpty) {
      await box.deleteAll(keysToDelete);
    }
  }

  @override
  Future<List<FileSystemEntity>> getSortedFiles({
    required List<FileSystemEntity> files,
    required SortBy sortBy,
    required bool ascending,
  }) async {
    final sorted = List<FileSystemEntity>.from(files);

    sorted.sort((a, b) {
      final statA = a.statSync();
      final statB = b.statSync();

      int result;

      switch (sortBy) {
        case SortBy.name:
          result = a.path
              .split('/')
              .last
              .toLowerCase()
              .compareTo(b.path.split('/').last.toLowerCase());
          break;

        case SortBy.size:
          result = statA.size.compareTo(statB.size);
          break;

        case SortBy.accessedTime:
          result = statA.accessed.compareTo(statB.accessed);
          break;

        case SortBy.modTime:
          result = statA.modified.compareTo(statB.modified);
          break;
        case SortBy.type:
          // TODO: Handle this case.
          throw UnimplementedError();
      }

      return ascending ? result : -result;
    });

    return sorted;
  }
}
