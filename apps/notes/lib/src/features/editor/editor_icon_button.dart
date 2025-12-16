import 'package:flutter/material.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';

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
                ? Image.asset(
                  iconPath,
                  height: 24,
                  width: 24,
                  color:
                      isSelected
                          ? NotesColors.secondaryTextColor
                          : Colors.white,
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
