import 'dart:io' as io;
import 'dart:ui';

import 'package:ellipsized_text/ellipsized_text.dart';
import 'package:flutter/material.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_circular_checkbox.dart';
import 'package:mechanix_files/src/controllers/file_manager.dart';
import 'package:mechanix_files/src/controllers/file_manager_controller.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:mechanix_files/src/features/files/presentation/recent_files.dart';
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
  ScrollController _scrollController;

  return ValueListenableBuilder<List<io.FileSystemEntity>>(
    valueListenable: controller.paginatedEntities,
    builder: (context, entities, _) {
      if (entities.isEmpty) {
        // Show message if folder is empty
        return Center(
          child: Text(
            isSearching ? "No results found" : "No items yet",
            style: Theme.of(context).textTheme.bodyLarge?.copyWith(
              color: context.colorScheme.onSecondaryFixed,
            ),
          ),
        );
      }

      // Move newly created folder to top
      if (controller.newFolderPath != null) {
        entities.sort((a, b) {
          if (a.path == controller.newFolderPath) return -1;
          if (b.path == controller.newFolderPath) return 1;
          return 0;
        });
      }

      return ScrollConfiguration(
        behavior: ScrollConfiguration.of(context).copyWith(
          dragDevices: {PointerDeviceKind.touch, PointerDeviceKind.mouse},
        ),
        child: ListView.builder(
          controller: scrollController,
          padding: const EdgeInsets.only(bottom: 80),
          itemCount: entities.length,
          itemBuilder: (context, index) {
            final entity = entities[index];
            final title = controller.getDisplayName(entity);

            final modified = entity.statSync().modified;
            final isSelected = selectedPaths.contains(entity.path);
            final isNew = entity.path == controller.newFolderPath;
            return Padding(
              padding: const EdgeInsets.symmetric(
                vertical: 1,
              ), // spacing between items
              child: GestureDetector(
                onSecondaryTap: () => state?.toggleSelection(entity.path),
                onLongPress: () => state?.toggleSelection(entity.path),
                child: Container(
                  decoration: BoxDecoration(
                    color:
                        isNew
                            ? context.colorScheme.secondaryContainer
                            : isSelected
                            ? context.colorScheme.secondaryContainer
                            : Colors.transparent,
                  ),
                  child: ListTile(
                    minTileHeight: 36,
                    contentPadding: const EdgeInsets.only(
                      bottom: 8,
                      top: 8,
                      left: 16,
                      right: 16,
                    ),
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
                          width: 36,
                          height: 36,
                          padding: const EdgeInsets.all(6),
                          child: Center(
                            child: Image.asset(
                              entity.iconPath,
                              fit: BoxFit.contain,
                              width: 24,
                              height: 24,
                              color: context.colorScheme.primaryContainer,
                            ),
                          ),
                        ),
                      ],
                    ),
                    title: EllipsizedText(
                      title,
                      type: EllipsisType.middle,
                      style: TextStyle(
                        fontSize: 20,
                        color: context.colorScheme.onSurface,
                        fontWeight: FontWeight.w400,
                      ),
                    ),
                    trailing: Text(
                      formatModifiedTime(modified),
                      style: TextStyle(
                        fontSize: 18,
                        color: context.colorScheme.onSecondaryFixed,
                        fontWeight: FontWeight.w400,
                      ),
                    ),
                    onTap: () async {
                      if (isSelectionMode) {
                        state?.toggleSelection(entity.path);
                        return;
                      }

                      if (FileManager.isDirectory(entity)) {
                        await controller.openDirectory(entity);
                        scrollController.jumpTo(0);

                        if (isSearching) {
                          state?.clearSearch();
                        }
                      } else {
                        handleFileTap(
                          context,
                          entity,
                          entity.path,
                          isSelectionMode,
                          state,
                          controller,
                        );

                        if (isSearching) {
                          state?.clearSearch();
                        }
                      }
                    },
                  ),
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
  final state = context.findAncestorStateOfType<RecentFilesPageState>();
  final isSelectionMode = state?.selectionMode ?? false;
  final selectedPaths = state?.selectedPaths ?? {};

  // Create a list of tuples (fullPath, fileItem)
  final files =
      fileSystemList.map((entity) {
        final fullPath = entity.path;
        final name = p.basename(fullPath);
        final accessed = entity.statSync().accessed;
        final fileItem = FileItem(
          name: name,
          type: p.extension(fullPath).toLowerCase(),
          children: [],
          modified: accessed,
        );
        return MapEntry(fullPath, fileItem);
      }).toList();

  return ScrollConfiguration(
    behavior: ScrollConfiguration.of(
      context,
    ).copyWith(dragDevices: {PointerDeviceKind.touch, PointerDeviceKind.mouse}),
    child: ListView.builder(
      padding: const EdgeInsets.only(bottom: 80),
      itemCount: files.length,
      itemBuilder: (context, index) {
        final entry = files[index];
        final fullPath = entry.key;
        final file = entry.value;
        final isSelected = selectedPaths.contains(fullPath);
        return Padding(
          padding: const EdgeInsets.symmetric(vertical: 1),
          child: GestureDetector(
            onSecondaryTap: () => state?.toggleSelection(fullPath),
            onLongPress: () => state?.toggleSelection(fullPath),
            child: Container(
              decoration: BoxDecoration(
                color:
                    isSelected
                        ? context.colorScheme.secondaryContainer
                        : Colors.transparent,
              ),
              child: ListTile(
                minTileHeight: 36,
                contentPadding: const EdgeInsets.only(
                  bottom: 8,
                  top: 8,
                  left: 16,
                  right: 16,
                ),
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
                      width: 36,
                      height: 36,
                      padding: const EdgeInsets.all(6),
                      child: Center(
                        child: Image.asset(
                          file.iconPath,
                          fit: BoxFit.contain,
                          width: 24,
                          height: 24,
                          color: context.colorScheme.primaryContainer,
                        ),
                      ),
                    ),
                  ],
                ),
                title: EllipsizedText(
                  file.name,
                  type: EllipsisType.middle,
                  style: TextStyle(
                    fontSize: 20,
                    color: context.colorScheme.onSurface,
                    fontWeight: FontWeight.w400,
                  ),
                ),
                trailing:
                    file.modified != null
                        ? Text(
                          formatModifiedTime(file.modified!),
                          style: TextStyle(
                            fontSize: 18,
                            color: context.colorScheme.onSecondaryFixed,
                            fontWeight: FontWeight.w400,
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
            ),
          ),
        );
      },
    ),
  );
}

Widget buildListViewMoveAndExtract(
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
        return Padding(
          padding: const EdgeInsets.only(
            bottom: 8,
            top: 8,
            left: 16,
            right: 16,
          ), // space above Move bar
          child: Text(
            isSearching ? "No results found" : "No items yet",
            style: Theme.of(context).textTheme.bodyLarge?.copyWith(
              color: context.colorScheme.onSecondaryFixed,
            ),
          ),
        );
      }

      // Move newly created folder to top
      if (controller.newFolderPath != null) {
        entities.sort((a, b) {
          if (a.path == controller.newFolderPath) return -1;
          if (b.path == controller.newFolderPath) return 1;
          return 0;
        });
      }

      return ScrollConfiguration(
        behavior: ScrollConfiguration.of(context).copyWith(
          dragDevices: {PointerDeviceKind.touch, PointerDeviceKind.mouse},
        ),
        child: ListView.builder(
          controller: scrollController,
          padding: const EdgeInsets.only(bottom: 80),
          itemCount: entities.length,
          itemBuilder: (context, index) {
            final entity = entities[index];
            final title = controller.getDisplayName(entity);
            final modified = entity.statSync().modified;
            final isSelected = selectedPaths.contains(entity.path);
            final isNew = entity.path == controller.newFolderPath;

            final isDirectory = FileManager.isDirectory(entity);
            final isDisabled = !isDirectory; // disable if file

            return GestureDetector(
              onSecondaryTap:
                  !isDisabled
                      ? () => state?.toggleSelection(entity.path)
                      : null,
              onLongPress:
                  !isDisabled
                      ? () => state?.toggleSelection(entity.path)
                      : null,
              child: Opacity(
                opacity: isDisabled ? 0.4 : 1, // grey out
                child: IgnorePointer(
                  ignoring: isDisabled, // block interaction
                  child: Container(
                    decoration: BoxDecoration(
                      color:
                          isNew
                              ? context.colorScheme.surfaceContainerHighest
                              : Colors.transparent,
                    ),
                    child: ListTile(
                      minTileHeight: 36,
                      contentPadding: const EdgeInsets.only(
                        bottom: 8,
                        top: 8,
                        left: 16,
                        right: 16,
                      ),
                      leading: Row(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          if (isSelectionMode)
                            Padding(
                              padding: const EdgeInsets.only(right: 16),
                              child: CustomCircleCheckbox(
                                isChecked: isSelected,
                                onTap:
                                    () => state?.toggleSelection(entity.path),
                              ),
                            ),
                          Container(
                            width: 36,
                            height: 36,
                            padding: const EdgeInsets.all(6),
                            child: Center(
                              child: Image.asset(
                                entity.iconPath,
                                fit: BoxFit.contain,
                                width: 24,
                                height: 24,
                                color: context.colorScheme.primaryContainer,
                              ),
                            ),
                          ),
                        ],
                      ),
                      title: EllipsizedText(
                        title,
                        type: EllipsisType.middle,
                        style: TextStyle(
                          fontSize: 20,
                          color: context.colorScheme.onSurface,
                          fontWeight: FontWeight.w400,
                        ),
                      ),
                      trailing: Text(
                        formatModifiedTime(modified),
                        style: TextStyle(
                          fontSize: 18,
                          color: context.colorScheme.onSecondaryFixed,
                          fontWeight: FontWeight.w400,
                        ),
                      ),
                      onTap: () {
                        if (isSelectionMode) {
                          state?.toggleSelection(entity.path);
                          return;
                        }

                        if (isSearching) {
                          state?.clearSearch();
                        }

                        if (isDirectory) {
                          controller.openDirectory(entity);
                          scrollController.jumpTo(0);
                        }
                      },
                    ),
                  ),
                ),
              ),
            );
          },
        ),
      );
    },
  );
}
