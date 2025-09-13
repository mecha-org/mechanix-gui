import 'package:flutter/material.dart';
import 'package:mechanix_notes/src/features/editor/editor_icon_button.dart';

class ColorButton extends StatelessWidget {
  final Color color;
  final VoidCallback onPressed;
  final bool isSelected;
  const ColorButton({
    super.key,
    required this.color,
    required this.onPressed,
    required this.isSelected,
  });

  @override
  Widget build(BuildContext context) {
    return EditorIconButton(
      isSelected: isSelected,
      onPressed: onPressed,
      icon: Container(
        margin: EdgeInsets.all(4),
        width: 28,
        height: 28,
        decoration: BoxDecoration(
          color: color,
          borderRadius: BorderRadius.circular(4),
        ),
      ),
    );
  }
}
