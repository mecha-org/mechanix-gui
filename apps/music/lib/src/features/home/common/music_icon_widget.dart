import 'package:flutter/material.dart';
import 'package:mechanix_music/src/commons/colors.dart';

class MusicIconButton extends StatelessWidget {
  final VoidCallback? onPressed;

  // Icon
  final String icon;
  final double iconSize;

  // Button sizing
  final double buttonSize;
  final EdgeInsetsGeometry padding;

  // Styling
  final Color backgroundColor;
  final BorderRadius borderRadius;

  // States
  final bool enabled;

  const MusicIconButton({
    super.key,
    required this.icon,
    this.onPressed,
    this.iconSize = 28,
    this.buttonSize = 44,
    this.padding = EdgeInsets.zero,
    this.backgroundColor = MusicColors.buttonBackgroundColor,
    this.borderRadius = const BorderRadius.all(Radius.circular(8)),
    this.enabled = true,
  });

  @override
  Widget build(BuildContext context) {
    return IconButton(
      onPressed: enabled ? onPressed : null,
      iconSize: iconSize,
      padding: padding,
      hoverColor: Colors.transparent,
      splashColor: Colors.transparent,
      style: ButtonStyle(
        fixedSize: WidgetStatePropertyAll(Size(buttonSize, buttonSize)),
        backgroundColor: WidgetStatePropertyAll(backgroundColor),
        shape: WidgetStatePropertyAll(
          RoundedRectangleBorder(borderRadius: borderRadius),
        ),
      ),
      icon: Image.asset(icon, width: iconSize, height: iconSize),
    );
  }
}
