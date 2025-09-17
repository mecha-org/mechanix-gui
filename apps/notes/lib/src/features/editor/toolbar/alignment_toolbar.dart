import 'package:flutter/material.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/commons/styles/styles.dart';
import 'package:mechanix_notes/src/features/editor/editor_icon_button.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/toolbar_container.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/toolbar_row.dart';

class AlignmentToolbar extends StatelessWidget {
  final QuillController controller;
  final OverlayEntry? entry;
  final FocusNode focusNode;
  final VoidCallback? onClose;

  const AlignmentToolbar({
    super.key,
    required this.controller,
    this.entry,
    required this.focusNode,
    this.onClose,
  });

  @override
  Widget build(BuildContext context) {
    void requestFocus() {
      if (!focusNode.hasFocus) {
        focusNode.requestFocus();
      }
    }

    void toggleList(Attribute attribute) {
      requestFocus();
      final selection = controller.selection;
      final attrs = controller.getSelectionStyle().attributes;
      final currentAttr = attrs[attribute.key];

      if (currentAttr != null && currentAttr.value == attribute.value) {
        controller.formatSelection(Attribute.clone(attribute, null));
      } else {
        controller.formatSelection(attribute);
      }
      controller.updateSelection(selection, ChangeSource.local);
    }

    void toggleIndent({bool increase = true}) {
      requestFocus();
      controller.indentSelection(increase);
    }

    bool isSelectionStyleApplied(
      Attribute attribute,
      String value, {
      bool isValue = true,
    }) {
      return isValue
          ? controller.getSelectionStyle().attributes[attribute.key]?.value ==
              value
          : controller.getSelectionStyle().attributes.containsKey(
            attribute.key,
          );
    }

    return ListenableBuilder(
      listenable: controller,
      builder: (context, child) {
        return ToolbarContainer(
          child: [
            ToolbarRow(
              child: [
                Expanded(
                  child: EditorIconButton(
                    isSelected: isSelectionStyleApplied(
                      Attribute.list,
                      'ordered',
                    ),
                    iconPath: NotesIcon.numberIcon,
                    onPressed: () => toggleList(Attribute.ol),
                  ),
                ),
                Expanded(
                  child: EditorIconButton(
                    isSelected: isSelectionStyleApplied(
                      Attribute.list,
                      'bullet',
                    ),
                    iconPath: NotesIcon.bulletIcon,
                    onPressed: () {
                      toggleList(Attribute.ul);
                    },
                  ),
                ),
                Expanded(
                  child: EditorIconButton(
                    isSelected: isSelectionStyleApplied(
                      Attribute.list,
                      'unchecked',
                    ),
                    iconPath: NotesIcon.checkboxListIcon,
                    onPressed: () {
                      toggleList(Attribute.unchecked);
                    },
                    border: Border(right: borderSideStyle),
                  ),
                ),
                Expanded(
                  child: Container(
                    decoration: BoxDecoration(
                      border: Border(
                        right: borderSideStyle,
                        left: borderSideStyle,
                      ),
                    ),
                    child: EditorIconButton(
                      isSelected: isSelectionStyleApplied(
                        Attribute.blockQuote,
                        '',
                        isValue: false,
                      ),
                      iconPath: NotesIcon.quoteIcon,
                      onPressed: () {
                        toggleList(Attribute.blockQuote);
                      },
                    ),
                  ),
                ),
                Expanded(
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
            ),
            ToolbarRow(
              isBorder: false,
              child: [
                Expanded(
                  child: EditorIconButton(
                    isSelected: false,
                    iconPath: NotesIcon.indentIncreaseIcon,
                    onPressed: () => toggleIndent(),
                  ),
                ),
                Expanded(
                  child: Container(
                    decoration: BoxDecoration(
                      border: Border(right: borderSideStyle),
                    ),
                    child: EditorIconButton(
                      isSelected: false,
                      iconPath: NotesIcon.indentDecreaseIcon,
                      onPressed: () => toggleIndent(increase: false),
                    ),
                  ),
                ),
                Expanded(
                  child: EditorIconButton(
                    isSelected: isSelectionStyleApplied(
                      Attribute.align,
                      'left',
                    ),
                    icon: Icon(Icons.format_align_left),
                    onPressed: () {
                      toggleList(AlignAttribute('left'));
                    },
                  ),
                ),
                Expanded(
                  child: EditorIconButton(
                    isSelected: isSelectionStyleApplied(
                      Attribute.align,
                      'center',
                    ),
                    iconPath: NotesIcon.centerAlignIcon,
                    onPressed: () {
                      toggleList(AlignAttribute('center'));
                    },
                  ),
                ),
                Expanded(
                  child: EditorIconButton(
                    isSelected: isSelectionStyleApplied(
                      Attribute.align,
                      'right',
                    ),
                    icon: Icon(Icons.format_align_right),
                    onPressed: () {
                      toggleList(AlignAttribute('right'));
                    },
                  ),
                ),
              ],
            ),
          ],
        );
      },
    );
  }
}
