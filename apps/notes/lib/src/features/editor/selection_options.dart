import 'package:flutter/material.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:widgets/mechanix.dart';

class SelectionOptions extends StatelessWidget {
  final VoidCallback onCopy;
  final VoidCallback onCut;
  final VoidCallback onPaste;
  final VoidCallback onSelectAll;
  final VoidCallback onDelete;

  const SelectionOptions({
    super.key,
    required this.onCopy,
    required this.onCut,
    required this.onPaste,
    required this.onSelectAll,
    required this.onDelete,
  });

  @override
  Widget build(BuildContext context) {
    return TextFieldTapRegion(
      child: Container(
        height: 44,
        padding: const EdgeInsets.symmetric(horizontal: 4, vertical: 4),
        decoration: BoxDecoration(
          color: context.secondaryContainer,
          borderRadius: BorderRadius.circular(8),
          boxShadow: const [
            BoxShadow(
              offset: Offset(0, 0),
              blurRadius: 8,
              spreadRadius: 0,
              color: Color(0x99000000),
            ),
            BoxShadow(
              offset: Offset(0, 1),
              blurRadius: 5,
              spreadRadius: 0,
              color: Color(0x26424242),
            ),
            BoxShadow(
              offset: Offset(0, -1),
              blurRadius: 5,
              spreadRadius: 0,
              color: Color(0x40424242),
            ),
            BoxShadow(
              offset: Offset(0, 4),
              blurRadius: 4,
              spreadRadius: 0,
              color: Color(0x40000000),
            ),
          ],
        ),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.center,
          spacing: 5,
          children: [
            _buildIconButton(icon: NotesIcon.copyIcon, onPressed: onCopy),
            _buildIconButton(icon: NotesIcon.cutIcon, onPressed: onCut),
            _buildIconButton(icon: NotesIcon.pasteIcon, onPressed: onPaste),
            _buildIconButton(
              icon: NotesIcon.selectAllOptionIcon,
              onPressed: onSelectAll,
            ),
            _buildIconButton(icon: NotesIcon.deleteIcon, onPressed: onDelete),
          ],
        ),
      ),
    );
  }

  Widget _buildIconButton({
    required String icon,
    required VoidCallback onPressed,
  }) {
    return SizedBox(
      width: 36,
      height: 36,
      child: IconButton(
        padding: EdgeInsets.zero,
        iconSize: 24,
        constraints: const BoxConstraints(),
        splashRadius: 18,
        onPressed: onPressed,
        icon: IconWidget(
          iconPath: icon,
          iconHeight: 24,
          iconWidth: 24,
          boxHeight: 24,
          boxWidth: 24,
        ),
      ),
    );
  }
}
