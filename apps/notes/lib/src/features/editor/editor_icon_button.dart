import 'package:flutter/material.dart';
import 'package:widgets/mechanix.dart';

class EditorIconButton extends StatelessWidget {
  final bool isSelected;
  final VoidCallback onPressed;
  final Border? border;
  final Widget? icon;
  final String iconPath;

  const EditorIconButton({
    super.key,
    required this.isSelected,
    required this.onPressed,
    this.icon,
    this.border,
    this.iconPath = '',
  });
  @override
  Widget build(BuildContext context) {
    return Container(
      width: 32,
      height: 32,
      // margin: border != null ? null : const EdgeInsets.all(4),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(4),
        // border: border,
      ),
      child: IconButton(
        onPressed: onPressed,
        icon:
            iconPath.isNotEmpty
                ? IconWidget(
                  iconPath: iconPath,
                  iconHeight: 24,
                  iconWidth: 24,
                  boxHeight: 24,
                  boxWidth: 24,
                  isActive: isSelected,
                  activeIconColor: context.primaryContainer,
                  // iconColor:
                  //     isSelected
                  //         ? NotesColors.secondaryTextColor
                )
                : icon!,
        padding: EdgeInsets.zero,
        style: ButtonStyle(
          // fixedSize: WidgetStateProperty<Size>.resolveWith( Size),
          fixedSize: WidgetStateProperty.resolveWith<Size?>(
            (states) => const Size(20, 20),
          ),
        ),

        splashRadius: 24,
        iconSize: 24,
      ),
    );
  }
}
