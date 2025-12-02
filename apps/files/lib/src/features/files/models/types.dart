import 'dart:io' as io;

import 'package:file/file.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/controllers/file_manager_controller.dart';

class FileItem {
  final String name;
  final String type;
  final List<FileItem>? children;
  final DateTime? modified;

  FileItem(
      {required this.name, required this.type, this.children, this.modified});

  @override
  String toString() {
    return 'FileItem(name: $name, type: $type, children: $children, modified: $modified)';
  }
}

extension FileItemIcon on FileItem {
  String get iconPath {
    if (type == 'dir') return Images.unfoldDir;
    if (type == '.pdf') return Images.pdfFile;
    if (type == '.xlsx') return Images.excelFile;
    if (type == '.txt') return Images.textFile;
    if (imageFileTypes.contains(type)) return Images.imageFile;
    if (audioFileTypes.contains(type)) return Images.audioFile;
    if (videoFileTypes.contains(type)) return Images.videoFile;
    if (type == '.csv') return Images.csvFile;
    if (type == '.zip') return Images.archiveFile;
    if (textFileTypes.contains(type)) return Images.codeFile;

    return Images.file;
  }
}

const textFileTypes = [
  '.dart',
  '.yaml',
  '.yml',
  '.sql',
  '.json',
  '.java',
  '.c',
  '.cpp',
  '.js',
  '.py',
  '.ini',
  '.toml',
  '.rb',
  '.xml',
  '.rs',
  '.txt'
];

const audioFileTypes = [
  '.mp3',
  '.wav',
  '.flac',
  '.m4a',
  '.aac',
  '.ogg',
  '.opus'
];

const videoFileTypes = [
  '.mp4',
  '.mkv',
  '.avi',
  '.mov',
  '.wmv',
];

const imageFileTypes = [
  '.png',
  '.jpg',
  '.jpeg',
  '.webp',
  '.svg',
  '.gif',
  '.bmp'
];

const int page = 1;
const int pageSize = 20;

// FileSystemEntity extension
extension FileSystemEntityIcon on io.FileSystemEntity {
  String get iconPath {
    final path = this.path;
    var ext = path.contains('.') ? path.split('.').last.toLowerCase() : 'dir';
    ext = ".$ext";

    if (ext == '.dir') return Images.unfoldDir;
    if (ext == '.pdf') return Images.pdfFile;
    if (ext == '.xlsx' || ext == '.xls') return Images.excelFile;
    if (ext == '.txt') return Images.textFile;
    if (imageFileTypes.contains(ext)) return Images.imageFile;
    if (audioFileTypes.contains(ext)) return Images.audioFile;
    if (videoFileTypes.contains(ext)) return Images.videoFile;
    if (ext == '.csv') return Images.csvFile;
    if (ext == '.zip' || ext == '.rar' || ext == '.7z') {
      return Images.archiveFile;
    }
    if (textFileTypes.contains(ext)) return Images.codeFile;

    return Images.file;
  }
}

SortBy sortByFromKey(String key) {
  switch (key) {
    case 'name':
      return SortBy.name;
    case 'type':
      return SortBy.type;
    case 'mod_time':
      return SortBy.date;
    case 'size_asc':
    case 'size_desc':
      return SortBy.size;
    default:
      return SortBy.name;
  }
}

String keyFromSort(SortBy sortBy, bool ascending) {
  switch (sortBy) {
    case SortBy.name:
      return 'name';
    case SortBy.type:
      return 'type';
    case SortBy.date:
      return 'mod_time';
    case SortBy.size:
      return ascending ? 'size_asc' : 'size_desc';
  }
}

const double menuItemHeight = 42.0;
