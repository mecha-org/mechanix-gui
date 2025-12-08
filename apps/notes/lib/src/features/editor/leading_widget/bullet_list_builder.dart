import 'package:flutter/material.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';

class BulletListBuilder extends StatelessWidget {
  const BulletListBuilder({super.key});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(right: 0, top: 12),
      child: Container(
        width: 4,
        height: 4,
        decoration: const BoxDecoration(
          color: NotesColors.editorTextColor,
          shape: BoxShape.circle,
        ),
      ),
    );
  }
}
