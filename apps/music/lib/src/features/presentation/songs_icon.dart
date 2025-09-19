import 'package:flutter/material.dart';

class SongsIcon extends StatelessWidget {
  final String iconPath;
  final Color color;
  final double height;
  final double width;

  const SongsIcon({
    super.key,
    required this.iconPath,
    this.color = Colors.white,
    this.height = 32,
    this.width = 32,
  });

  @override
  Widget build(BuildContext context) {
    return Image.asset(iconPath, width: 44, height: 44, color: color);
  }
}
