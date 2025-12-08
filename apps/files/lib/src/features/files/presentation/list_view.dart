import 'dart:io' as io;
import 'dart:ui';

import 'package:file/file.dart';
import 'package:flutter/material.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_circular_checkbox.dart';
import 'package:mechanix_files/src/commons/styles/file_theme_extenstions.dart';
import 'package:mechanix_files/src/controllers/file_manager.dart';
import 'package:mechanix_files/src/controllers/file_manager_controller.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:mechanix_files/src/features/files/presentation/extract_file_dialog.dart';
import 'package:mechanix_files/src/features/files/presentation/files_home.dart';
import 'package:mechanix_files/src/features/files/presentation/move_file_dialog.dart';
import 'package:widgets/mechanix.dart';
import 'files.dart';
import 'package:path/path.dart' as p;

Widget buildListView(
  BuildContext context,
  ScrollController scrollController,
  FileManagerController controller,
) {
  final state = context.findAncestorStateOfType<FileExplorerPageState>();
  final isSelectionMode = state?.selectionMode ?? false;
  final selectedPaths = state?.selectedPaths ?? {};
  final isSearching = state?.isSearching ?? false;

  return ValueListenableBuilder<List<io.FileSystemEntity>>(
    valueListenable: controller.paginatedEntities,
    builder: (context, entities, _) {
      if (entities.isEmpty) {
        // Show message if folder is empty
        return Center(
          child: Text(
            "Folder is empty",
            style: Theme.of(context).textTheme.bodyLarge?.copyWith(
                  color: Colors.grey,
                ),
          ),
        );
      }

      return ScrollConfiguration(
        behavior: ScrollConfiguration.of(context).copyWith(
          dragDevices: {
            PointerDeviceKind.touch,
            PointerDeviceKind.mouse,
          },
        ),
        child: ListView.builder(
          controller: scrollController,
          padding: const EdgeInsets.only(bottom: 80),
          itemCount: entities.length,
          itemBuilder: (context, index) {
            final entity = entities[index];
            final title = FileManager.basename(entity);
            final modified = entity.statSync().modified;
            final isSelected = selectedPaths.contains(entity.path);

            return GestureDetector(
              onSecondaryTap: () => state?.toggleSelection(entity.path),
              onLongPress: () => state?.toggleSelection(entity.path),
              child: Container(
                color: isSelected ? Colors.grey[900] : Colors.transparent,
                child: ListTile(
                  minTileHeight: 65,
                  leading: Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      if (isSelectionMode)
                        Padding(
                          padding: const EdgeInsets.only(right: 16),
                          child: CustomCircleCheckbox(
                            isChecked: isSelected,
                            onTap: () => state?.toggleSelection(entity.path),
                          ),
                        ),
                      Container(
                        width: 40,
                        height: 40,
                        padding: const EdgeInsets.all(6),
                        child: Center(
                          child: Image.asset(
                            entity.iconPath,
                            fit: BoxFit.contain,
                            width: 28,
                            height: 28,
                          ),
                        ),
                      ),
                    ],
                  ),
                  title: Text(
                    title,
                    overflow: TextOverflow.ellipsis,
                    maxLines: 1,
                    style: TextStyle(
                      fontSize: 18,
                      color: const Color(0xFFD2D2D2),
                      fontWeight: FontWeight.w400,
                      fontFamily: Theme.of(context)
                          .extension<FilesTheme>()!
                          .defaultFontFamily,
                    ),
                  ),
                  trailing: Text(
                    formatModifiedTime(modified),
                    style: TextStyle(
                      fontSize: 16,
                      color: const Color(0xFF717171),
                      fontWeight: FontWeight.w400,
                      fontFamily: Theme.of(context)
                          .extension<FilesTheme>()!
                          .defaultFontFamily,
                    ),
                  ),
                  onTap: () {
                    if (isSelectionMode) {
                      // Select / unselect item instead of opening
                      state?.toggleSelection(entity.path);
                      return;
                    }

                    if (isSearching) {
                      state?.clearSearch();
                    }

                    if (FileManager.isDirectory(entity)) {
                      controller.openDirectory(entity);
                      scrollController.jumpTo(0);
                    } else {
                      handleFileTap(
                        context,
                        entity,
                        entity.path,
                        isSelectionMode,
                        state,
                        controller,
                      );
                    }
                  },
                ),
              ),
            );
          },
        ),
      );
    },
  );
}

