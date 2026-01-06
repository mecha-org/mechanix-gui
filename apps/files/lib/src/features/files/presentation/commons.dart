import 'dart:async';
import 'dart:io' as io;
import 'dart:math' as Math;
import 'dart:ui' as ui;

import 'package:archive/archive.dart';
import 'package:file/file.dart';
import 'package:file/local.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/src/controllers/file_manager_controller.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:mechanix_files/src/features/preview/presentation/audio_player.dart';
import 'package:mechanix_files/src/features/preview/presentation/code_editor.dart';
import 'package:mechanix_files/src/features/preview/presentation/image_viewer.dart';
import 'package:mechanix_files/src/features/preview/presentation/pdf_viewer.dart';
import 'package:mechanix_files/src/features/preview/presentation/video_player.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/filled_button/mechanix_filled_button_theme.dart';
import 'files.dart';
import 'package:path/path.dart' as p;
import 'package:intl/intl.dart';

String formatModifiedTime(DateTime modified) {
  final now = DateTime.now();

  final isSameDay = now.year == modified.year &&
      now.month == modified.month &&
      now.day == modified.day;

  final isSameYear = now.year == modified.year;

  if (isSameDay) {
    return DateFormat.jm().format(modified); // e.g., 12:30 PM
  } else if (isSameYear) {
    return DateFormat('dd MMM').format(modified); // e.g., 20 Jul
  } else {
    return DateFormat('dd MMM yyyy').format(modified); // e.g., 20 Jul 2025
  }
}

String formatDateTime(DateTime dateTime) {
  final formatter = DateFormat('dd-MM-yyyy, hh:mm a');
  return formatter.format(dateTime).toLowerCase();
}

String formatBytes(int bytes, [int decimals = 2]) {
  if (bytes <= 0) return "0 B";
  const suffixes = ["B", "KB", "MB", "GB", "TB"];
  final i = (bytes == 0) ? 0 : (Math.log(bytes) / Math.log(1024)).floor();
  final size = bytes / Math.pow(1024, i);
  return "${size.toStringAsFixed(decimals)} ${suffixes[i]}";
}

void _navigateToDirectory(
  BuildContext context,
  List<FileItem> currentPath,
  FileItem directory,
) {
  final newPath = [...currentPath, directory];
  final pathString = '/${newPath.map((e) => e.name).join('/')}';

  Navigator.push(
    context,
    MaterialPageRoute(
      builder: (_) => FileExplorerPage(
        title: directory.name,
        startPath: pathString,
      ),
    ),
  );
}

void handleTap(
  BuildContext context,
  FileItem file,
  List<FileItem> currentPath,
  String fullPath,
  bool isSelectionMode,
  FileExplorerPageState? state,
) {
  final fileType = p.extension(fullPath).toLowerCase();

  if (isSelectionMode) {
    state?.toggleSelection(fullPath);
    return;
  }

  if (state?.isSearching == true) {
    state?.clearSearch(); // will reset and remove overlay
  }

  if (file.type == 'dir') {
    _navigateToDirectory(context, currentPath, file);
    return;
  }

  if (textFileTypes.contains(fileType)) {
    context.read<FilesBloc>().add(AddToRecentFiles(fullPath));

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (_) =>
            CodeEditorPage(rootContext: context, filePath: fullPath),
      ),
    );
    return;
  }

  if (audioFileTypes.contains(fileType)) {
    context.read<FilesBloc>().add(AddToRecentFiles(fullPath));

    showDialog(
      context: context,
      barrierDismissible: false,
      builder: (_) =>
          AudioPlayerOverlay(rootContext: context, filePath: fullPath),
    );
    return;
  }

  if (videoFileTypes.contains(fileType)) {
    context.read<FilesBloc>().add(AddToRecentFiles(fullPath));

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (_) => VideoPlayer(rootContext: context, filePath: fullPath),
      ),
    );
    return;
  }

  if (fileType == '.pdf') {
    context.read<FilesBloc>().add(AddToRecentFiles(fullPath));

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (_) => PdfViewerPage(rootContext: context, filePath: fullPath),
      ),
    );
    return;
  }

  if (imageFileTypes.contains(fileType)) {
    context.read<FilesBloc>().add(AddToRecentFiles(fullPath));

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (_) =>
            ImageViewerPage(rootContext: context, filePath: fullPath),
      ),
    );
    return;
  }
}

