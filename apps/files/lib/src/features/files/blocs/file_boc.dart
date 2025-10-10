import 'dart:async';

import 'package:file/local.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/web.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/blocs/file_state.dart';
import 'package:mechanix_files/src/features/files/data/file_repository.dart';
import 'package:mechanix_files/src/features/files/data/recent_file_manager_repository.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:path/path.dart' as p;
import 'package:shared_preferences/shared_preferences.dart';

class FilesBloc extends Bloc<FilesEvent, FilesState> {
  final FileRepository fileRepository;
  final logger = Logger();
  final RecentFilesManager recentFilesManager;

  FilesBloc({
    required this.fileRepository,
    required this.recentFilesManager,
  }) : super(const FilesState(
            fileSystemList: [],
            loading: false,
            error: null,
            currentSortBy: '',
            conflictDestinationPath: '')) {
    on<InitializeFiles>(_onInitializeFiles);
    on<LoadFilesAtPath>(_onLoadFilesAtPath);
    on<CreateFolder>(_onCreateFolder);
    on<DeleteEntities>(_onDeleteEntities);
    on<Rename>(_onRename);

    on<Copy>(_onCopy);
    on<StartCopyMode>(_onStartCopyMode);
    on<CancelCopyMode>(_onCancelCopyMode);
    on<ContinueCopyWithConflictResolution>(
        _onContinueCopyWithConflictResolution);

    on<Move>(_onMove);
    on<StartMoveMode>(_onStartMoveMode);
    on<CancelMoveMode>(_onCancelMoveMode);
    on<ContinueMoveWithConflictResolution>(
        _onContinueMoveWithConflictResolution);

    on<SortFiles>(_onSortFiles);
    on<FetchFileDetails>(_onFetchFileDetails);
    on<ToggleHiddenFiles>(_onToggleHiddenFiles);

    on<CompressEntitiesEvent>(_onCompressEntities);
    on<ExtractZipTo>(_onExtractZipTo);
    on<StartExtractMode>(_onStartExtractMode);
    on<CancelExtractMode>(_onCancelExtractMode);

    on<LoadRecentFiles>(_onLoadRecentFiles);
    on<AddToRecentFiles>(_onAddToRecentFiles);

    on<SearchFilesInDirectory>(_onSearchFilesInDirectory);
    on<ClearSearchResults>((event, emit) {
      emit(state.copyWith(fileSystemList: [], loading: false));
    });
  }

  Future<void> _onInitializeFiles(
      InitializeFiles event, Emitter<FilesState> emit) async {
    emit(state.copyWith(loading: true));

    final prefs = await SharedPreferences.getInstance();
    logger.d('sort_mode: ${prefs.getString('sort_mode')}');
    logger.d('show_hidden_files: ${prefs.getBool('show_hidden_files')}');

    final savedSort = prefs.getString('sort_mode') ?? 'name';
    final savedHidden = prefs.getBool('show_hidden_files') ?? false;

    emit(state.copyWith(
      currentSortBy: savedSort,
      showHiddenFiles: savedHidden,
    ));

    //  await _loadAndEmitSortedFiles(emit: emit, path: '/');
  }

  // Future<void> _onLoadFilesAtPath(
  //     LoadFilesAtPath event, Emitter<FilesState> emit) async {
  //   emit(state.copyWith(loading: true));
  //   await _loadAndEmitSortedFiles(emit: emit, path: event.path);
  // }

  Future<void> _onLoadFilesAtPath(
    LoadFilesAtPath event,
    Emitter<FilesState> emit,
  ) async {
    emit(state.copyWith(loading: true));
    await _loadAndEmitSortedFiles(
      emit: emit,
      path: event.path,
      page: event.page,
      pageSize: event.pageSize,
    );
  }

  Future<void> _onCreateFolder(
      CreateFolder event, Emitter<FilesState> emit) async {
    try {
      emit(state.copyWith(loading: true));
      logger.i("Creating folder: ${event.folderName} in ${event.path}");
      await fileRepository.createFolder(event.path, event.folderName);
      emit(state.copyWith(loading: false));

      // await _loadAndEmitSortedFiles(emit: emit, path: event.path);
    } catch (e) {
      emit(state.copyWith(error: e.toString(), loading: false));
    }
  }

  Future<void> _onDeleteEntities(
    DeleteEntities event,
    Emitter<FilesState> emit,
  ) async {
    if (event.entitiesPath.isEmpty) {
      emit(state.copyWith(error: 'No file or folder selected', loading: false));
      return;
    }

    try {
      emit(state.copyWith(loading: true));
      await fileRepository.deleteEntities(event.entitiesPath);
      final parentPath = p.dirname(event.entitiesPath.first);
      await _loadAndEmitSortedFiles(emit: emit, path: parentPath);
    } catch (e) {
      emit(state.copyWith(error: 'Failed to delete: $e', loading: false));
    }
  }

