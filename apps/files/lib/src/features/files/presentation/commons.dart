import 'dart:async';

import 'package:archive/archive.dart';
import 'package:file/file.dart';
import 'package:file/local.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:mechanix_files/src/features/preview/presentation/audio_player.dart';
import 'package:mechanix_files/src/features/preview/presentation/code_editor.dart';
import 'package:mechanix_files/src/features/preview/presentation/csv_viewer.dart';
import 'package:mechanix_files/src/features/preview/presentation/excel_viewer.dart';
import 'package:mechanix_files/src/features/preview/presentation/image_viewer.dart';
import 'package:mechanix_files/src/features/preview/presentation/pdf_viewer.dart';
import 'package:mechanix_files/src/features/preview/presentation/video_player.dart';
import 'files.dart';
import 'package:path/path.dart' as p;
import 'package:intl/intl.dart';

String formatModifiedTime(DateTime modified) {
  final now = DateTime.now();

  final isSameDay = now.year == modified.year &&
      now.month == modified.month &&
      now.day == modified.day;

  if (isSameDay) {
    return DateFormat.jm().format(modified); // e.g., 12:30 PM
  } else {
    return DateFormat('dd-MMM-yyyy').format(modified); // e.g., 20-Jul-2025
  }
}

String formatDateTime(DateTime dateTime) {
  final formatter = DateFormat('dd-MM-yyyy, hh:mm a');
  return formatter.format(dateTime).toLowerCase();
}

void _navigateToDirectory(
  BuildContext context,
  List<FileItem> currentPath,
  FileItem directory,
) {
  final newPath = [...currentPath, directory];
  final pathString = '/${newPath.map((e) => e.name).join('/')}';
  final filesBloc = BlocProvider.of<FilesBloc>(context);
  filesBloc.add(LoadFilesAtPath(pathString, page, pageSize));

  Navigator.push(
    context,
    MaterialPageRoute(
      builder: (_) => BlocProvider.value(
        value: filesBloc,
        child: FileExplorerPage(
          title: directory.name,
          path: newPath,
          // homeContext: context,
        ),
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
        builder: (_) => CodeEditorPage(filePath: fullPath),
      ),
    );
    return;
  }

  if (audioFileTypes.contains(fileType)) {
    context.read<FilesBloc>().add(AddToRecentFiles(fullPath));

    showDialog(
      context: context,
      barrierDismissible: false,
      builder: (_) => AudioPlayerOverlay(filePath: fullPath),
    );
    return;
  }

  if (videoFileTypes.contains(fileType)) {
    context.read<FilesBloc>().add(AddToRecentFiles(fullPath));

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (_) => VideoPlayer(filePath: fullPath),
      ),
    );
    return;
  }

  if (fileType == '.pdf') {
    context.read<FilesBloc>().add(AddToRecentFiles(fullPath));

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (_) => PdfViewerPage(filePath: fullPath),
      ),
    );
    return;
  }

  if (fileType == '.xlsx') {
    context.read<FilesBloc>().add(AddToRecentFiles(fullPath));

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (_) => ExcelViewer(filePath: fullPath),
      ),
    );
    return;
  }

  if (fileType == '.csv') {
    context.read<FilesBloc>().add(AddToRecentFiles(fullPath));

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (_) => CsvViewer(filePath: fullPath),
      ),
    );
    return;
  }

  if (imageFileTypes.contains(fileType)) {
    context.read<FilesBloc>().add(AddToRecentFiles(fullPath));

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (_) => ImageViewerPage(imagePath: fullPath),
      ),
    );
    return;
  }

  if (fileType == '.zip') {
    context.read<FilesBloc>().add(AddToRecentFiles(fullPath));
    final state = context.findAncestorStateOfType<FileExplorerPageState>();

    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.grey[850],
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(16)),
      ),
      builder: (ctx) {
        return Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            ListTile(
              minTileHeight: 20,
              leading: const Icon(Icons.folder_zip, color: Colors.white70),
              title: const Text("Extract here",
                  style: TextStyle(color: Colors.white, fontSize: 14)),
              onTap: () async {
                Navigator.pop(ctx);

                // Validate zip file
                if (!isZipFileValid(fullPath)) {
                  ScaffoldMessenger.of(context).showSnackBar(
                    const SnackBar(
                      content: Text(
                        "ZIP file is corrupted or invalid",
                        style: TextStyle(color: Colors.red),
                      ),
                      backgroundColor: Colors.white,
                    ),
                  );
                  return;
                }

                // Extract in same folder
                final currentDir =
                    fullPath.substring(0, fullPath.lastIndexOf('/'));
                final bloc = context.read<FilesBloc>();
                final completer = Completer<void>();
                bloc.add(StartExtractMode(fullPath));

                bloc.add(ExtractZipTo(
                  fullPath,
                  currentDir,
                  completer,
                ));
                await completer.future;
                bloc.add(CancelExtractMode());

                // reload after extraction
                bloc.add(LoadFilesAtPath(currentDir, page, pageSize));

                ScaffoldMessenger.of(ctx).showSnackBar(
                  SnackBar(
                    content: const Text("Finished extracting",
                        style: TextStyle(color: Colors.white)),
                    duration: const Duration(seconds: 2),
                    backgroundColor: Colors.grey[800],
                  ),
                );
              },
            ),
            const Divider(height: 1, color: Colors.white24),
            ListTile(
              minTileHeight: 20,
              leading: const Icon(Icons.drive_file_move, color: Colors.white70),
              title: const Text("Extract to...",
                  style: TextStyle(color: Colors.white, fontSize: 14)),
              onTap: () {
                Navigator.pop(ctx);

                // Handle extraction
                state?.handleExtract(fullPath);
              },
            ),
          ],
        );
      },
    );

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
