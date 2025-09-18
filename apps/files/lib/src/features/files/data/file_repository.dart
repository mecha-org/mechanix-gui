import 'package:file/file.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';

abstract class FileRepository {
  Future<List<FileSystemEntity>> getFileSystemList({String path});

  Future<void> createFolder(String path, String folderName);

  Future<void> deleteEntities(List<String> path);

  Future<void> renameEntity(String oldPath, String newName);

  Future<void> copyEntities(
    List<String> sourcePaths,
    String destinationPath, {
    ConflictResolutionStrategy strategy = ConflictResolutionStrategy.replace,
  });

  Future<void> moveEntities(
    List<String> sourcePaths,
    String destinationPath, {
    ConflictResolutionStrategy strategy = ConflictResolutionStrategy.replace,
  });

  Future<List<FileSystemEntity>> sortEntities(
      List<FileSystemEntity> list, String sortBy);

  Future<FileStat> getFileDetails(String path);

  Future<void> compressEntities(
      List<String> sourcePaths, String destinationZipPath);

  Future<void> extractZip(String zipFilePath, String destinationPath);

  Future<bool> entityExists(String path);

  Future<List<FileSystemEntity>> searchFiles(String rootPath, String query);
}