  Future<void> _onRename(Rename event, Emitter<FilesState> emit) async {
    try {
      emit(state.copyWith(loading: true));
      await fileRepository.renameEntity(event.oldPath, event.newName);
      final parentPath = p.dirname(event.oldPath);
      await _loadAndEmitSortedFiles(emit: emit, path: parentPath);
    } catch (e) {
      emit(state.copyWith(error: 'Failed to rename item: $e', loading: false));
    }
  }

  Future<void> _onCopy(Copy event, Emitter<FilesState> emit) async {
    try {
      emit(state.copyWith(loading: true, error: null));

      final conflicts = <String>[];
      final nonConflicts = <String>[];

      for (final sourcePath in event.sourcePaths) {
        final fileName = p.basename(sourcePath);
        final destination = p.join(event.destinationPath, fileName);

        final exists = await fileRepository.entityExists(destination);

        if (exists) {
          conflicts.add(sourcePath);
        } else {
          nonConflicts.add(sourcePath);
        }
      }

      // First: copy non-conflicting files
      if (nonConflicts.isNotEmpty) {
        await fileRepository.copyEntities(
          nonConflicts,
          event.destinationPath,
          strategy: ConflictResolutionStrategy.replace,
        );
      }

      // Then: handle conflicts
      if (conflicts.isNotEmpty) {
        emit(state.copyWith(
            loading: false,
            conflictingPaths: conflicts,
            conflictDestinationPath: event.destinationPath,
            isCopyMode: true));
      } else {
        // No conflicts, all done
        await _loadAndEmitSortedFiles(emit: emit, path: event.destinationPath);
        emit(state.copyWith(loading: false));
      }
    } catch (e) {
      emit(state.copyWith(error: 'Failed to copy: $e', loading: false));
    }
  }

  Future<void> _onContinueCopyWithConflictResolution(
    ContinueCopyWithConflictResolution event,
    Emitter<FilesState> emit,
  ) async {
    try {
      emit(state.copyWith(loading: true, error: null));

      await fileRepository.copyEntities(
        [event.sourcePaths.first], // Only resolve the first conflict
        event.destinationPath,
        strategy: event.strategy,
      );

      final remainingConflicts = [...event.sourcePaths]..removeAt(0);
      logger.i("Remaining conflicts: $remainingConflicts");
      if (remainingConflicts.isNotEmpty) {
        // Emit next conflict to show dialog again
        emit(state.copyWith(
          conflictingPaths: remainingConflicts,
          loading: false,
        ));
      } else {
        logger.i("In elseRemaining conflicts: $remainingConflicts");

        // All conflicts resolved
        await _loadAndEmitSortedFiles(
          emit: emit,
          path: event.destinationPath,
        );
        emit(state.copyWith(
          conflictingPaths: [],
          conflictDestinationPath: '',
          isCopyMode: false,
          loading: false,
        ));
        logger.i("Emitting final state: ${state.toString()}");
      }
    } catch (e) {
      emit(state.copyWith(
        error: 'Failed to copy: $e',
        loading: false,
      ));
    }
  }

  Future<void> _onMove(Move event, Emitter<FilesState> emit) async {
    try {
      emit(state.copyWith(loading: true, error: null));

      final conflicts = <String>[];
      final nonConflicts = <String>[];

      for (final sourcePath in event.sourcePaths) {
        final fileName = p.basename(sourcePath);
        final destination = p.join(event.destinationPath, fileName);

        final exists = await fileRepository.entityExists(destination);

        if (exists) {
          conflicts.add(sourcePath);
        } else {
          nonConflicts.add(sourcePath);
        }
      }

      // Move non-conflicting files immediately
      if (nonConflicts.isNotEmpty) {
        await fileRepository.moveEntities(
          nonConflicts,
          event.destinationPath,
          strategy: ConflictResolutionStrategy.replace,
        );
      }

      if (conflicts.isNotEmpty) {
        emit(state.copyWith(
          loading: false,
          conflictingPaths: conflicts,
          conflictDestinationPath: event.destinationPath,
          isMoveMode: true,
        ));
      } else {
        event.completer?.complete();
        final firstSource = event.sourcePaths.first;
        final sourceParent = p.dirname(firstSource);

        await _loadAndEmitSortedFiles(emit: emit, path: sourceParent);
        emit(state.copyWith(loading: false));
      }
    } catch (e) {
      emit(state.copyWith(error: 'Failed to move: $e', loading: false));
    }
  }