Widget buildListViewForRecentFiles(
  BuildContext context,
  List<io.FileSystemEntity> fileSystemList,
) {
  final state = context.findAncestorStateOfType<FileExplorerPageState>();
  final isSelectionMode = state?.selectionMode ?? false;
  final selectedPaths = state?.selectedPaths ?? {};

  // Create a list of tuples (fullPath, fileItem)
  final files = fileSystemList.map((entity) {
    final fullPath = entity.path;
    final name = p.basename(fullPath);
    final accessed = entity.statSync().accessed;
    final fileItem = FileItem(
        name: name,
        type: p.extension(fullPath).toLowerCase(),
        children: [],
        modified: accessed);
    return MapEntry(fullPath, fileItem);
  }).toList();

  return ScrollConfiguration(
    behavior: ScrollConfiguration.of(context).copyWith(
      dragDevices: {
        PointerDeviceKind.touch,
        PointerDeviceKind.mouse,
      },
    ),
    child: ListView.builder(
      padding: const EdgeInsets.only(bottom: 80),
      itemCount: files.length,
      itemBuilder: (context, index) {
        final entry = files[index];
        final fullPath = entry.key;
        final file = entry.value;
        final isSelected = selectedPaths.contains(fullPath);

        return GestureDetector(
          onSecondaryTap: () => state?.toggleSelection(fullPath),
          onLongPress: () => state?.toggleSelection(fullPath),
          child: ListTile(
            minTileHeight: 65,
            leading: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                if (isSelectionMode)
                  Padding(
                    padding: const EdgeInsets.only(right: 16),
                    child: CustomCircleCheckbox(
                      isChecked: isSelected,
                      onTap: () => state?.toggleSelection(fullPath),
                    ),
                  ),
                Container(
                  width: 40,
                  height: 40,
                  padding: const EdgeInsets.all(6),
                  child: Center(
                    child: Image.asset(
                      file.iconPath,
                      fit: BoxFit.contain,
                      width: 28,
                      height: 28,
                    ),
                  ),
                ),
              ],
            ),
            title: Text(file.name),
            trailing: file.modified != null
                ? Text(
                    formatModifiedTime(file.modified!),
                    style: const TextStyle(
                      color: Colors.grey,
                      fontSize: 12,
                    ),
                  )
                : null,
            onTap: () {
              handleTap(
                context,
                file,
                [], // no currentPath for recent
                fullPath, // ✅ correct path for this file
                isSelectionMode,
                state,
              );
            },
          ),
        );
      },
    ),
  );
}

Widget buildListViewMove(
  List<io.FileSystemEntity> foldersList,
  BuildContext context,
  String currentPath,
  FilesBloc filesBloc,
  VoidCallback onMoveCompleted,
  ScrollController scrollController,
) {
  return ScrollConfiguration(
    behavior: ScrollConfiguration.of(context).copyWith(
      dragDevices: {
        PointerDeviceKind.touch,
        PointerDeviceKind.mouse,
      },
    ),
    child: ListView.separated(
      controller: scrollController,
      itemCount: foldersList.length,
      separatorBuilder: (context, index) => Divider(
        height: 1,
        thickness: 1,
        color: Colors.grey[800],
      ),
      itemBuilder: (context, index) {
        final file = foldersList[index];
        final folderName = getCurrentFolderName(file.path);
        final newPath = '$currentPath/$folderName';

        return Container(
          color: Colors.grey[850],
          child: ListTile(
            contentPadding: const EdgeInsets.symmetric(horizontal: 12),
            leading: Image.asset(
              file.iconPath,
              width: 24,
              height: 24,
              fit: BoxFit.contain,
            ),
            title: Text(
              folderName,
              style: const TextStyle(fontSize: 14, color: Colors.white),
            ),
            trailing: trailingIcon(),
            onTap: () {
              onTap(context, newPath, folderName, filesBloc, onMoveCompleted);
              scrollController.jumpTo(0);
            },
          ),
        );
      },
    ),
  );
}

