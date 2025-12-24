import 'package:flutter/material.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/commons/styles/styles.dart';
import 'package:mechanix_notes/src/features/editor/editor_icon_button.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/focus_preserve_button.dart';

class AlignmentToolbar extends StatefulWidget {
  final QuillController controller;
  final OverlayEntry? entry;
  final FocusNode focusNode;
  final VoidCallback? onClose;

  const AlignmentToolbar({
    super.key,
    required this.controller,
    required this.focusNode,
    this.entry,
    this.onClose,
  });

  @override
  State<AlignmentToolbar> createState() => _AlignmentToolbarState();
}

class _AlignmentToolbarState extends State<AlignmentToolbar> {
  @override
  void initState() {
    super.initState();
  }

  @override
  void dispose() {
    super.dispose();
  }

  void requestFocus() {
    if (!widget.focusNode.hasFocus) {
      widget.focusNode.requestFocus();
    }
  }

  void toggleList(Attribute attribute) {
    requestFocus();
    final selection = widget.controller.selection;
    final attrs = widget.controller.getSelectionStyle().attributes;
    final currentAttr = attrs[attribute.key];
    if (currentAttr != null && currentAttr.value == attribute.value) {
      widget.controller.formatSelection(Attribute.clone(attribute, null));
    } else {
      widget.controller.formatSelection(attribute);
    }
    widget.controller.updateSelection(selection, ChangeSource.local);
  }

  bool isSelectionStyleApplied(
    Attribute attribute,
    String value, {
    bool isValue = true,
  }) {
    return isValue
        ? widget.controller
                .getSelectionStyle()
                .attributes[attribute.key]
                ?.value ==
            value
        : widget.controller.getSelectionStyle().attributes.containsKey(
          attribute.key,
        );
  }

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: widget.controller,
      builder: (context, child) {
        final isChecked =
            isSelectionStyleApplied(Attribute.list, 'unchecked') ||
            isSelectionStyleApplied(Attribute.list, 'checked');
        return Row(
          crossAxisAlignment: CrossAxisAlignment.center,
          spacing: 28,
          children: [
            FocusPreserveButton(
              child: EditorIconButton(
                isSelected: isSelectionStyleApplied(Attribute.list, 'bullet'),
                iconPath: NotesIcon.bulletIcon,
                onPressed: () {
                  toggleList(Attribute.ul);
                },
              ),
            ),

            FocusPreserveButton(
              child: EditorIconButton(
                isSelected: isSelectionStyleApplied(Attribute.list, 'ordered'),
                iconPath: NotesIcon.numberIcon,
                onPressed: () => toggleList(Attribute.ol),
              ),
            ),

            FocusPreserveButton(
              child: EditorIconButton(
                isSelected: isChecked,
                iconPath: NotesIcon.checkboxListIcon,
                onPressed: () {
                  toggleList(Attribute.unchecked);
                },
                border: const Border(right: borderSideStyle),
              ),
            ),

            FocusPreserveButton(
              child: EditorIconButton(
                isSelected: isSelectionStyleApplied(
                  Attribute.codeBlock,
                  '',
                  isValue: false,
                ),
                iconPath: NotesIcon.codeBlockIcon,
                onPressed: () {
                  toggleList(Attribute.codeBlock);
                },
              ),
            ),
          ],
        );
      },
    );
  }
}
