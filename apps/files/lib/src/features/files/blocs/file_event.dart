import 'dart:async';

import 'package:equatable/equatable.dart';
import 'package:mechanix_files/src/controllers/file_manager_controller.dart';

abstract class FilesEvent extends Equatable {
  @override
  List<Object?> get props => [];
}

class InitializeFiles extends FilesEvent {}

class CreateFolder extends FilesEvent {
  final String path;
  final String folderName;
  final FileManagerController controller;

  CreateFolder(
      {required this.path, required this.folderName, required this.controller});
}

class DeleteEntities extends FilesEvent {
  final List<String> entitiesPath;
  final FileManagerController controller;

  DeleteEntities(this.entitiesPath, this.controller);
}

class Rename extends FilesEvent {
  final String oldPath;
  final String newName;
  final FileManagerController controller;

  Rename(
      {required this.oldPath, required this.newName, required this.controller});
}

class Copy extends FilesEvent {
  final List<String> sourcePaths;
  final String destinationPath;
  final FileManagerController? controller;

  Copy(
      {required this.sourcePaths,
      required this.destinationPath,
      required this.controller});
}

enum ConflictResolutionStrategy {
  replace,
  skip,
}

class ContinueCopyWithConflictResolution extends FilesEvent {
  final List<String> sourcePaths;
  final String destinationPath;
  final ConflictResolutionStrategy strategy;
  final FileManagerController? controller;

  ContinueCopyWithConflictResolution({
    required this.sourcePaths,
    required this.destinationPath,
    required this.strategy,
    required this.controller,
  });
}

class StartCopyMode extends FilesEvent {
  final List<String> copiedPaths;
  StartCopyMode(this.copiedPaths);
}

class CancelCopyMode extends FilesEvent {}

class Move extends FilesEvent {
  final List<String> sourcePaths;
  final String destinationPath;
  final Completer<void>? completer;

  Move({
    required this.sourcePaths,
    required this.destinationPath,
    this.completer,
  });
}

class ContinueMoveWithConflictResolution extends FilesEvent {
  final List<String> sourcePaths;
  final String destinationPath;
  final ConflictResolutionStrategy strategy;

  ContinueMoveWithConflictResolution({
    required this.sourcePaths,
    required this.destinationPath,
    required this.strategy,
  });
}

class StartMoveMode extends FilesEvent {
  final List<String> movedPaths;
  StartMoveMode(this.movedPaths);
}

class CancelMoveMode extends FilesEvent {}

class SortFiles extends FilesEvent {
  final String sortBy; // e.g., 'name', 'size_asc', 'type', etc.
  final bool isAscending; // true / false
  SortFiles(this.sortBy, this.isAscending);
}

class FetchFileDetails extends FilesEvent {
  final String path;

  FetchFileDetails(this.path);
}

class ToggleHiddenFiles extends FilesEvent {
  ToggleHiddenFiles();
}

class CompressEntitiesEvent extends FilesEvent {
  final List<String> sourcePaths;
  final String destinationZipPath;
  final FileManagerController? controller;

  CompressEntitiesEvent({
    required this.sourcePaths,
    required this.destinationZipPath,
    required this.controller,
  });
}

class ExtractZipTo extends FilesEvent {
  final String zipPath;
  final String targetPath;
  final Completer completer;
  final int index;
  final int total;

  ExtractZipTo(
    this.zipPath,
    this.targetPath,
    this.completer, {
    required this.index,
    required this.total,
  });
}

class ExtractZipBatchCompleted extends FilesEvent {
  final int successCount;
  final int failureCount;

  ExtractZipBatchCompleted({
    required this.successCount,
    required this.failureCount,
  });
}

class StartExtractMode extends FilesEvent {
  final List<String> zipFilePaths;
  StartExtractMode(this.zipFilePaths);
}

class CancelExtractMode extends FilesEvent {}

class LoadRecentFiles extends FilesEvent {}

class AddToRecentFiles extends FilesEvent {
  final String path;
  AddToRecentFiles(this.path);
}

class SearchFilesInDirectory extends FilesEvent {
  final String path;
  final String query;

  SearchFilesInDirectory(this.path, this.query);
}

class ClearSearchResults extends FilesEvent {}
