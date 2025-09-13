import 'package:flutter/material.dart';

class TitleEdit extends StatelessWidget {
  final TextEditingController controller;
  final FocusNode focusNode;

  const TitleEdit({
    super.key,
    required this.controller,
    required this.focusNode,
  });

  @override
  Widget build(BuildContext context) {
    final isDark = Theme.of(context).brightness == Brightness.dark;

    return TextField(
      controller: controller,
      focusNode: focusNode,
      decoration: InputDecoration(
        hintText: 'Enter title...',
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(12),
        ),
        filled: true,
        fillColor: isDark ? Colors.grey[800] : Colors.grey[300],
      ),
      style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
    );
  }
}
