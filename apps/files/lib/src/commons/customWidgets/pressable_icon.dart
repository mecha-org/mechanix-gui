import 'package:flutter/material.dart';
import 'package:widgets/mechanix.dart';

class DecoratedPressableIcon extends StatefulWidget {
  final String? iconPath;
  final Widget? icon;
  final VoidCallback? onTap;
  final bool isDisabled;
  final Color? tapBackgroundColor;

  const DecoratedPressableIcon({
    super.key,
    this.iconPath,
    this.icon,
    this.onTap,
    this.isDisabled = false,
    this.tapBackgroundColor,
  });

  @override
  State<DecoratedPressableIcon> createState() => _DecoratedPressableIconState();
}

class _DecoratedPressableIconState extends State<DecoratedPressableIcon> {
  bool pressed = false;

  @override
  Widget build(BuildContext context) {
    final disabled = widget.isDisabled;
    final bgColor = widget.tapBackgroundColor ??
        context.colorScheme.surfaceContainerHigh.withAlpha(100);

    final Widget iconWidget = widget.icon ??
        IconWidget(
          iconPath: widget.iconPath!,
          iconHeight: 28,
          iconWidth: 28,
          boxWidth: 48,
          boxHeight: 48,
          iconColor: disabled
              ? context.colorScheme.outline
              : pressed
                  ? context.colorScheme.primaryContainer
                  : context.colorScheme.onSurface,
        );

    final Widget coloredIcon = widget.icon != null
        ? IconTheme(
            data: IconThemeData(
              size: 28,
              color: disabled
                  ? context.colorScheme.outline
                  : pressed
                      ? context.colorScheme.primaryContainer
                      : context.colorScheme.onSurface,
            ),
            child: SizedBox(
              width: 48,
              height: 48,
              child: Center(child: widget.icon),
            ),
          )
        : iconWidget;

    return GestureDetector(
      behavior: HitTestBehavior.opaque,
      onTapDown: disabled ? null : (_) => setState(() => pressed = true),
      onTapUp: disabled ? null : (_) => setState(() => pressed = false),
      onTapCancel: disabled ? null : () => setState(() => pressed = false),
      onTap: disabled ? null : widget.onTap,
      child: Container(
        decoration: pressed
            ? BoxDecoration(
                borderRadius: BorderRadius.circular(8),
                color: bgColor,
              )
            : null,
        child: coloredIcon,
      ),
    );
  }
}