void handleFileTap(
  BuildContext context,
  io.FileSystemEntity file,
  String fullPath,
  bool isSelectionMode,
  FileExplorerPageState? state,
  FileManagerController controller,
) {
  final fileType = p.extension(fullPath).toLowerCase();

  if (isSelectionMode) {
    state?.toggleSelection(fullPath);
    return;
  }

  if (state?.isSearching == true) {
    state?.clearSearch(); // will reset and remove overlay
  }

  if (textFileTypes.contains(fileType)) {
    context.read<FilesBloc>().add(AddToRecentFiles(fullPath));

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (_) =>
            CodeEditorPage(rootContext: context, filePath: fullPath),
      ),
    );
    return;
  }

  if (audioFileTypes.contains(fileType)) {
    context.read<FilesBloc>().add(AddToRecentFiles(fullPath));

    showDialog(
      context: context,
      barrierDismissible: false,
      builder: (_) =>
          AudioPlayerOverlay(rootContext: context, filePath: fullPath),
    );
    return;
  }

  if (videoFileTypes.contains(fileType)) {
    context.read<FilesBloc>().add(AddToRecentFiles(fullPath));

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (_) => VideoPlayer(rootContext: context, filePath: fullPath),
      ),
    );
    return;
  }

  if (fileType == '.pdf') {
    context.read<FilesBloc>().add(AddToRecentFiles(fullPath));

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (_) => PdfViewerPage(rootContext: context, filePath: fullPath),
      ),
    );
    return;
  }

  if (imageFileTypes.contains(fileType)) {
    context.read<FilesBloc>().add(AddToRecentFiles(fullPath));

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (_) =>
            ImageViewerPage(rootContext: context, filePath: fullPath),
      ),
    );
    return;
  }

  if (fileType == '.zip') {
    context.read<FilesBloc>().add(AddToRecentFiles(fullPath));
    final state = context.findAncestorStateOfType<FileExplorerPageState>();
    state?.handleExtraction(context, {fullPath});
    return;
  }
}

bool isZipFileValid(String path) {
  final FileSystem fileSystem = const LocalFileSystem();

  try {
    final bytes = fileSystem.file(path).readAsBytesSync();
    final archive = ZipDecoder().decodeBytes(bytes);
    return archive.isNotEmpty;
  } catch (_) {
    return false;
  }
}

/// Helper function to extract the current folder name
String getCurrentFolderName(String path) {
  if (path.isEmpty) return 'Home';

  // Use path utilities instead of instantiating Directory (which may be
  // shadowed by package:file's abstract Directory).
  String name = p.basename(path);

  // If path ends with a separator, basename can be empty; fall back to parent.
  if (name.isEmpty) {
    name = p.basename(p.dirname(path));
  }

  // Optionally map specific directories to nicer names
  if (name == 'home') return 'Home';

  return name;
}

/// Ensures a unique folder name by appending "(1)", "(2)", etc.
Future<String> getUniqueExtractPath(String basePath) async {
  final io.Directory dir = io.Directory(basePath);

  if (!await dir.exists()) {
    return basePath; // safe, doesn't exist yet
  }

  final parent = p.dirname(basePath);
  final name = p.basename(basePath);
  int count = 1;

  while (true) {
    final newPath = p.join(parent, '$name ($count)');
    if (!await io.Directory(newPath).exists()) {
      return newPath;
    }
    count++;
  }
}

Future<String> generateUniqueFolderName(String basePath) async {
  const String baseName = "New Folder";

  // First check the default folder name
  String candidate = p.join(basePath, baseName);

  int counter = 1;

  // If "New Folder" exists, try "New Folder (1)", "New Folder (2)"...
  while (await io.Directory(candidate).exists()) {
    candidate = p.join(basePath, '$baseName ($counter)');
    counter++;
  }

  // Only return the folder name, not full path
  return p.basename(candidate);
}

