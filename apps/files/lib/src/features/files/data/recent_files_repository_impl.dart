import 'package:hive/hive.dart';
import 'package:mechanix_files/app_config.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/models/recent_files.dart';

import 'recent_files_repository.dart';

class RecentFilesRepositoryImpl extends RecentFilesRepository {
  Future<void> ensureRecentFilesConnected() async {
    if (!Hive.isBoxOpen(TableName.recentFilesTable)) {
      await Hive.openBox<RecentFile>(TableName.recentFilesTable);
    }
  }

  Box<RecentFile> _box() => Hive.box<RecentFile>(TableName.recentFilesTable);

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

    final limit = AppConfig().recentFilesCount;

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
}
