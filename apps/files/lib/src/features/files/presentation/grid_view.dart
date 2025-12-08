import 'dart:io' as io;
import 'dart:ui';

import 'package:file/file.dart';
import 'package:flutter/material.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_circular_checkbox.dart';
import 'package:mechanix_files/src/commons/styles/file_theme_extenstions.dart';
import 'package:mechanix_files/src/controllers/file_manager_controller.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:mechanix_files/src/features/files/presentation/files_home.dart';
import '../../../controllers/file_manager.dart';
import 'files.dart';
import 'package:path/path.dart' as p;

Widget buildGridView(
  BuildContext context,
  ScrollController scrollController,
  FileManagerController controller,
) {
  final state = context.findAncestorStateOfType<FileExplorerPageState>();
  final isSelectionMode = state?.selectionMode ?? false;
  final selectedPaths = state?.selectedPaths ?? {};

  final screenWidth = MediaQuery.of(context).size.width;
  final crossAxisCount =
      (screenWidth ~/ 120).clamp(2, 15); // responsive columns

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
        child: GridView.builder(
          controller: scrollController,
          padding:
              const EdgeInsets.only(left: 20, top: 20, right: 20, bottom: 80),
          gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
            crossAxisCount: crossAxisCount,
            crossAxisSpacing: 30,
            mainAxisSpacing: 20,
            childAspectRatio: 0.8, // slightly taller to fit name
          ),
          itemCount: entities.length,
          itemBuilder: (context, index) {
            final entity = entities[index];
            final title = FileManager.basename(entity);
            final fullPath = entity.path;
            final isSelected = selectedPaths.contains(fullPath);

            return LayoutBuilder(
              builder: (context, constraints) {
                final size = constraints.maxWidth * 0.8;

                return GestureDetector(
                    onTap: () async {
                      if (isSelectionMode) {
                        // Select / unselect instead of opening
                        state?.toggleSelection(fullPath);
                        return;
                      }

                      if (FileManager.isDirectory(entity)) {
                        await controller.openDirectory(entity);
                        scrollController.jumpTo(0);
                      } else {
                        handleFileTap(
                          context,
                          entity,
                          fullPath,
                          isSelectionMode,
                          state,
                          controller,
                        );
                      }
                    },
                    onLongPress: () => state?.toggleSelection(fullPath),
                    onSecondaryTap: () => state?.toggleSelection(fullPath),
                    child: Column(
                      children: [
                        Stack(
                          clipBehavior: Clip.none,
                          children: [
                            Container(
                              width: size,
                              height: size,
                              decoration: BoxDecoration(
                                color: Colors.grey.shade900,
                                borderRadius: BorderRadius.circular(14),
                                border: isSelected
                                    ? Border.all(
                                        color: Theme.of(context)
                                            .extension<FilesTheme>()!
                                            .primaryColor,
                                        width: 1,
                                      )
                                    : null,
                              ),
                              child: Center(
                                child: Image.asset(
                                  entity.iconPath,
                                  width: size * 0.5,
                                  height: size * 0.5,
                                  fit: BoxFit.contain,
                                ),
                              ),
                            ),
                            if (isSelectionMode)
                              Positioned(
                                left: -8,
                                bottom: -8,
                                child: CustomCircleCheckbox(
                                  isChecked: isSelected,
                                  onTap: () => state?.toggleSelection(fullPath),
                                ),
                              ),
                          ],
                        ),
                        const SizedBox(height: 4),
                        Flexible(
                          child: Text(
                            title,
                            textAlign: TextAlign.center,
                            maxLines: 1,
                            overflow: TextOverflow.ellipsis,
                            style: TextStyle(
                              fontSize: 18,
                              color: const Color(0xFFD2D2D2),
                              fontWeight: FontWeight.w400,
                              fontFamily: Theme.of(context)
                                  .extension<FilesTheme>()!
                                  .defaultFontFamily,
                            ),
                          ),
                        ),
                      ],
                    ));
              },
            );
          },
        ),
      );
    },
  );
}

