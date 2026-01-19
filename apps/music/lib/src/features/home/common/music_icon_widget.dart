import 'package:flutter/material.dart';
import 'package:widgets/mechanix.dart';

class MusicIconButton extends StatelessWidget {
  final VoidCallback? onPressed;

  // Icon
  final String icon;
  final double iconSize;

  // Button sizing
  final double buttonSize;
  final EdgeInsetsGeometry padding;

  // Styling
  final Color? backgroundColor;
  final BorderRadius borderRadius;
  final Color? iconColor;

  // States
  final bool enabled;

  const MusicIconButton({
    super.key,
    required this.icon,
    this.onPressed,
    this.iconSize = 28,
    this.buttonSize = 44,
    this.padding = EdgeInsets.zero,
    this.backgroundColor,
    this.borderRadius = const BorderRadius.all(Radius.circular(8)),
    this.enabled = true,
    this.iconColor,
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
        backgroundColor: WidgetStatePropertyAll(
          backgroundColor ?? context.secondary,
        ),
        shape: WidgetStatePropertyAll(
          RoundedRectangleBorder(borderRadius: borderRadius),
        ),
      ),
      icon: IconWidget(
        iconPath: icon,
        iconWidth: iconSize,
        iconHeight: iconSize,
        iconColor: iconColor,
        boxWidth: iconSize,
        boxHeight: iconSize,
      ),
    );
  }
}
