import 'dart:async';
import 'package:archive/archive_io.dart';
import 'package:file/file.dart';
import 'package:file/local.dart';
import 'package:logger/web.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/data/file_repository.dart';
import 'package:path/path.dart' as p;
import 'package:archive/archive.dart';
import 'dart:typed_data';
import 'dart:convert';

class FileRepositoryImpl implements FileRepository {
  final FileSystem _fs = LocalFileSystem();
  final Logger logger = Logger();

  @override
  Future<List<FileSystemEntity>> getFileSystemList({String path = '/'}) async {
    try {
      final Directory dir = _fs.directory(path);

      if (!dir.existsSync()) {
        logger.w("Directory does not exist: $path");
        return [];
      }

      final List<FileSystemEntity> contents = dir.listSync();
      return contents;
    } catch (e, stackTrace) {
      logger.e("Failed to list file system at $path: $e, $stackTrace");
      return [];
    }
  }

  @override
  Future<void> createFolder(String path, String folderName) async {
    final dir = _fs.directory(path).childDirectory(folderName);
    if (!dir.existsSync()) {
      dir.createSync();
    }
  }

  @override
  Future<void> deleteEntities(List<String> paths) async {
    for (final path in paths) {
      final file = _fs.file(path);
      final dir = _fs.directory(path);

      if (await file.exists()) {
        await file.delete();
      } else if (await dir.exists()) {
        await dir.delete(recursive: true);
      }
    }
  }

  @override
  Future<void> renameEntity(String oldPath, String newName) async {
    final newPath = p.join(p.dirname(oldPath), newName);
    final type = _fs.typeSync(oldPath);

    if (type == FileSystemEntityType.directory) {
      await _fs.directory(oldPath).rename(newPath);
    } else {
      await _fs.file(oldPath).rename(newPath);
    }
  }

  @override
  Future<void> copyEntities(
    List<String> sourcePaths,
    String destinationPath, {
    ConflictResolutionStrategy strategy = ConflictResolutionStrategy.replace,
  }) async {
    for (final srcPath in sourcePaths) {
      final name = p.basename(srcPath);
      final destPath = p.join(destinationPath, name);
      final type = _fs.file(srcPath).statSync().type;

      final exists = await _fs.file(destPath).exists() ||
          await _fs.directory(destPath).exists();

      if (exists) {
        if (strategy == ConflictResolutionStrategy.skip) {
          continue; // Skip copying this one
        } else if (strategy == ConflictResolutionStrategy.replace) {
          // Delete existing before overwriting
          await _fs.directory(destPath).exists()
              ? await _fs.directory(destPath).delete(recursive: true)
              : await _fs.file(destPath).delete();
        }
      }

      if (type == FileSystemEntityType.file) {
        await _fs.file(srcPath).copy(destPath);
      } else if (type == FileSystemEntityType.directory) {
        await _copyDirectory(_fs.directory(srcPath), _fs.directory(destPath));
      } else {
        logger.w('Unknown or unsupported entity: $srcPath');
      }
    }
  }

  Future<void> _copyDirectory(Directory source, Directory destination) async {
    if (!destination.existsSync()) {
      destination.createSync(recursive: true);
    }

    await for (var entity in source.list(recursive: false)) {
      final newPath = p.join(destination.path, p.basename(entity.path));

      if (entity is File) {
        await entity.copy(newPath);
      } else if (entity is Directory) {
        await _copyDirectory(entity, _fs.directory(newPath));
      }
    }
  }

  @override
  Future<void> moveEntities(
    List<String> sourcePaths,
    String destinationPath, {
    ConflictResolutionStrategy strategy = ConflictResolutionStrategy.replace,
  }) async {
    for (final path in sourcePaths) {
      final name = p.basename(path);
      final newPath = p.join(destinationPath, name);

      final isFile = await _fs.file(path).exists();
      final entity = isFile ? _fs.file(path) : _fs.directory(path);

      final destFile = _fs.file(newPath);
      final destDir = _fs.directory(newPath);

      final destExists = await destFile.exists() || await destDir.exists();

      if (destExists) {
        switch (strategy) {
          case ConflictResolutionStrategy.skip:
            continue;
          case ConflictResolutionStrategy.replace:
            if (await destDir.exists()) {
              await destDir.delete(recursive: true);
            } else if (await destFile.exists()) {
              await destFile.delete();
            }
            break;
        }
      }

      await entity.rename(newPath);
    }
  }

