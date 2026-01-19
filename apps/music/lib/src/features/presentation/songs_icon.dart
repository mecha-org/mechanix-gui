import 'package:flutter/material.dart';
import 'package:widgets/mechanix.dart';

class SongsIcon extends StatelessWidget {
  final String iconPath;
  final double boxSize;
  final double iconSize;
  final bool isActive;
  final Color? iconColor;
  const SongsIcon({
    super.key,
    required this.iconPath,
    this.boxSize = 24,
    this.iconSize = 24,
    this.isActive = false,
    this.iconColor
  });

  @override
  Widget build(BuildContext context) {
    return IconWidget(
      iconPath: iconPath,
      boxHeight: boxSize,
      boxWidth: boxSize,
      iconHeight: iconSize,
      iconWidth: iconSize,
      isActive: isActive,
      iconColor: iconColor,
    );
  }
}
