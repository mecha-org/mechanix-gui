import 'dart:async';

import 'package:equatable/equatable.dart';

abstract class FilesEvent extends Equatable {
  @override
  List<Object?> get props => [];
}

class InitializeFiles extends FilesEvent {}

class LoadFilesAtPath extends FilesEvent {
  final String path;
  LoadFilesAtPath(this.path);
}

class CreateFolder extends FilesEvent {
  final String path;
  final String folderName;

  CreateFolder({required this.path, required this.folderName});
}

class DeleteEntities extends FilesEvent {
  final List<String> entitiesPath;

  DeleteEntities(this.entitiesPath);
}

class Rename extends FilesEvent {
  final String oldPath;
  final String newName;

  Rename({required this.oldPath, required this.newName});
}

class Copy extends FilesEvent {
  final List<String> sourcePaths;
  final String destinationPath;

  Copy({required this.sourcePaths, required this.destinationPath});
}

enum ConflictResolutionStrategy {
  replace,
  skip,
}

class ContinueCopyWithConflictResolution extends FilesEvent {
  final List<String> sourcePaths;
  final String destinationPath;
  final ConflictResolutionStrategy strategy;

  ContinueCopyWithConflictResolution({
    required this.sourcePaths,
    required this.destinationPath,
    required this.strategy,
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
  SortFiles(this.sortBy);
}

class FetchFileDetails extends FilesEvent {
  final String path;

  FetchFileDetails(this.path);
}

class ToggleHiddenFiles extends FilesEvent {
  final String path;

  ToggleHiddenFiles({required this.path});
}

class CompressEntitiesEvent extends FilesEvent {
  final List<String> sourcePaths;
  final String destinationZipPath;

  CompressEntitiesEvent({
    required this.sourcePaths,
    required this.destinationZipPath,
  });
}

class ExtractZipTo extends FilesEvent {
  final String zipFilePath;
  final String targetPath;

  ExtractZipTo(this.zipFilePath, this.targetPath);
}

class StartExtractMode extends FilesEvent {
  final String zipFilePath;
  StartExtractMode(this.zipFilePath);
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
