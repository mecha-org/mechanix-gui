import 'package:flutter/material.dart';
import 'package:widgets/mechanix.dart';

Widget buildLoadingDialog(BuildContext context, String message) {
  return AlertDialog(
    content: Row(
      children: [
        CircularProgressIndicator(color: context.colorScheme.primaryFixed),
        const SizedBox(width: 20),
        Text(message),
      ],
    ),
  );
}
