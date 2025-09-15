import 'package:flutter/material.dart';

class NotesFabIcon extends StatelessWidget {
  final String iconPath;
  final double iconSize;
  final Color? color;

  const NotesFabIcon({
    super.key,
    required this.iconPath,
    this.iconSize = 24,
    this.color,
  });

  @override
  Widget build(BuildContext context) {
    return Image.asset(
      iconPath,
      height: iconSize,
      width: iconSize,
      color: color,
    );
  }
}
