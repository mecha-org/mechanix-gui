import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'files.dart';

Widget buildNavigations(
  BuildContext context,
  List<FileItem> path, {
  required int selectedCount,
}) {
  final filesBloc = BlocProvider.of<FilesBloc>(context);
  final state = context.findAncestorStateOfType<FileExplorerPageState>();
  final isSelecting = state?.selectionMode ?? false;

  final List<Widget> navigationWidgets = [];

  // Back arrow
  if (!isSelecting && path.length > 1) {
    final backIndex = path.length - 2;
    final backPath = path.sublist(0, backIndex + 1);
    final backTitle = path[backIndex].name;

    navigationWidgets.add(
      GestureDetector(
        onTap: () {
          final fullPath = '/${backPath.map((e) => e.name).join('/')}';
          filesBloc.add(LoadFilesAtPath(fullPath.isEmpty ? '/' : fullPath));

          Navigator.pushReplacement(
            context,
            MaterialPageRoute(
              builder: (_) => BlocProvider.value(
                value: filesBloc,
                child: FileExplorerPage(
                  title: backTitle,
                  path: backPath,
                ),
              ),
            ),
          );
        },
        child: Row(
          children: [
            const SizedBox(width: 2),
          ],
        ),
      ),
    );
  }

  // Title text
  navigationWidgets.add(
    Flexible(
      child: Text(
        isSelecting
            ? "Select items"
            : (path.isNotEmpty ? path.last.name : 'Root'),
        overflow: TextOverflow.ellipsis,
        maxLines: 1,
        style: const TextStyle(
          fontWeight: FontWeight.bold,
          fontSize: 18,
        ),
      ),
    ),
  );

  return Padding(
    padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 4),
    child: Row(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        // Flexible left side
        Expanded(
          child: Row(
            children: navigationWidgets,
          ),
        ),
        // Right side: selection info
        if (isSelecting)
          Text(
            "$selectedCount item${selectedCount > 1 ? 's' : ''} selected",
            style: const TextStyle(
              color: Colors.grey,
              fontSize: 14,
            ),
          ),
      ],
    ),
  );
}
