import 'package:flutter/material.dart';
import 'package:mechanix_files/src/commons/styles/file_theme_extenstions.dart';

Widget buildLoadingDialog(BuildContext context, String message) {
  return AlertDialog(
    content: Row(
      children: [
        CircularProgressIndicator(
            color: Theme.of(context).extension<FilesTheme>()!.primaryColor),
        const SizedBox(width: 20),
        Text(message),
      ],
    ),
  );
}