  Future<void> _onContinueMoveWithConflictResolution(
    ContinueMoveWithConflictResolution event,
    Emitter<FilesState> emit,
  ) async {
    try {
      emit(state.copyWith(loading: true, error: null));

      await fileRepository.moveEntities(
        [event.sourcePaths.first], // One at a time
        event.destinationPath,
        strategy: event.strategy,
      );

      final remainingConflicts = [...event.sourcePaths]..removeAt(0);

      if (remainingConflicts.isNotEmpty) {
        emit(state.copyWith(
          conflictingPaths: remainingConflicts,
          loading: false,
        ));
      } else {
        await _loadAndEmitSortedFiles(emit: emit, path: event.destinationPath);
        emit(state.copyWith(
          conflictingPaths: [],
          conflictDestinationPath: '',
          isMoveMode: false,
          loading: false,
        ));
      }
    } catch (e) {
      emit(state.copyWith(
        error: 'Failed to move: $e',
        loading: false,
      ));
    }
  }

  // Future<void> _loadAndEmitSortedFiles({
  //   required Emitter<FilesState> emit,
  //   String path = '/',
  // }) async {
  //   try {
  //     final contents = await fileRepository.getFileSystemList(path: path);
  //     final visible = state.showHiddenFiles
  //         ? contents
  //         : contents.where((e) => !p.basename(e.path).startsWith('.')).toList();

  //     final sorted =
  //         await fileRepository.sortEntities(visible, state.currentSortBy);
  //     emit(state.copyWith(fileSystemList: sorted, loading: false));
  //   } catch (e) {
  //     logger.e("Error loading path $path: $e");
  //     emit(state.copyWith(error: e.toString(), loading: false));
  //   }
  // }

  Future<void> _loadAndEmitSortedFiles({
    required Emitter<FilesState> emit,
    String path = '/',
    int page = page,
    int pageSize = pageSize,
  }) async {
    try {
      emit(state.copyWith(loading: true, error: null));

      // Load page from repository (streamed, paginated)
      final contents = await fileRepository.getPaginatedFileSystemList(
        path: path,
        page: page,
        pageSize: pageSize,
      );

      // Filter hidden files
      final visible = state.showHiddenFiles
          ? contents
          : contents.where((e) => !p.basename(e.path).startsWith('.')).toList();

      // Sort visible files
      final sorted = await fileRepository.sortEntities(
        visible,
        state.currentSortBy,
      );

      // If page == 1 → replace, else append
      final newList = page == 1 ? sorted : [...state.fileSystemList, ...sorted];

      // Determine if we reached the last page
      final hasMore = contents.length == pageSize;
      logger.i("current page: $page");
      logger.i("has more pages: $hasMore");

      emit(state.copyWith(
        fileSystemList: newList,
        loading: false,
        currentPage: page,
        hasMorePages: hasMore,
      ));
    } catch (e, stackTrace) {
      logger.e("Error loading path $path: $e", stackTrace: stackTrace);
      emit(state.copyWith(error: e.toString(), loading: false));
    }
  }

  void _onStartCopyMode(StartCopyMode event, Emitter<FilesState> emit) {
    emit(state.copyWith(
      isCopyMode: true,
      copiedPaths: event.copiedPaths,
      isMoveMode: false,
      movedPaths: [],
    ));
  }

  void _onCancelCopyMode(CancelCopyMode event, Emitter<FilesState> emit) {
    emit(state.copyWith(
      isCopyMode: false,
      copiedPaths: [],
    ));
  }

  void _onStartMoveMode(StartMoveMode event, Emitter<FilesState> emit) {
    emit(state.copyWith(
      isMoveMode: true,
      movedPaths: event.movedPaths,
      isCopyMode: false,
      copiedPaths: [],
    ));
  }

  void _onCancelMoveMode(CancelMoveMode event, Emitter<FilesState> emit) {
    emit(state.copyWith(
      isMoveMode: false,
      movedPaths: [],
    ));
  }

  Future<void> _onSortFiles(SortFiles event, Emitter<FilesState> emit) async {
    final sorted =
        await fileRepository.sortEntities(state.fileSystemList, event.sortBy);

    final prefs = await SharedPreferences
        .getInstance(); // Get shared preferences instance
    await prefs.setString(
        'sort_mode', event.sortBy); // Save sort mode to shared preferences

    emit(state.copyWith(
      fileSystemList: sorted,
      currentSortBy: event.sortBy,
    ));
  }