Widget buildSearchResultsList(
  List<FileSystemEntity> results,
  BuildContext context,
) {
  final displayedFiles = getFilesAtPath([], results);
  final state = context.findAncestorStateOfType<FileExplorerPageState>();
  final isSelectionMode = state?.selectionMode ?? false;
  final selectedPaths = state?.selectedPaths ?? {};

  return ScrollConfiguration(
    behavior: ScrollConfiguration.of(context).copyWith(
      dragDevices: {
        PointerDeviceKind.touch,
        PointerDeviceKind.mouse,
      },
    ),
    child: ListView.builder(
      padding: const EdgeInsets.only(bottom: 80),
      itemCount: results.length,
      itemBuilder: (context, index) {
        final entity = results[index];
        final file = displayedFiles[index];
        final fullPath = entity.path;
        final name = p.basename(fullPath);
        final isDir = entity is Directory;
        final isSelected = selectedPaths.contains(fullPath);

        return GestureDetector(
          onSecondaryTap: () => state?.toggleSelection(fullPath),
          onLongPress: () => state?.toggleSelection(fullPath),
          child: ListTile(
            minVerticalPadding: 12,
            leading: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                if (isSelectionMode)
                  Padding(
                    padding: const EdgeInsets.only(right: 16),
                    child: CustomCircleCheckbox(
                      isChecked: isSelected,
                      onTap: () => state?.toggleSelection(fullPath),
                    ),
                  ),
                Container(
                  width: 40,
                  height: 40,
                  padding: const EdgeInsets.all(6),
                  child: Image.asset(
                    file.iconPath,
                    fit: BoxFit.contain,
                    width: 28,
                    height: 28,
                  ),
                ),
              ],
            ),
            title: Text(
              name,
              overflow: TextOverflow.ellipsis,
              maxLines: 1,
              style: Theme.of(context).textTheme.bodyMedium,
            ),
            subtitle: Text(
              fullPath,
              overflow: TextOverflow.ellipsis,
              maxLines: 1,
              style: const TextStyle(fontSize: 12, color: Colors.grey),
            ),
            onTap: () {
              handleTap(
                context,
                FileItem(name: entity.basename, type: isDir ? 'dir' : 'file'),
                pathToSegments(p.dirname(fullPath)),
                fullPath,
                isSelectionMode,
                state,
              );
            },
          ),
        );
      },
    ),
  );
}

Widget buildListViewExtract(
  List<io.FileSystemEntity> foldersList,
  BuildContext context,
  String currentPath,
  FilesBloc filesBloc,
  VoidCallback onMoveCompleted,
  ScrollController scrollController,
) {
  return ScrollConfiguration(
    behavior: ScrollConfiguration.of(context).copyWith(
      dragDevices: {
        PointerDeviceKind.touch,
        PointerDeviceKind.mouse,
      },
    ),
    child: ListView.separated(
      controller: scrollController,
      itemCount: foldersList.length,
      separatorBuilder: (context, index) => Divider(
        height: 1,
        thickness: 1,
        color: Colors.grey[800],
      ),
      itemBuilder: (context, index) {
        final file = foldersList[index];
        final folderName = getCurrentFolderName(file.path);
        final newPath = '$currentPath/$folderName';

        return Container(
          color: Colors.grey[850],
          height: 50,
          child: ListTile(
            dense: true,
            contentPadding: const EdgeInsets.symmetric(horizontal: 12),
            leading: Image.asset(
              file.iconPath,
              width: 24,
              height: 24,
              fit: BoxFit.contain,
            ),
            title: Text(
              folderName,
              style: const TextStyle(fontSize: 14, color: Colors.white),
            ),
            trailing: trailingIcon(),
            onTap: () {
              onItemTap(
                context,
                newPath,
                folderName,
                filesBloc,
                onMoveCompleted,
              );
              scrollController.jumpTo(0);
            },
          ),
        );
      },
    ),
  );
}

Widget trailingIcon() {
  return SizedBox(
    child: const Icon(
      size: 14,
      Icons.arrow_forward_ios,
      color: Colors.grey,
    ).padAll(4),
  );
}