List<FileItem> getFilesAtPath(
    List<FileItem> path, List<FileSystemEntity> fileSystemList) {
  // Build full path from root and path list
  String currentPath = '/';
  for (final item in path) {
    currentPath = p.join(currentPath, item.name);
  }

  final List<FileItem> items = [];

  try {
    for (final entity in fileSystemList) {
      final String name = p.basename(entity.path);
      if (name.isEmpty) continue;

      final stat = entity.statSync();
      final modifiedTime = stat.modified;

      if (entity is Directory) {
        items.add(FileItem(name: name, type: 'dir', modified: modifiedTime));
      } else if (entity is File) {
        final ext = p.extension(name);
        items.add(FileItem(
            name: name,
            type: ext.isNotEmpty ? ext : 'file',
            modified: modifiedTime));
      }
    }
  } catch (e) {
    print('Error reading directory at $currentPath: $e');
  }

  return items;
}

enum MechanixButtonType {
  action,
  delete,
  cancel,
  disable,
}

MechanixFilledButtonThemeData buttonThemeData(
  BuildContext context, {
  MechanixButtonType type = MechanixButtonType.action,
  Size size = const Size(246, 40),
}) {
  Color backgroundColor;
  bool isDisabled = type == MechanixButtonType.disable;

  switch (type) {
    case MechanixButtonType.delete:
      backgroundColor = const Color(0xFFD3002A); // red
      break;

    case MechanixButtonType.cancel:
      backgroundColor = context.colorScheme.secondary; // dark grey
      break;

    case MechanixButtonType.disable:
      backgroundColor = context.colorScheme.surfaceContainerHigh; // dark grey
      break;

    case MechanixButtonType.action:
    default:
      backgroundColor = context.colorScheme.primaryFixed; // theme primary
      break;
  }

  return MechanixFilledButtonThemeData(
      buttonSize: size,
      buttonColor: backgroundColor,
      pressedButtonColor: Color.lerp(backgroundColor, Colors.white, 0.12)!,
      textStyle: TextStyle(
        color: isDisabled
            ? context.colorScheme.onSurface
            : context.colorScheme.surfaceContainerLowest,
        fontSize: 18,
        fontWeight: FontWeight.w400,
      ));
}

TextStyle regularStyle(BuildContext context) => TextStyle(
      color: context.colorScheme.onSurface,
      fontSize: 20,
      fontWeight: FontWeight.w400,
    );

TextStyle boldStyle(BuildContext context) => TextStyle(
      color: context.colorScheme.onSurface,
      fontSize: 20,
      fontWeight: FontWeight.w600,
    );

TextStyle previewTitleStyle(BuildContext context) => TextStyle(
      color: context.colorScheme.primary,
      fontSize: 20,
      fontWeight: FontWeight.w600,
    );

Widget searchNavButton({
  required IconData icon,
  required VoidCallback? onTap,
  required BuildContext context,
}) {
  final isEnabled = onTap != null;
  final primary = context.colorScheme.primaryFixed;

  return GestureDetector(
    onTap: onTap,
    child: Container(
      margin: const EdgeInsets.symmetric(horizontal: 4),
      padding: const EdgeInsets.all(6),
      decoration: isEnabled
          ? BoxDecoration(
              color: isEnabled
                  ? context.colorScheme.onPrimary
                  : context.colorScheme.tertiary,
              borderRadius: BorderRadius.circular(6),
            )
          : null,
      child: Icon(icon,
          size: 24,
          color:
              isEnabled ? primary : context.colorScheme.surfaceContainerHigh),
    ),
  );
}

Future<String> generateUniqueZipName({
  required String destinationDir,
  required String baseName,
}) async {
  int index = 0;
  String name;
  String fullPath;

  do {
    name = index == 0 ? '$baseName.zip' : '$baseName ($index).zip';
    fullPath = p.join(destinationDir, name);
    index++;
  } while (await io.File(fullPath).exists());

  return name;
}

double textWidth(String text, TextStyle style) {
  final painter = TextPainter(
    text: TextSpan(text: text, style: style),
    maxLines: 1,
    textDirection: ui.TextDirection.ltr,
  )..layout();

  return painter.width;
}
