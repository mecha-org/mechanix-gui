import 'package:equatable/equatable.dart';
import 'package:file/file.dart';

enum FileCompressionStatus {
  idle,
  inProgress,
  success,
  failure,
}

class FilesState extends Equatable {
  final bool loading;
  final List<FileSystemEntity> fileSystemList;
  final String? error;

  final bool isCopyMode;
  final List<String> copiedPaths;
  final List<String> conflictingPaths;
  final String conflictDestinationPath;

  final bool isMoveMode;
  final List<String> movedPaths;

  final String currentSortBy;

  final FileStat? fileDetails;

  final bool showHiddenFiles;

  //compression state
  final FileCompressionStatus compressionStatus;
  final String? compressedZipPath;
  final String? compressionError;

  final bool isExtractMode;
  final String zipFilePath;

  const FilesState({
    this.loading = false,
    this.fileSystemList = const [],
    this.error,
    this.isCopyMode = false,
    this.copiedPaths = const [],
    this.conflictingPaths = const [],
    required this.conflictDestinationPath,
    this.isMoveMode = false,
    this.movedPaths = const [],
    required this.currentSortBy,
    this.fileDetails,
    this.showHiddenFiles = false,
    this.compressionStatus = FileCompressionStatus.idle,
    this.compressedZipPath,
    this.compressionError,
    this.isExtractMode = false,
    this.zipFilePath = '',
  });

  FilesState copyWith({
    bool? loading,
    List<FileSystemEntity>? fileSystemList,
    String? error,
    bool? isCopyMode,
    List<String>? copiedPaths,
    List<String>? conflictingPaths,
    String? conflictDestinationPath,
    bool? isMoveMode,
    List<String>? movedPaths,
    String? currentSortBy,
    FileStat? fileDetails,
    bool? showHiddenFiles,
    // Compression
    FileCompressionStatus? compressionStatus,
    String? compressedZipPath,
    String? compressionError,
    bool? isExtractMode,
    String? zipFilePath,
  }) {
    return FilesState(
      loading: loading ?? this.loading,
      fileSystemList: fileSystemList ?? this.fileSystemList,
      error: error ?? this.error,
      isCopyMode: isCopyMode ?? this.isCopyMode,
      copiedPaths: copiedPaths ?? this.copiedPaths,
      isMoveMode: isMoveMode ?? this.isMoveMode,
      movedPaths: movedPaths ?? this.movedPaths,
      currentSortBy: currentSortBy ?? this.currentSortBy,
      fileDetails: fileDetails ?? this.fileDetails,
      showHiddenFiles: showHiddenFiles ?? this.showHiddenFiles,
      compressionStatus: compressionStatus ?? this.compressionStatus,
      compressedZipPath: compressedZipPath ?? this.compressedZipPath,
      compressionError: compressionError ?? this.compressionError,
      isExtractMode: isExtractMode ?? this.isExtractMode,
      zipFilePath: zipFilePath ?? this.zipFilePath,
      conflictingPaths: conflictingPaths ?? this.conflictingPaths,
      conflictDestinationPath:
          conflictDestinationPath ?? this.conflictDestinationPath,
    );
  }

  @override
  List<Object?> get props => [
        loading,
        fileSystemList,
        isCopyMode,
        copiedPaths,
        error,
        isMoveMode,
        movedPaths,
        currentSortBy,
        fileDetails,
        compressionStatus,
        compressedZipPath,
        compressionError,
        isExtractMode,
        zipFilePath,
        conflictingPaths,
        conflictDestinationPath,
      ];
}
