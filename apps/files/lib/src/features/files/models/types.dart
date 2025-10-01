import 'package:mechanix_files/src/commons/constants.dart';

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
