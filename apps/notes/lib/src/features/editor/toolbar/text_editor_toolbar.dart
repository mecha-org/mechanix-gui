
import 'package:flutter/material.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/features/editor/editor_icon_button.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/focus_preserve_button.dart';
import 'package:widgets/extensions/color.dart';

class TextEditorToolbar extends StatefulWidget {
  final QuillController controller;
  final FocusNode focusNode;

  const TextEditorToolbar({
    super.key,
    required this.controller,
    required this.focusNode,
  });
  @override
  State<TextEditorToolbar> createState() => _TextEditorToolbarState();
}

class _TextEditorToolbarState extends State<TextEditorToolbar> {
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
    final attrs = widget.controller.getSelectionStyle().attributes;
    final currentAttr = attrs[attribute.key];

    if (currentAttr != null && currentAttr.value == attribute.value) {
      widget.controller.formatSelection(Attribute.clone(attribute, null));
    } else {
      if (attribute.key == Attribute.header.key) {
        widget.controller.formatSelection(
          Attribute.clone(Attribute.size, null),
        );
      }
      widget.controller.formatSelection(attribute);
    }
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

  void backgroundColorFormat() {
    requestFocus();
    final currentBg =
        widget.controller
            .getSelectionStyle()
            .attributes[Attribute.background.key]
            ?.value;

    // If selecting the same color → remove it
    context.primaryContainer.withValues(alpha: 0.8);
    final hex = colorToHex(context.primaryContainer.withValues(alpha: 0.8));
    if (currentBg == hex) {
      widget.controller.formatSelection(
        Attribute(Attribute.background.key, AttributeScope.inline, null),
      );
    } else {
      // Apply background color
      widget.controller.formatSelection(
        Attribute(Attribute.background.key, AttributeScope.inline, hex),
      );
    }
  }

  String colorToHex(Color color) {
    final r = (color.r * 255).floor().toRadixString(16).padLeft(2, '0');
    final g = (color.g * 255).floor().toRadixString(16).padLeft(2, '0');
    final b = (color.b * 255).floor().toRadixString(16).padLeft(2, '0');
    final a = (color.a * 255).floor().toRadixString(16).padLeft(2, '0');

    return '#${a}${r}${g}${b}'.toUpperCase();
  }

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: widget.controller,
      builder: (context, child) {
        return Row(
          crossAxisAlignment: CrossAxisAlignment.center,
          spacing: 24,
          children: [
            FocusPreserveButton(
              child: EditorIconButton(
                isSelected: widget.controller.getSelectionStyle().containsKey(
                  Attribute.bold.key,
                ),
                iconPath: NotesIcon.boldIcon,
                onPressed: () => toggleList(Attribute.bold),
              ),
            ),
            FocusPreserveButton(
              child: EditorIconButton(
                isSelected: widget.controller.getSelectionStyle().containsKey(
                  Attribute.italic.key,
                ),
                iconPath: NotesIcon.italicIcon,
                onPressed: () {
                  toggleList(Attribute.italic);
                },
              ),
            ),
            FocusPreserveButton(
              child: EditorIconButton(
                isSelected: widget.controller.getSelectionStyle().containsKey(
                  Attribute.underline.key,
                ),
                iconPath: NotesIcon.textUnderlineIcon,
                onPressed: () {
                  toggleList(Attribute.underline);
                },
              ),
            ),
            FocusPreserveButton(
              child: EditorIconButton(
                isSelected: widget.controller.getSelectionStyle().containsKey(
                  Attribute.background.key,
                ),
                iconPath: NotesIcon.highlightIcon,
                onPressed: () {
                  backgroundColorFormat();
                },
              ),
            ),
            // Divider
            Container(color: context.outline, height: 24, width: 1),

            FocusPreserveButton(
              child: EditorIconButton(
                isSelected:
                    widget.controller
                        .getSelectionStyle()
                        .attributes[Attribute.header.key]
                        ?.value ==
                    1,
                iconPath: NotesIcon.h1Icon,
                onPressed: () => toggleList(Attribute.h1),
              ),
            ),
            FocusPreserveButton(
              child: EditorIconButton(
                isSelected:
                    widget.controller
                        .getSelectionStyle()
                        .attributes[Attribute.header.key]
                        ?.value ==
                    2,
                iconPath: NotesIcon.h2Icon,
                onPressed: () => toggleList(Attribute.h2),
              ),
            ),
            FocusPreserveButton(
              child: EditorIconButton(
                isSelected:
                    widget.controller
                                .getSelectionStyle()
                                .attributes[Attribute.header.key]
                                ?.key ==
                            "header"
                        ? false
                        : true,
                iconPath: NotesIcon.normalTextIcon,
                onPressed: () => textSizeFormat(const SizeAttribute("18")),
              ),
            ),
            FocusPreserveButton(
              child: EditorIconButton(
                isSelected: widget.controller.getSelectionStyle().containsKey(
                  Attribute.inlineCode.key,
                ),
                iconPath: NotesIcon.inlineCodeIcon,
                onPressed: () {
                  toggleList(Attribute.inlineCode);
                },
              ),
            ),
          ],
        );
      },
    );
  }
}
