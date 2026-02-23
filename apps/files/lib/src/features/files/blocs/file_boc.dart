import 'dart:async';

import 'package:file/local.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/web.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/blocs/file_state.dart';
import 'package:mechanix_files/src/features/files/data/app_settings_repository.dart';
import 'package:mechanix_files/src/features/files/data/file_repository.dart';
import 'package:mechanix_files/src/features/files/data/recent_files_repository.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:path/path.dart' as p;

class FilesBloc extends Bloc<FilesEvent, FilesState> {
  final FileRepository fileRepository;
  final logger = Logger();
  final RecentFilesRepository recentFilesRepository;
  final AppSettingsRepository appSettingsRepository;

  FilesBloc({
    required this.fileRepository,
    required this.recentFilesRepository,
    required this.appSettingsRepository,
  }) : super(
         const FilesState(
           fileSystemList: [],
           loading: false,
           error: null,
           currentSortBy: '',
           isAscending: false,
           conflictDestinationPath: '',
         ),
       ) {
    on<InitializeFiles>(_onInitializeFiles);
    on<CreateFolder>(_onCreateFolder);
    on<DeleteEntities>(_onDeleteEntities);
    on<Rename>(_onRename);

    on<Copy>(_onCopy);
    on<StartCopyMode>(_onStartCopyMode);
    on<CancelCopyMode>(_onCancelCopyMode);
    on<ContinueCopyWithConflictResolution>(
      _onContinueCopyWithConflictResolution,
    );

    on<Move>(_onMove);
    on<StartMoveMode>(_onStartMoveMode);
    on<CancelMoveMode>(_onCancelMoveMode);
    on<ContinueMoveWithConflictResolution>(
      _onContinueMoveWithConflictResolution,
    );

    on<SortFiles>(_onSortFiles);
    on<FetchFileDetails>(_onFetchFileDetails);
    on<ToggleHiddenFiles>(_onToggleHiddenFiles);

    on<CompressEntitiesEvent>(_onCompressEntities);
    on<ExtractZipTo>(_onExtractZipTo);
    on<ExtractZipBatchCompleted>(_onExtractZipBatchCompleted);
    on<StartExtractMode>(_onStartExtractMode);
    on<CancelExtractMode>(_onCancelExtractMode);

    on<LoadRecentFiles>(_onLoadRecentFiles);
    on<AddToRecentFiles>(_onAddToRecentFiles);
    on<RemoveRecentEntities>(_onRemoveRecentEntities);
    on<SortRecentFiles>(_onSortRecentFiles);

    on<SearchFilesInDirectory>(_onSearchFilesInDirectory);
    on<ClearSearchResults>((event, emit) {
      emit(state.copyWith(fileSystemList: [], loading: false));
    });
  }

  Future<void> _onInitializeFiles(
    InitializeFiles event,
    Emitter<FilesState> emit,
  ) async {
    emit(state.copyWith(loading: true));

    final settings = await appSettingsRepository.getSettings();

    emit(
      state.copyWith(
        loading: false,
        currentSortBy: settings.sortMode,
        isAscending: settings.ascending,
        showHiddenFiles: settings.showHiddenFiles,
      ),
    );
  }

  Future<void> _onCreateFolder(
    CreateFolder event,
    Emitter<FilesState> emit,
  ) async {
    try {
      emit(state.copyWith(loading: true));

      logger.i("Creating folder: ${event.folderName} in ${event.path}");

      // Create folder
      await fileRepository.createFolder(event.path, event.folderName);

      // Mark this folder as new (important!)
      final newFolderPath = "${event.path}/${event.folderName}";
      event.controller.markNewFolder(newFolderPath);

      // Reload file list AFTER tagging the new folder
      await event.controller.reload();

      emit(state.copyWith(loading: false));
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
      await event.controller.reload();
      emit(state.copyWith(loading: false));
    } catch (e) {
      emit(state.copyWith(error: 'Failed to delete: $e', loading: false));
    }
  }

