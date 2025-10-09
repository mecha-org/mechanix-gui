import 'dart:async';
import 'dart:io';
import 'package:flutter/widgets.dart';
import 'package:path_provider/path_provider.dart';

enum SortBy { name, date, type, size }

class FileManagerController {
  final ValueNotifier<String> _path = ValueNotifier<String>('');
  final ValueNotifier<SortBy> _short = ValueNotifier<SortBy>(SortBy.name);

  // Pagination state
  int _currentPage = 1;
  final int _pageSize = 20;
  bool _hasMorePages = true;

  Stream<FileSystemEntity>? _entityStream;
  StreamIterator<FileSystemEntity>? _iterator;
  List<FileSystemEntity> _currentEntities = [];
  final ValueNotifier<List<FileSystemEntity>> paginatedEntities =
      ValueNotifier<List<FileSystemEntity>>([]);

  _updatePath(String path) {
    _path.value = path;
    titleNotifier.value = path.split('/').last;
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
  ValueNotifier<SortBy> get getSortedByNotifier => _short;

  /// The sorting type that is currently in use is returned.
  SortBy get getSortedBy => _short.value;

  /// [setSortedBy] is used to set the sorting type.
  ///
  /// `SortBy{ name, type, date, size }`
  /// ie: `controller.sortBy(SortBy.date)`
  void sortBy(SortBy sortType) => _short.value = sortType;

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

  /// Open directory by providing [Directory].
  // void openDirectory(FileSystemEntity entity) {
  //   if (entity is Directory) {
  //     _updatePath(entity.path);
  //   } else {
  //     throw ("FileSystemEntity entity is File. Please provide a Directory(folder) to be opened not File");
  //   }
  // }
  Future<void> openDirectory(FileSystemEntity entity) async {
    if (entity is Directory) {
      _updatePath(entity.path);

      _currentPage = 1;
      _hasMorePages = true;
      _currentEntities.clear();

      _entityStream = entity.list(recursive: false);
      _iterator = StreamIterator(_entityStream!);

      await loadNextChunk(); // load first page
    } else {
      throw ("Please provide a Directory (not a File)");
    }
  }

  Future<void> loadNextChunk() async {
    if (!_hasMorePages || _iterator == null) return;

    final List<FileSystemEntity> chunk = [];

    for (int i = 0; i < _pageSize; i++) {
      final hasNext = await _iterator!.moveNext();
      if (!hasNext) {
        _hasMorePages = false;
        break;
      }
      chunk.add(_iterator!.current);
    }

    _currentEntities.addAll(_sortEntities(chunk));
    paginatedEntities.value = List<FileSystemEntity>.from(_currentEntities);
    _currentPage++;
  }

  List<FileSystemEntity> _sortEntities(List<FileSystemEntity> list) {
    switch (_short.value) {
      case SortBy.date:
        list.sort(
            (a, b) => b.statSync().modified.compareTo(a.statSync().modified));
        break;
      case SortBy.size:
        list.sort((a, b) => b.statSync().size.compareTo(a.statSync().size));
        break;
      case SortBy.type:
        list.sort((a, b) =>
            a.runtimeType.toString().compareTo(b.runtimeType.toString()));
        break;
      case SortBy.name:
      default:
        list.sort(
            (a, b) => a.path.toLowerCase().compareTo(b.path.toLowerCase()));
        break;
    }
    return list;
  }

  /// Dispose FileManagerController
  void dispose() {
    _path.dispose();
    titleNotifier.dispose();
    _short.dispose();
  }
}
