import 'package:equatable/equatable.dart';
import 'package:file/file.dart';

enum FileCompressionStatus {
  idle,
  inProgress,
  success,
  failure,
}

enum FileExtractStatus {
  none,
  inProgress,
  completed,
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
  final bool isAscending;

  final FileStat? fileDetails;

  final bool showHiddenFiles;

  //compression state
  final FileCompressionStatus compressionStatus;
  final String? compressedZipPath;
  final String? compressionError;

  final bool isExtractMode;
  final List<String> zipFilePaths;
  final int extractSuccessCount;
  final int extractFailureCount;
  final FileExtractStatus extractStatus;
  final String? extractError;

  final int currentPage;
  final bool hasMorePages;

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
    required this.isAscending,
    this.fileDetails,
    this.showHiddenFiles = false,
    this.compressionStatus = FileCompressionStatus.idle,
    this.compressedZipPath,
    this.compressionError,
    this.isExtractMode = false,
    this.extractStatus = FileExtractStatus.none,
    this.extractSuccessCount = 0,
    this.extractFailureCount = 0,
    this.extractError,
    this.zipFilePaths = const [],
    this.currentPage = 1,
    this.hasMorePages = true,
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
    bool? isAscending,
    FileStat? fileDetails,
    bool? showHiddenFiles,
    // Compression
    FileCompressionStatus? compressionStatus,
    String? compressedZipPath,
    String? compressionError,
    bool? isExtractMode,
    FileExtractStatus? extractStatus,
    int? extractSuccessCount,
    int? extractFailureCount,
    String? extractError,
    List<String>? zipFilePaths,
    int? currentPage,
    bool? hasMorePages,
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
      isAscending: isAscending ?? this.isAscending,
      fileDetails: fileDetails ?? this.fileDetails,
      showHiddenFiles: showHiddenFiles ?? this.showHiddenFiles,
      compressionStatus: compressionStatus ?? this.compressionStatus,
      compressedZipPath: compressedZipPath ?? this.compressedZipPath,
      compressionError: compressionError ?? this.compressionError,
      isExtractMode: isExtractMode ?? this.isExtractMode,
      extractStatus: extractStatus ?? this.extractStatus,
      extractSuccessCount: extractSuccessCount ?? this.extractSuccessCount,
      extractFailureCount: extractFailureCount ?? this.extractFailureCount,
      extractError: extractError ?? this.extractError,
      zipFilePaths: zipFilePaths ?? this.zipFilePaths,
      conflictingPaths: conflictingPaths ?? this.conflictingPaths,
      conflictDestinationPath:
          conflictDestinationPath ?? this.conflictDestinationPath,
      currentPage: currentPage ?? this.currentPage,
      hasMorePages: hasMorePages ?? this.hasMorePages,
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
        extractStatus,
        extractError,
        zipFilePaths,
        conflictingPaths,
        conflictDestinationPath,
        currentPage,
        hasMorePages,
      ];
}
