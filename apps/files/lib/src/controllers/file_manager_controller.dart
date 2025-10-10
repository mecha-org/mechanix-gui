import 'dart:async';
import 'dart:io';
import 'package:flutter/widgets.dart';
import 'package:logger/web.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:path/path.dart' as p;
import 'package:path_provider/path_provider.dart';

enum SortBy { name, date, type, size }

class FileManagerController {
  final logger = Logger();
  final ValueNotifier<String> _path = ValueNotifier<String>('');
  final ValueNotifier<SortBy> _sort = ValueNotifier<SortBy>(SortBy.name);

  int _currentPage = 1;

  final ValueNotifier<List<FileSystemEntity>> paginatedEntities =
      ValueNotifier<List<FileSystemEntity>>([]);

  bool _isLoadingChunk = false;
  bool _hasMorePages = true;

  final List<FileSystemEntity> _currentEntities = [];

  bool _sizeAscending = true; // new flag for size order
  bool get isSizeAscending => _sizeAscending;

  SortBy get sortedBy => _sort.value;

  void _updatePath(String path) {
    _path.value = path;
  }

  /// ValueNotifier of the current directory's basename
  ///
  /// ie:
  /// ```dart
  /// ValueListenableBuilder<String>(
  ///    valueListenable: controller.titleNotifier,
  ///    builder: (context, title, _) {
  ///     return Text(title);
  ///   },
  /// ),
  /// ```
  final ValueNotifier<String> titleNotifier = ValueNotifier<String>('');

  /// Get ValueNotifier of path
  ValueNotifier<String> get getPathNotifier => _path;

  /// Get ValueNotifier of SortedBy
  ValueNotifier<SortBy> get getSortedByNotifier => _sort;

  /// The sorting type that is currently in use is returned.
  SortBy get getSortedBy => _sort.value;

  /// [setSortedBy] is used to set the sorting type.
  ///
  /// `SortBy{ name, type, date, size }`
  /// ie: `controller.sortBy(SortBy.date)`
  // void sortBy(SortBy sortType) => _sort.value = sortType;

  /// Get current Directory.
  Directory get getCurrentDirectory => Directory(_path.value);

  /// Get current path, similar to [getCurrentDirectory].
  String get getCurrentPath => _path.value;

  /// Set current directory path by providing string of path, similar to [openDirectory].
  set setCurrentPath(String path) {
    _updatePath(path);
  }

  /// return true if current directory is the root. false, if the current directory not on root of the stogare.
  Future<bool> isRootDirectory() async {
    final List<Directory> storageList = (await getStorageList());
    return (storageList
        .where((element) => element.path == Directory(_path.value).path)
        .isNotEmpty);
  }

  /// Get list of available storage in the device
  /// returns an empty list if there is no storage
  static Future<List<Directory>> getStorageList() async {
    if (Platform.isLinux) {
      final Directory dir = await getApplicationDocumentsDirectory();

      // Gives the home directory.
      final Directory home = dir.parent.parent;

      // you may provide root directory.
      // final Directory root = dir.parent.parent.parent;
      return [home];
    }
    return [];
  }

  // /// Jumps to the parent directory of currently opened directory if the parent is accessible.
  Future<void> goToParentDirectory() async {
    if (!(await isRootDirectory()))
      openDirectory(Directory(_path.value).parent);
  }

  /// Open a directory and initialize pagination
  Future<void> openDirectory(FileSystemEntity entity) async {
    if (entity is! Directory) {
      throw ("Please provide a Directory (not a File)");
    }

    _updatePath(entity.path);

    // Reset state
    _currentEntities.clear();
    _hasMorePages = true;
    _isLoadingChunk = false;
    _currentPage = 1;

    // Load first chunk
    await loadNextChunk();

    // If directory is empty, still update the paginatedEntities to empty list
    if (_currentEntities.isEmpty) {
      paginatedEntities.value = [];
    }
  }

  /// Load next page of files and emit via StreamController
  Future<void> loadNextChunk() async {
    if (_isLoadingChunk || !_hasMorePages) return;

    _isLoadingChunk = true;

    try {
      final Directory dir = Directory(_path.value);
      final int start = (_currentPage - 1) * pageSize;

      // Get next page of items
      final List<FileSystemEntity> chunk = await dir
          .list(recursive: false, followLinks: false)
          .skip(start)
          .take(pageSize)
          .toList();
      if (chunk.isEmpty) {
        _hasMorePages = false;
      } else {
        // Optionally sort chunk
        _sortEntities(chunk);
        _currentEntities.addAll(chunk);
        paginatedEntities.value = List<FileSystemEntity>.from(_currentEntities);
      }
    } catch (e, st) {
      logger.e('Error loading chunk: $e\n$st');
    } finally {
      _isLoadingChunk = false;
      _currentPage++;
    }
  }

  void sortBy(SortBy sortBy, {bool? sizeAscending}) {
    if (sortBy == SortBy.size) {
      // Flip direction if tapping size again
      if (sizeAscending == null && _sort.value == SortBy.size) {
        _sizeAscending = !_sizeAscending;
      } else if (sizeAscending != null) {
        _sizeAscending = sizeAscending;
      }
    }

    _sort.value = sortBy;
    debugPrint('Sorting by: $sortBy, ascending: $_sizeAscending');
  }

  List<FileSystemEntity> _sortEntities(List<FileSystemEntity> list) {
    final Map<String, int> sizeMap = {};

    list.sort((a, b) {
      final aName = p.basename(a.path).toLowerCase();
      final bName = p.basename(b.path).toLowerCase();

      switch (_sort.value) {
        case SortBy.name:
          if (a is Directory && b is! Directory) return -1;
          if (b is Directory && a is! Directory) return 1;
          return aName.compareTo(bName);

        case SortBy.type:
          if (a is Directory && b is! Directory) return -1;
          if (b is Directory && a is! Directory) return 1;

          final aType =
              a is Directory ? 'dir' : p.extension(a.path).toLowerCase();
          final bType =
              b is Directory ? 'dir' : p.extension(b.path).toLowerCase();
          final typeCompare = aType.compareTo(bType);
          if (typeCompare != 0) return typeCompare;

          return aName.compareTo(bName);

        case SortBy.size:
          // Directories grouped first
          if (a is Directory && b is! Directory) return -1;
          if (b is Directory && a is! Directory) return 1;

          if (a is File && b is File) {
            sizeMap[a.path] ??= a.lengthSync();
            sizeMap[b.path] ??= b.lengthSync();
            return _sizeAscending
                ? sizeMap[a.path]!.compareTo(sizeMap[b.path]!)
                : sizeMap[b.path]!.compareTo(sizeMap[a.path]!);
          }

          return aName.compareTo(bName);

        case SortBy.date:
          final aTime = a.statSync().modified;
          final bTime = b.statSync().modified;
          return bTime.compareTo(aTime);

        default:
          return 0;
      }
    });

    return list;
  }

  /// Reloads the currently open directory (same as reopening it)
  Future<void> reload() async {
    final currentPath = getPathNotifier.value;

    if (currentPath.isEmpty) return;

    final dir = Directory(currentPath);
    if (!dir.existsSync()) return;

    await openDirectory(dir);
  }

  /// Dispose FileManagerController
  void dispose() {
    _path.dispose();
    paginatedEntities.dispose();
    titleNotifier.dispose();
    _sort.dispose();
  }
}
