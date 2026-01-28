import 'package:flutter/material.dart';
import 'package:widgets/mechanix.dart';

class PressableIcon extends StatefulWidget {
  final VoidCallback? onTap;
  final String iconPath;
  final bool isDisabled;

  const PressableIcon({
    super.key,
    required this.iconPath,
    this.onTap,
    this.isDisabled = false,
  });

  @override
  State<PressableIcon> createState() => _PressableIconState();
}

class _PressableIconState extends State<PressableIcon> {
  bool pressed = false;

  @override
  Widget build(BuildContext context) {
    final bool disabled = widget.isDisabled;

    return GestureDetector(
      behavior: HitTestBehavior.opaque,
      onTapDown: disabled ? null : (_) => setState(() => pressed = true),
      onTapUp: disabled ? null : (_) => setState(() => pressed = false),
      onTapCancel: disabled ? null : () => setState(() => pressed = false),
      onTap: disabled ? null : widget.onTap,
      child: IconWidget(
        iconPath: widget.iconPath,
        iconHeight: 26,
        iconWidth: 26,
        iconColor: disabled
            ? context.colorScheme.outline
            : pressed
                ? context.colorScheme.primaryContainer
                : context.colorScheme.onSurface,
      ),
    );
  }
}

class DecoratedPressableIcon extends StatefulWidget {
  final String iconPath;
  final VoidCallback? onTap;
  final bool isDisabled;

  const DecoratedPressableIcon({
    super.key,
    required this.iconPath,
    this.onTap,
    this.isDisabled = false,
  });

  @override
  State<DecoratedPressableIcon> createState() => _DecoratedPressableIconState();
}

class _DecoratedPressableIconState extends State<DecoratedPressableIcon> {
  bool pressed = false;

  @override
  Widget build(BuildContext context) {
    final bool disabled = widget.isDisabled;

    return GestureDetector(
      behavior: HitTestBehavior.opaque,
      onTapDown: disabled ? null : (_) => setState(() => pressed = true),
      onTapUp: disabled ? null : (_) => setState(() => pressed = false),
      onTapCancel: disabled ? null : () => setState(() => pressed = false),
      onTap: disabled ? null : widget.onTap,
      child: Container(
        decoration: pressed
            ? BoxDecoration(
                borderRadius: const BorderRadius.all(Radius.circular(8)),
                color: context.colorScheme.secondary,
              )
            : null,
        padding: const EdgeInsets.all(12),
        child: IconWidget(
          iconPath: widget.iconPath,
          iconHeight: 28,
          iconWidth: 28,
          iconColor: disabled
              ? context.colorScheme.outline
              : pressed
                  ? context.colorScheme.primaryContainer
                  : context.colorScheme.onSurface,
        ),
      ),
    );
  }
}
