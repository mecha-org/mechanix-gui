import 'package:flutter/material.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';
import 'package:mechanix_notes/src/commons/styles/styles.dart';
import 'package:mechanix_notes/src/constants/constants.dart';
import 'package:mechanix_notes/src/features/editor/editor_icon_button.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/color_button.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/toolbar_container.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/toolbar_row.dart';
import 'package:widgets/mechanix.dart';

class TextEditorToolbar extends StatefulWidget {
  final QuillController controller;
  final OverlayEntry? entry;
  final FocusNode focusNode;
  final VoidCallback? onClose;

  const TextEditorToolbar({
    super.key,
    required this.controller,
    this.entry,
    required this.focusNode,
    this.onClose,
  });
  @override
  State<TextEditorToolbar> createState() => _TextEditorToolbarState();
}

class _TextEditorToolbarState extends State<TextEditorToolbar> {
  bool isTextColorSelected = false;

  void requestFocus() {
    if (!widget.focusNode.hasFocus) {
      widget.focusNode.requestFocus();
    }
  }

  void toggleList(QuillController controller, Attribute attribute) {
    requestFocus();
    final attrs = controller.getSelectionStyle().attributes;
    final currentAttr = attrs[attribute.key];

    if (currentAttr != null && currentAttr.value == attribute.value) {
      controller.formatSelection(Attribute.clone(attribute, null));
    } else {
      if (attribute.key == Attribute.header.key) {
        controller.formatSelection(Attribute.clone(Attribute.size, null));
      }
      controller.formatSelection(attribute);
    }
  }

  void changeTextColor(String color) {
    requestFocus();
    widget.controller.formatSelection(
      ColorAttribute(color.replaceAll('0xFF', '#')),
    );
    setState(() {
      isTextColorSelected = false;
    });
  }

  void textSizeFormat(Attribute attribute) {
    requestFocus();
    final currentSize =
        widget.controller
            .getSelectionStyle()
            .attributes[Attribute.size.key]
            ?.value;

    if (currentSize == attribute.value) {
      widget.controller.formatSelection(Attribute.clone(Attribute.size, null));
    } else {
      widget.controller.formatSelection(
        Attribute.clone(Attribute.header, null),
      );
      widget.controller.formatSelection(attribute);
    }
  }

  Color getCurrentFontColor() {
    final style = widget.controller.getSelectionStyle();
    final colorHex = style.attributes[Attribute.color.key]?.value;

    if (colorHex != null) {
      return _hexToColor(colorHex);
    }
    return NotesColors.editorTextColor;
  }

  Color _hexToColor(String hex) {
    hex = hex.replaceAll("#", "0xFF");
    return Color(int.parse(hex));
  }

