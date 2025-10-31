import 'package:flutter/material.dart';

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
    return Center(
      child: Container(
        width: 40,
        height: 40,
        margin: border != null ? null : const EdgeInsets.all(4),
        decoration: BoxDecoration(
          color: isSelected ? const Color(0xFF48494B) : Colors.transparent,
          borderRadius: BorderRadius.circular(4),

          // border: border,
        ),
        child: IconButton(
          onPressed: onPressed,
          icon:
              iconPath.isNotEmpty
                  ? Image.asset(
                    iconPath,
                    height: 22,
                    width: 22,
                    color: Colors.white,
                  )
                  : icon!,
          padding: EdgeInsets.zero,
          style: ButtonStyle(
            // fixedSize: WidgetStateProperty<Size>.resolveWith( Size),
            fixedSize: WidgetStateProperty.resolveWith<Size?>(
              (states) => const Size(20, 20),
            ),
          ),

          splashRadius: 22,
          iconSize: 22,
          splashColor: Colors.transparent,
          highlightColor: Colors.transparent,
          hoverColor: Colors.transparent,
        ),
      ),
    );
  }
}
