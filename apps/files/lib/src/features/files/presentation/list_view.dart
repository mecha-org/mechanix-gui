import 'dart:ui';

import 'package:file/file.dart';
import 'package:flutter/material.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_circular_checkbox.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:mechanix_files/src/features/files/presentation/extract_file_dialog.dart';
import 'package:mechanix_files/src/features/files/presentation/files_home.dart';
import 'package:mechanix_files/src/features/files/presentation/move_file_dialog.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/sectionList/mechanix_section_list_theme.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';
import 'files.dart';
import 'package:path/path.dart' as p;

Widget buildListView(
  List<FileItem> files,
  BuildContext context,
  List<FileItem> currentPath,
  ScrollController scrollController,
) {
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
      controller: scrollController,
      padding: const EdgeInsets.only(bottom: 80),
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
    ),
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
    ),
  );
}

Widget buildListViewMove(
    List<FileItem> files,
    BuildContext context,
    List<FileItem> currentPath,
    FilesBloc filesBloc,
    VoidCallback onMoveCompleted) {
  // Only folders
  final folders = files.where((file) => file.type == 'dir').toList();

  final sectionItems = folders.map((file) {
    return SectionListItems(
      title: file.name,
      titleTextStyle: const TextStyle(fontSize: 14),
      leading: Image.asset(file.iconPath,
          width: 24, height: 24, fit: BoxFit.contain),
      defaultTrailingIcon: false,
      trailing: trailingIcon(),
      onTap: () {
        final newPath = [...currentPath, file];
        final pathString = '/${newPath.map((e) => e.name).join('/')}';
        onTap(context, pathString, file.name, filesBloc, onMoveCompleted);
      },
    );
  }).toList();

  return ScrollConfiguration(
    behavior: ScrollConfiguration.of(context).copyWith(
      dragDevices: {
        PointerDeviceKind.touch,
        PointerDeviceKind.mouse,
      },
    ),
    child: MechanixSectionListTheme(
      style: MechanixSectionListThemeData(
        height: 50,
        dividerPadding: EdgeInsets.zero,
        widgetPadding: EdgeInsets.zero,
        backgroundColor: WidgetStateProperty.all(Colors.grey[850]),
      ),
      child: SingleChildScrollView(
        scrollDirection: Axis.vertical,
        child: MechanixSectionList(
          sectionListItems: sectionItems,
        ),
      ),
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
                  width: 60,
                  height: 60,
                  padding: const EdgeInsets.all(6),
                  decoration: BoxDecoration(
                    color: Colors.grey.shade900,
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: Image.asset(file.iconPath,
                      width: 24, height: 24, fit: BoxFit.contain),
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
    List<FileItem> files,
    BuildContext context,
    List<FileItem> currentPath,
    FilesBloc filesBloc,
    VoidCallback onExtractCompleted) {
  // Only folders
  final folders = files.where((file) => file.type == 'dir').toList();

  final sectionItems = folders.map((file) {
    return SectionListItems(
      title: file.name,
      titleTextStyle: const TextStyle(fontSize: 14),
      leading: Image.asset(file.iconPath,
          width: 24, height: 24, fit: BoxFit.contain),
      defaultTrailingIcon: false,
      trailing: trailingIcon(),
      onTap: () {
        final newPath = [...currentPath, file];
        final pathString = '/${newPath.map((e) => e.name).join('/')}';
        onItemTap(
            context, pathString, file.name, filesBloc, onExtractCompleted);
      },
    );
  }).toList();

  return ScrollConfiguration(
    behavior: ScrollConfiguration.of(context).copyWith(
      dragDevices: {
        PointerDeviceKind.touch,
        PointerDeviceKind.mouse,
      },
    ),
    child: MechanixSectionListTheme(
      style: MechanixSectionListThemeData(
        height: 50,
        dividerPadding: EdgeInsets.zero,
        widgetPadding: EdgeInsets.zero,
        backgroundColor: WidgetStateProperty.all(Colors.grey[850]),
      ),
      child: SingleChildScrollView(
        scrollDirection: Axis.vertical,
        child: MechanixSectionList(
          sectionListItems: sectionItems,
        ),
      ),
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