  Widget getCurrentSelectedFormat() {
    final style = widget.controller.getSelectionStyle();
    final headerValue = style.attributes[Attribute.header.key]?.value;
    final sizeValue = style.attributes[Attribute.size.key]?.value;

    if (headerValue == 1) {
      return EditorIconButton(
        isSelected: true,
        iconPath: NotesIcon.h1Icon,
        onPressed: () => toggleList(widget.controller, Attribute.h1),
      );
    } else if (headerValue == 2) {
      return EditorIconButton(
        isSelected: true,
        iconPath: NotesIcon.h2Icon,
        onPressed: () => toggleList(widget.controller, Attribute.h2),
      );
    } else if (sizeValue == '14') {
      return EditorIconButton(
        isSelected: true,
        iconPath: NotesIcon.t1Icon,
        onPressed: () => textSizeFormat(SizeAttribute('14')),
      );
    } else {
      return EditorIconButton(
        isSelected: sizeValue == '12',
        iconPath: NotesIcon.t2Icon,
        onPressed: () => textSizeFormat(SizeAttribute('12')),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    return ToolbarContainer(
      child: [
        ToolbarRow(
          child: [
            Expanded(
              child: EditorIconButton(
                isSelected: widget.controller.getSelectionStyle().containsKey(
                  Attribute.bold.key,
                ),
                iconPath: NotesIcon.boldIcon,
                onPressed: () => toggleList(widget.controller, Attribute.bold),
              ),
            ),
            Expanded(
              child: EditorIconButton(
                isSelected: widget.controller.getSelectionStyle().containsKey(
                  Attribute.italic.key,
                ),
                iconPath: NotesIcon.italicIcon,
                onPressed: () {
                  toggleList(widget.controller, Attribute.italic);
                },
              ),
            ),
            Expanded(
              child: EditorIconButton(
                isSelected: widget.controller.getSelectionStyle().containsKey(
                  Attribute.underline.key,
                ),
                iconPath: NotesIcon.textUnderlineIcon,
                onPressed: () {
                  toggleList(widget.controller, Attribute.underline);
                },
              ),
            ),
            Expanded(
              child: EditorIconButton(
                isSelected: widget.controller.getSelectionStyle().containsKey(
                  Attribute.strikeThrough.key,
                ),
                iconPath: NotesIcon.strikeThroughIcon,
                onPressed: () {
                  toggleList(widget.controller, Attribute.strikeThrough);
                },
              ),
            ),
          ],
        ),
        ToolbarRow(
          isBorder: false,
          child: [
            if (!isTextColorSelected) ...[
              Expanded(
                child: EditorIconButton(
                  isSelected:
                      widget.controller
                          .getSelectionStyle()
                          .attributes[Attribute.header.key]
                          ?.value ==
                      1,
                  iconPath: NotesIcon.h1Icon,
                  onPressed: () => toggleList(widget.controller, Attribute.h1),
                ),
              ),
              Expanded(
                child: EditorIconButton(
                  isSelected:
                      widget.controller
                          .getSelectionStyle()
                          .attributes[Attribute.header.key]
                          ?.value ==
                      2,
                  iconPath: NotesIcon.h2Icon,
                  onPressed: () => toggleList(widget.controller, Attribute.h2),
                ),
              ),
              Expanded(
                child: EditorIconButton(
                  isSelected:
                      widget.controller
                          .getSelectionStyle()
                          .attributes[Attribute.size.key]
                          ?.value ==
                      '14',
                  iconPath: NotesIcon.t1Icon,
                  onPressed: () => textSizeFormat(SizeAttribute('14')),
                ),
              ),
            ],
            if (isTextColorSelected)
              Expanded(
                child: Container(
                  decoration: BoxDecoration(
                    border: Border(right: borderSideStyle),
                  ),
                  child: getCurrentSelectedFormat(),
                ),
              ),
            if (!isTextColorSelected)
              Expanded(
                child: Container(
                  decoration: BoxDecoration(
                    border: Border(right: borderSideStyle),
                  ),
                  child: EditorIconButton(
                    isSelected:
                        widget.controller
                            .getSelectionStyle()
                            .attributes[Attribute.size.key]
                            ?.value ==
                        '12',
                    iconPath: NotesIcon.t2Icon,
                    onPressed: () => textSizeFormat(SizeAttribute('12')),
                  ),
                ),
              ),
            if (!isTextColorSelected)
              Expanded(
                child: EditorIconButton(
                  isSelected: false,
                  icon: Container(
                    height: 28,
                    width: 28,
                    decoration: BoxDecoration(
                      color: getCurrentFontColor(),
                      borderRadius: CircularRadius.sm,
                    ),
                  ),
                  onPressed:
                      () => {
                        setState(() {
                          isTextColorSelected = !isTextColorSelected;
                        }),
                      },
                  border: Border(left: borderSideStyle),
                ),
              ),
            if (isTextColorSelected) ...[
              ...colorItems.map(
                (item) => Expanded(
                  child: Center(
                    child: ColorButton(
                      isSelected:
                          getCurrentFontColor() == Color(int.parse(item.color)),
                      color: Color(int.parse(item.color)),
                      onPressed: () => changeTextColor(item.color),
                    ),
                  ),
                ),
              ),
            ],
          ],
        ),
      ],
    );
  }
}
