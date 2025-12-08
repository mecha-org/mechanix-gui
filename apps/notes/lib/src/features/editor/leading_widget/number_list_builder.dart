import 'package:flutter/material.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';

class NumberListBuilder extends StatelessWidget {
  final String number;
  const NumberListBuilder({super.key, required this.number});

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 28,
      alignment: Alignment.center,
      padding: const EdgeInsets.only(top: 2, right: 0),
      child: Text(
        "$number.",
        style: const TextStyle(
          fontSize: 18,
          fontWeight: FontWeight.w400,
          color: NotesColors.titleTextColor,
          height: 1.45, // Word-like line spacing
          letterSpacing: 0.0,
        ),
      ),
    );
  }
}
