import 'package:flutter/material.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';

Future<void> showCreateFolderDialog({
  required BuildContext context,
  required String currentPath,
  required FilesBloc filesBloc,
}) async {
  String folderName = '';

  await showDialog(
    context: context,
    builder: (_) {
      return AlertDialog(
        title: const Text('Create New Folder'),
        content: TextField(
          autofocus: true,
          decoration: const InputDecoration(hintText: 'Enter folder name'),
          onChanged: (value) => folderName = value,
          onSubmitted: (_) {
            if (folderName.trim().isNotEmpty) {
              filesBloc.add(CreateFolder(
                  path: currentPath, folderName: folderName.trim()));
              Navigator.of(context).pop();
            }
          },
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Cancel'),
          ),
          TextButton(
            onPressed: () {
              if (folderName.trim().isNotEmpty) {
                filesBloc.add(CreateFolder(
                    path: currentPath, folderName: folderName.trim()));
                Navigator.of(context).pop();
              }
            },
            child: const Text('Create'),
          ),
        ],
      );
    },
  );
}
