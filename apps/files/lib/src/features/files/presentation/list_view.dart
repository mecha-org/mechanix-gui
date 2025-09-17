import 'package:file/file.dart';
import 'package:flutter/material.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_circular_checkbox.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'files.dart';
import 'package:path/path.dart' as p;

Widget buildListView(
  List<FileItem> files,
  BuildContext context,
  List<FileItem> currentPath,
) {
  final state = context.findAncestorStateOfType<FileExplorerPageState>();
  final isSelectionMode = state?.selectionMode ?? false;
  final selectedPaths = state?.selectedPaths ?? {};

  return ListView.builder(
    // padding: const EdgeInsets.all(8),
    itemCount: files.length,
    itemBuilder: (context, index) {
      final file = files[index];
      final fullPath =
          '/${[...currentPath.map((e) => e.name), file.name].join('/')}';
      final isSelected = selectedPaths.contains(fullPath);

      return GestureDetector(
        onSecondaryTap: () => state?.toggleSelection(fullPath), // right-click
        onLongPress: () =>
            state?.toggleSelection(fullPath), // long press (mobile)
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
                width: 60,
                height: 60,
                padding: const EdgeInsets.all(6),
                decoration: BoxDecoration(
                  color: Colors.grey.shade900,
                  borderRadius: BorderRadius.circular(8),
                ),
                child: Center(
                    child: Image.asset(file.iconPath, fit: BoxFit.contain)),
              ),
            ],
          ),
          title: Row(
            children: [
              Flexible(
                child: Text(
                  file.name,
                  overflow: TextOverflow.ellipsis, // Prevent overflow
                  maxLines: 1, // Single-line truncation
                  style: Theme.of(context).textTheme.bodyMedium,
                ),
              ),
            ],
          ),
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
              currentPath,
              fullPath,
              isSelectionMode,
              state,
            );
          },
        ),
      );
    },
  );
}

Widget buildListViewForRecentFiles(
  BuildContext context,
  List<FileSystemEntity> fileSystemList,
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

  return ListView.builder(
    padding: const EdgeInsets.all(8),
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
                width: 60,
                height: 60,
                padding: const EdgeInsets.all(6),
                decoration: BoxDecoration(
                  color: Colors.grey.shade900,
                  borderRadius: BorderRadius.circular(8),
                ),
                child: Center(
                  child: Image.asset(file.iconPath, fit: BoxFit.contain),
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
  );
}
