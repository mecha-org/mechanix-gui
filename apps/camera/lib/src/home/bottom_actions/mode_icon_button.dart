import 'package:flutter/material.dart';
import 'package:widgets/widgets/icon_widget.dart';

/// Reusable icon button used in capture mode toggles and settings
class ModeIconButton extends StatelessWidget {
  final String iconPath;
  final bool isActive;
  final Color? backgroundColor;

  const ModeIconButton({
    super.key,
    required this.iconPath,
    this.isActive = false,
    this.backgroundColor,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: isActive ? backgroundColor : null,
        borderRadius: const BorderRadius.all(Radius.circular(6)),
      ),
      padding: EdgeInsets.zero,
      child: IconWidget(
        iconPath: iconPath,
        boxWidth: 40,
        boxHeight: 40,
        iconWidth: 20,
        iconHeight: 20,
      ),
    );
  }
}