  @override
  Future<List<FileSystemEntity>> sortEntities(
      List<FileSystemEntity> list, String sortBy) async {
    final sorted = List<FileSystemEntity>.from(list);
    final Map<String, int> sizeMap = {};

    sorted.sort((a, b) {
      final aName = p.basename(a.path).toLowerCase();
      final bName = p.basename(b.path).toLowerCase();

      switch (sortBy) {
        case 'name':
          // Group folders first
          if (a is Directory && b is! Directory) return -1;
          if (b is Directory && a is! Directory) return 1;

          // If both are the same type, sort by name
          return aName.compareTo(bName);

        case 'type':
          if (a is Directory && b is! Directory) return -1;
          if (b is Directory && a is! Directory) return 1;

          // Same type
          final aType = a is Directory ? 'dir' : p.extension(a.path);
          final bType = b is Directory ? 'dir' : p.extension(b.path);
          final typeCompare = aType.compareTo(bType);
          if (typeCompare != 0) return typeCompare;

          // Secondary: by name
          final aName = p.basename(a.path).toLowerCase();
          final bName = p.basename(b.path).toLowerCase();
          return aName.compareTo(bName);

        case 'size_asc':
          // Group folders first
          if (a is Directory && b is! Directory) return -1;
          if (b is Directory && a is! Directory) return 1;

          // Only compare files by size
          if (a is File && b is File) {
            sizeMap[a.path] ??= a.lengthSync();
            sizeMap[b.path] ??= b.lengthSync();
            final aSize = sizeMap[a.path]!;
            final bSize = sizeMap[b.path]!;
            return aSize.compareTo(bSize); // ascending
          }

          // If both are directories, sort alphabetically
          final aName = p.basename(a.path).toLowerCase();
          final bName = p.basename(b.path).toLowerCase();
          return aName.compareTo(bName);

        case 'size_desc':
          // Group folders last
          if (a is Directory && b is! Directory) return 1;
          if (b is Directory && a is! Directory) return -1;

          // Only compare files by size
          if (a is File && b is File) {
            sizeMap[a.path] ??= a.lengthSync();
            sizeMap[b.path] ??= b.lengthSync();
            final aSize = sizeMap[a.path]!;
            final bSize = sizeMap[b.path]!;
            return bSize.compareTo(aSize); // descending
          }

          // If both are directories, sort alphabetically
          final aName = p.basename(a.path).toLowerCase();
          final bName = p.basename(b.path).toLowerCase();
          return aName.compareTo(bName);

        case 'mod_time':
          final aTime = a.statSync().modified;
          final bTime = b.statSync().modified;
          return bTime.compareTo(aTime);

        default:
          return 0;
      }
    });

    return sorted;
  }

  @override
  Future<FileStat> getFileDetails(String path) async {
    final entity =
        _fs.file(path).existsSync() ? _fs.file(path) : _fs.directory(path);
    return entity.statSync();
  }

  @override
  Future<void> compressEntities(
      List<String> sourcePaths, String destinationZipPath) async {
    final archive = Archive();

    for (final path in sourcePaths) {
      final type = _fs.file(path).statSync().type;
      final nameInArchive = p.basename(path);

      if (type == FileSystemEntityType.file) {
        final file = _fs.file(path);
        final data = await file.readAsBytes();
        archive.addFile(ArchiveFile(nameInArchive, data.length, data));
      } else if (type == FileSystemEntityType.directory) {
        await _addDirectoryToArchive(archive, _fs.directory(path),
            basePath: path);
      }
    }

    final zipData = ZipEncoder().encode(archive)!;
    final zipFile = _fs.file(destinationZipPath);
    await zipFile.writeAsBytes(zipData);
  }

  Future<void> _addDirectoryToArchive(
    Archive archive,
    Directory dir, {
    required String basePath,
  }) async {
    final entities = dir.listSync(recursive: true, followLinks: false);

    for (final entity in entities) {
      final relativePath = p.relative(entity.path, from: basePath);
      if (entity is File) {
        final data = await entity.readAsBytes();
        archive.addFile(ArchiveFile(relativePath, data.length, data));
      } else if (entity is Directory) {
        archive.addFile(
            ArchiveFile('$relativePath/', 0, Uint8List(0))..isFile = false);
      }
    }
  }

  @override
  Future<void> extractZip(String zipFilePath, String destinationPath) async {
    final zipFile = _fs.file(zipFilePath);
    if (!zipFile.existsSync()) {
      throw Exception('Zip file not found: $zipFilePath');
    }

    // Get the base name of the ZIP file (without extension)
    final zipFileName = _fs.path.basenameWithoutExtension(zipFilePath);

    // Create a folder inside the destination with this name
    final extractRoot = _fs.path.join(destinationPath, zipFileName);
    _fs.directory(extractRoot).createSync(recursive: true);

    // Read and decode the zip
    final bytes = zipFile.readAsBytesSync();
    final Archive archive = ZipDecoder().decodeBytes(bytes);

    for (final ArchiveFile file in archive) {
      final String filename = file.name;
      final outPath = _fs.path.join(extractRoot, filename);

      if (file.isFile) {
        final content = file.content;
        late List<int> data;

        if (content is List<int>) {
          data = content;
        } else if (content is String) {
          data = utf8.encode(content); // convert String to bytes
        } else {
          throw Exception(
              'Unsupported file content type: ${content.runtimeType}');
        }

        _fs.file(outPath)
          ..createSync(recursive: true)
          ..writeAsBytesSync(data);
      } else {
        _fs.directory(outPath).createSync(recursive: true);
      }
    }
  }

  @override
  Future<bool> entityExists(String path) async {
    final fileExists = await _fs.file(path).exists();
    if (fileExists) return true;

    final dirExists = await _fs.directory(path).exists();
    return dirExists;
  }
}