Widget buildGridViewForRecentFiles(
  BuildContext context,
  List<io.FileSystemEntity> fileSystemList,
) {
  final state = context.findAncestorStateOfType<FileExplorerPageState>();
  final isSelectionMode = state?.selectionMode ?? false;
  final selectedPaths = state?.selectedPaths ?? {};

  final screenWidth = MediaQuery.of(context).size.width;
  final crossAxisCount = (screenWidth ~/ 120).clamp(2, 15);

  // Convert to entries (fullPath, FileItem)
  final files = fileSystemList.map((entity) {
    final fullPath = entity.path;
    final name = p.basename(fullPath);
    final fileItem = FileItem(
        name: name, type: p.extension(fullPath).toLowerCase(), children: []);
    return MapEntry(fullPath, fileItem);
  }).toList();

  return ScrollConfiguration(
    behavior: ScrollConfiguration.of(context).copyWith(
      dragDevices: {
        PointerDeviceKind.touch,
        PointerDeviceKind.mouse,
      },
    ),
    child: GridView.builder(
      padding: const EdgeInsets.only(left: 20, top: 20, right: 20, bottom: 80),
      gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
        crossAxisCount: crossAxisCount,
        crossAxisSpacing: 30,
        mainAxisSpacing: 20,
        childAspectRatio: 0.8,
      ),
      itemCount: files.length,
      itemBuilder: (context, index) {
        final entry = files[index];
        final fullPath = entry.key;
        final file = entry.value;
        final isSelected = selectedPaths.contains(fullPath);

        return LayoutBuilder(
          builder: (context, constraints) {
            final size = constraints.maxWidth * 0.8;

            return GestureDetector(
              onTap: () {
                handleTap(
                  context,
                  file,
                  [], // recent files have no path hierarchy
                  fullPath,
                  isSelectionMode,
                  state,
                );
              },
              onLongPress: () => state?.toggleSelection(fullPath),
              onSecondaryTap: () => state?.toggleSelection(fullPath),
              child: Column(
                children: [
                  Container(
                    width: size,
                    height: size,
                    decoration: BoxDecoration(
                      color: Colors.grey.shade900,
                      borderRadius: BorderRadius.circular(14),
                    ),
                    child: Stack(
                      children: [
                        Center(
                          child: Image.asset(
                            file.iconPath,
                            width: size * 0.5,
                            height: size * 0.5,
                            fit: BoxFit.contain,
                          ),
                        ),
                        if (isSelectionMode)
                          Positioned(
                            left: 0,
                            bottom: 0,
                            child: CustomCircleCheckbox(
                              isChecked: isSelected,
                              onTap: () => state?.toggleSelection(fullPath),
                            ),
                          ),
                      ],
                    ),
                  ),
                  const SizedBox(height: 6),
                  Flexible(
                    child: Text(
                      file.name,
                      style: const TextStyle(fontSize: 13, color: Colors.white),
                      textAlign: TextAlign.center,
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                    ),
                  ),
                ],
              ),
            );
          },
        );
      },
    ),
  );
}

Widget buildSearchResultsGrid(
  List<FileSystemEntity> results,
  BuildContext context,
) {
  final displayedFiles = getFilesAtPath([], results);
  final state = context.findAncestorStateOfType<FileExplorerPageState>();
  final isSelectionMode = state?.selectionMode ?? false;
  final selectedPaths = state?.selectedPaths ?? {};

  final screenWidth = MediaQuery.of(context).size.width;
  final crossAxisCount = (screenWidth ~/ 120).clamp(2, 15);

  return ScrollConfiguration(
    behavior: ScrollConfiguration.of(context).copyWith(
      dragDevices: {
        PointerDeviceKind.touch,
        PointerDeviceKind.mouse,
      },
    ),
    child: GridView.builder(
      padding: const EdgeInsets.only(left: 20, top: 20, right: 20, bottom: 80),
      gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
        crossAxisCount: crossAxisCount,
        crossAxisSpacing: 30,
        mainAxisSpacing: 20,
        childAspectRatio: 0.8,
      ),
      itemCount: results.length,
      itemBuilder: (context, index) {
        final entity = results[index];
        final file = displayedFiles[index];
        final fullPath = entity.path;
        final isSelected = selectedPaths.contains(fullPath);
        final isDir = entity is Directory;

        return LayoutBuilder(
          builder: (context, constraints) {
            final size = constraints.maxWidth * 0.8;

            return GestureDetector(
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
              onLongPress: () => state?.toggleSelection(fullPath),
              onSecondaryTap: () => state?.toggleSelection(fullPath),
              child: Column(
                children: [
                  Container(
                    width: size,
                    height: size,
                    decoration: BoxDecoration(
                      color: Colors.grey.shade900,
                      borderRadius: BorderRadius.circular(14),
                    ),
                    child: Stack(
                      children: [
                        Center(
                          child: Image.asset(
                            file.iconPath,
                            width: size * 0.5,
                            height: size * 0.5,
                            fit: BoxFit.contain,
                          ),
                        ),
                        if (isSelectionMode)
                          Positioned(
                            left: 0,
                            bottom: 0,
                            child: CustomCircleCheckbox(
                              isChecked: isSelected,
                              onTap: () => state?.toggleSelection(fullPath),
                            ),
                          ),
                      ],
                    ),
                  ),
                  const SizedBox(height: 4),
                  Flexible(
                    child: Text(
                      p.basename(fullPath),
                      style: const TextStyle(fontSize: 13, color: Colors.white),
                      textAlign: TextAlign.center,
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                    ),
                  ),
                ],
              ),
            );
          },
        );
      },
    ),
  );
}
