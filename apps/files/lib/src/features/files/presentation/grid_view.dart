import 'package:file/file.dart';
import 'package:flutter/material.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_circular_checkbox.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'files.dart';
import 'package:path/path.dart' as p;

Widget buildGridView(
  List<FileItem> files,
  BuildContext context,
  List<FileItem> currentPath,
) {
  final state = context.findAncestorStateOfType<FileExplorerPageState>();
  final isSelectionMode = state?.selectionMode ?? false;
  final selectedPaths = state?.selectedPaths ?? {};

  final screenWidth = MediaQuery.of(context).size.width;
  final crossAxisCount =
      (screenWidth ~/ 120).clamp(2, 15); // responsive columns

  return GridView.builder(
    padding: const EdgeInsets.all(20),
    gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
      crossAxisCount: crossAxisCount,
      crossAxisSpacing: 30,
      mainAxisSpacing: 20,
      childAspectRatio: 0.8, // slightly taller to fit name
    ),
    itemCount: files.length,
    itemBuilder: (context, index) {
      final file = files[index];
      final fullPath =
          '/${[...currentPath.map((e) => e.name), file.name].join('/')}';
      final isSelected = selectedPaths.contains(fullPath);

      return LayoutBuilder(
        builder: (context, constraints) {
          final size = constraints.maxWidth;

          return GestureDetector(
            onTap: () {
              handleTap(
                context,
                file,
                currentPath,
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
                  height: size, // make it square
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
  );
}

Widget buildGridViewForRecentFiles(
  BuildContext context,
  List<FileSystemEntity> fileSystemList,
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

  return GridView.builder(
    padding: const EdgeInsets.all(20),
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
          final size = constraints.maxWidth;

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
  );
}
