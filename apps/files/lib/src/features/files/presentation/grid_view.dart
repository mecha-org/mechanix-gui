import 'dart:io' as io;
import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_circular_checkbox.dart';
import 'package:mechanix_files/src/commons/customWidgets/middle_ellipsis_text.dart';
import 'package:mechanix_files/src/controllers/file_manager_controller.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:widgets/mechanix.dart';
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
  final isSearching = state?.isSearching ?? false;

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
            isSearching ? "No results found" : "Folder is empty",
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
                                        color: context.colorScheme.primaryFixed,
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
                                  color: context.colorScheme.primary,
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
                          child: MiddleEllipsisText(
                            title,
                            textAlign: TextAlign.center,
                            style: TextStyle(
                              fontSize: 16,
                              color: context.colorScheme.onSurface,
                              fontWeight: FontWeight.w400,
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
                            color: context.colorScheme.primary,
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
                    child: MiddleEllipsisText(
                      file.name,
                      style: TextStyle(
                        fontSize: 18,
                        color: context.colorScheme.onSurface,
                        fontWeight: FontWeight.w400,
                      ),
                      textAlign: TextAlign.center,
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
