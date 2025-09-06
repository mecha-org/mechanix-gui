import 'package:flutter/material.dart';

Widget buildLoadingDialog(String message) {
  return AlertDialog(
    content: Row(
      children: [
        const CircularProgressIndicator(),
        const SizedBox(width: 20),
        Text(message),
      ],
    ),
  );
}