  Future<void> _onRename(Rename event, Emitter<FilesState> emit) async {
    try {
      emit(state.copyWith(loading: true));
      await fileRepository.renameEntity(event.oldPath, event.newName);
      await event.controller.reload();
      emit(state.copyWith(loading: false));
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
        emit(
          state.copyWith(
            loading: false,
            conflictingPaths: conflicts,
            conflictDestinationPath: event.destinationPath,
            isCopyMode: true,
          ),
        );
      } else {
        // No conflicts, all done
        await event.controller!.reload();
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
        emit(
          state.copyWith(conflictingPaths: remainingConflicts, loading: false),
        );
      } else {
        logger.i("In elseRemaining conflicts: $remainingConflicts");

        // All conflicts resolved
        await event.controller!.reload();

        emit(
          state.copyWith(
            conflictingPaths: [],
            conflictDestinationPath: '',
            isCopyMode: false,
            loading: false,
          ),
        );
        logger.i("Emitting final state: ${state.toString()}");
      }
    } catch (e) {
      emit(state.copyWith(error: 'Failed to copy: $e', loading: false));
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
        emit(
          state.copyWith(
            loading: false,
            conflictingPaths: conflicts,
            conflictDestinationPath: event.destinationPath,
            isMoveMode: true,
          ),
        );
      } else {
        event.completer?.complete();
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
        emit(
          state.copyWith(conflictingPaths: remainingConflicts, loading: false),
        );
      } else {
        emit(
          state.copyWith(
            conflictingPaths: [],
            conflictDestinationPath: '',
            isMoveMode: false,
            loading: false,
          ),
        );
      }
    } catch (e) {
      emit(state.copyWith(error: 'Failed to move: $e', loading: false));
    }
  }

  void _onStartCopyMode(StartCopyMode event, Emitter<FilesState> emit) {
    emit(
      state.copyWith(
        isCopyMode: true,
        copiedPaths: event.copiedPaths,
        isMoveMode: false,
        movedPaths: [],
      ),
    );
  }

  void _onCancelCopyMode(CancelCopyMode event, Emitter<FilesState> emit) {
    emit(state.copyWith(isCopyMode: false, copiedPaths: []));
  }

  void _onStartMoveMode(StartMoveMode event, Emitter<FilesState> emit) {
    emit(
      state.copyWith(
        isMoveMode: true,
        movedPaths: event.movedPaths,
        isCopyMode: false,
        copiedPaths: [],
      ),
    );
  }

  void _onCancelMoveMode(CancelMoveMode event, Emitter<FilesState> emit) {
    emit(state.copyWith(isMoveMode: false, movedPaths: []));
  }

  Future<void> _onSortRecentFiles(
    SortRecentFiles event,
    Emitter<FilesState> emit,
  ) async {
    final sorted = await recentFilesRepository.getSortedFiles(
      files: state.fileSystemList,
      sortBy: sortByFromKey(event.sortBy),
      ascending: event.isAscending,
    );

    await appSettingsRepository.updateSort(event.sortBy, event.isAscending);

    emit(
      state.copyWith(
        fileSystemList: sorted,
        currentSortBy: event.sortBy,
        isAscending: event.isAscending,
      ),
    );
  }

  Future<void> _onSortFiles(SortFiles event, Emitter<FilesState> emit) async {
    logger.d("Sort by : ${event.sortBy}, asc: ${event.isAscending}");

    await appSettingsRepository.updateSort(event.sortBy, event.isAscending);

    emit(
      state.copyWith(
        currentSortBy: event.sortBy,
        isAscending: event.isAscending,
      ),
    );
  }

  Future<void> _onFetchFileDetails(
    FetchFileDetails event,
    Emitter<FilesState> emit,
  ) async {
    emit(state.copyWith(loading: true));

    try {
      final stat = await fileRepository.getFileDetails(event.path);
      logger.i('File stat : $stat');
      emit(state.copyWith(fileDetails: stat, loading: false));
    } catch (e) {
      emit(state.copyWith(error: e.toString(), loading: false));
    }
  }

  Future<void> _onToggleHiddenFiles(
    ToggleHiddenFiles event,
    Emitter<FilesState> emit,
  ) async {
    final newShowHidden = !state.showHiddenFiles;
    emit(state.copyWith(showHiddenFiles: newShowHidden, loading: true));
    await appSettingsRepository.updateShowHidden(newShowHidden);

    emit(state.copyWith(loading: false));
  }

  Future<void> _onCompressEntities(
    CompressEntitiesEvent event,
    Emitter<FilesState> emit,
  ) async {
    emit(
      state.copyWith(
        compressionStatus: FileCompressionStatus.inProgress,
        compressionError: null,
        compressedZipPath: null,
      ),
    );

    try {
      await fileRepository.compressEntities(
        event.sourcePaths,
        event.destinationZipPath,
      );

      emit(
        state.copyWith(
          compressionStatus: FileCompressionStatus.success,
          compressedZipPath: event.destinationZipPath,
        ),
      );
      await event.controller?.reload();
    } catch (e) {
      emit(
        state.copyWith(
          compressionStatus: FileCompressionStatus.failure,
          compressionError: e.toString(),
        ),
      );
    }
  }

  void _onExtractZipTo(ExtractZipTo event, Emitter<FilesState> emit) async {
    try {
      await fileRepository.extractZip(event.zipPath, event.targetPath);
      event.completer.complete('success');
    } catch (e) {
      event.completer.complete('failure');
    }
  }

  void _onExtractZipBatchCompleted(
    ExtractZipBatchCompleted event,
    Emitter<FilesState> emit,
  ) {
    emit(
      state.copyWith(
        extractStatus: FileExtractStatus.completed,
        extractSuccessCount: event.successCount,
        extractFailureCount: event.failureCount,
      ),
    );
  }

  void _onStartExtractMode(StartExtractMode event, Emitter<FilesState> emit) {
    emit(
      state.copyWith(
        isExtractMode: true,
        zipFilePaths: event.zipFilePaths,
        extractStatus: FileExtractStatus.inProgress,
        extractError: null,
      ),
    );
  }

  void _onCancelExtractMode(CancelExtractMode event, Emitter<FilesState> emit) {
    emit(
      state.copyWith(
        isExtractMode: false,
        zipFilePaths: [],
        extractStatus: FileExtractStatus.none,
      ),
    );
  }

  Future<void> _onLoadRecentFiles(
    LoadRecentFiles event,
    Emitter<FilesState> emit,
  ) async {
    try {
      final fileSystem = const LocalFileSystem();
      final recentFiles = await recentFilesRepository.getRecentFiles();

      final visibleFiles =
          recentFiles
              .where((recent) => fileSystem.file(recent.path).existsSync())
              .map((recent) => fileSystem.file(recent.path))
              .toList();

      emit(state.copyWith(loading: false, fileSystemList: visibleFiles));
    } catch (e) {
      emit(state.copyWith(loading: false));
    }
  }

  Future<void> _onAddToRecentFiles(
    AddToRecentFiles event,
    Emitter<FilesState> emit,
  ) async {
    await recentFilesRepository.addRecentFile(event.path);
  }

  Future<void> _onSearchFilesInDirectory(
    SearchFilesInDirectory event,
    Emitter<FilesState> emit,
  ) async {
    emit(state.copyWith(loading: true, error: null));

    try {
      final results = await fileRepository.searchFiles(event.path, event.query);
      emit(state.copyWith(fileSystemList: results, loading: false));
    } catch (e) {
      emit(state.copyWith(error: 'Search failed: $e', loading: false));
    }
  }

  FutureOr<void> _onRemoveRecentEntities(
    RemoveRecentEntities event,
    Emitter<FilesState> emit,
  ) async {
    emit(state.copyWith(loading: true, error: null));
    try {
      final fileSystem = const LocalFileSystem();
      await recentFilesRepository.removeRecentFile(event.entitiesPath);
      final recentFiles = await recentFilesRepository.getRecentFiles();
      final recentFilesList =
          recentFiles
              .where((recent) => fileSystem.file(recent.path).existsSync())
              .map((recent) => fileSystem.file(recent.path))
              .toList();

      emit(state.copyWith(loading: false, fileSystemList: recentFilesList));
    } catch (e) {
      emit(state.copyWith(error: 'Remove failed: $e', loading: false));
    }
  }
}