  Future<void> _onFetchFileDetails(
    FetchFileDetails event,
    Emitter<FilesState> emit,
  ) async {
    emit(state.copyWith(loading: true));

    try {
      final stat = await fileRepository.getFileDetails(event.path);
      logger.i('File stat : $stat');
      emit(state.copyWith(
        fileDetails: stat,
        loading: false,
      ));
    } catch (e) {
      emit(state.copyWith(
        error: e.toString(),
        loading: false,
      ));
    }
  }

  Future<void> _onToggleHiddenFiles(
      ToggleHiddenFiles event, Emitter<FilesState> emit) async {
    final newShowHidden = !state.showHiddenFiles;
    emit(state.copyWith(showHiddenFiles: newShowHidden, loading: true));

    final prefs = await SharedPreferences.getInstance();
    await prefs.setBool('show_hidden_files', newShowHidden);

    await _loadAndEmitSortedFiles(emit: emit, path: event.path);
  }

  Future<void> _onCompressEntities(
      CompressEntitiesEvent event, Emitter<FilesState> emit) async {
    emit(state.copyWith(
      compressionStatus: FileCompressionStatus.inProgress,
      compressionError: null,
      compressedZipPath: null,
    ));

    try {
      await fileRepository.compressEntities(
        event.sourcePaths,
        event.destinationZipPath,
      );

      emit(state.copyWith(
        compressionStatus: FileCompressionStatus.success,
        compressedZipPath: event.destinationZipPath,
      ));

      final parentDir = p.dirname(event.destinationZipPath);
      await _loadAndEmitSortedFiles(emit: emit, path: parentDir);
    } catch (e) {
      emit(state.copyWith(
        compressionStatus: FileCompressionStatus.failure,
        compressionError: e.toString(),
      ));
    }
  }

  Future<void> _onExtractZipTo(
    ExtractZipTo event,
    Emitter<FilesState> emit,
  ) async {
    emit(state.copyWith(loading: true));
    try {
      var targetDir = event.targetPath;
      if (targetDir.isEmpty) {
        targetDir = p.dirname(event.zipFilePath);
      }

      await fileRepository.extractZip(event.zipFilePath, targetDir);

      if (!(event.completer?.isCompleted ?? true)) {
        event.completer?.complete();
      }

      await _loadAndEmitSortedFiles(emit: emit, path: targetDir);
    } catch (e) {
      if (!(event.completer?.isCompleted ?? true)) {
        event.completer?.completeError(e);
      }
      emit(state.copyWith(error: 'Failed to extract ZIP: $e', loading: false));
    }
  }

  void _onStartExtractMode(StartExtractMode event, Emitter<FilesState> emit) {
    emit(state.copyWith(
      isExtractMode: true,
      zipFilePath: event.zipFilePath,
    ));
  }

  void _onCancelExtractMode(CancelExtractMode event, Emitter<FilesState> emit) {
    emit(state.copyWith(
      isExtractMode: false,
      zipFilePath: '',
    ));
  }

  Future<void> _onLoadRecentFiles(
    LoadRecentFiles event,
    Emitter<FilesState> emit,
  ) async {
    try {
      final fileSystem = const LocalFileSystem();
      final recentPaths = await recentFilesManager.getRecentFiles();
      final cleaned = <String>[];

      final visibleFiles = recentPaths
          .where((path) {
            final isVisible =
                state.showHiddenFiles || !p.basename(path).startsWith('.');
            if (!isVisible) return false;

            final file = fileSystem.file(path);
            if (file.existsSync()) {
              cleaned.add(path);
              return true;
            }
            return false;
          })
          .map((path) => fileSystem.file(path))
          .toList();

      await recentFilesManager.setRecentFiles(cleaned);

      emit(state.copyWith(
        loading: false,
        fileSystemList: visibleFiles,
      ));
    } catch (e) {
      emit(state.copyWith(loading: false));
    }
  }

  Future<void> _onAddToRecentFiles(
    AddToRecentFiles event,
    Emitter<FilesState> emit,
  ) async {
    await recentFilesManager.addRecentFile(event.path);
  }

  Future<void> _onSearchFilesInDirectory(
    SearchFilesInDirectory event,
    Emitter<FilesState> emit,
  ) async {
    emit(state.copyWith(loading: true, error: null));

    try {
      final results = await fileRepository.searchFiles(event.path, event.query);
      emit(state.copyWith(
        fileSystemList: results,
        loading: false,
      ));
    } catch (e) {
      emit(state.copyWith(
        error: 'Search failed: $e',
        loading: false,
      ));
    }
  }
}
