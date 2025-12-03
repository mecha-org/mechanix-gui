import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_bloc.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_event.dart';
import 'package:mechanix_notes/src/features/editor/editor_icon_button.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/toolbar_container.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/toolbar_row.dart';
import 'package:mechanix_notes/src/features/editor/models/toolbar_models.dart';

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
    widget.focusNode.addListener(focusListener);
  }

  @override
  void dispose() {
    widget.focusNode.removeListener(focusListener);
    super.dispose();
  }

  void focusListener() {
    if (widget.focusNode.hasFocus) {
      context.read<EditorBloc>().add(
        SelectToolbar(activeToolbar: ToolbarEnum.none),
      );
    }
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
    if (currentBg == NotesColors.highlightColor) {
      widget.controller.formatSelection(
        Attribute(Attribute.background.key, AttributeScope.inline, null),
      );
    } else {
      // Apply background color
      widget.controller.formatSelection(
        Attribute(
          Attribute.background.key,
          AttributeScope.inline,
          NotesColors.highlightColor,
        ),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: widget.controller,
      builder: (context, child) {
        return ToolbarContainer(
          width: 540,
          height: 44,
          child: [
            ToolbarRow(
              child: [
                Expanded(
                  child: EditorIconButton(
                    isSelected: widget.controller
                        .getSelectionStyle()
                        .containsKey(Attribute.bold.key),
                    iconPath: NotesIcon.boldIcon,
                    onPressed: () => toggleList(Attribute.bold),
                  ),
                ),
                Expanded(
                  child: EditorIconButton(
                    isSelected: widget.controller
                        .getSelectionStyle()
                        .containsKey(Attribute.italic.key),
                    iconPath: NotesIcon.italicIcon,
                    onPressed: () {
                      toggleList(Attribute.italic);
                    },
                  ),
                ),
                Expanded(
                  child: EditorIconButton(
                    isSelected: widget.controller
                        .getSelectionStyle()
                        .containsKey(Attribute.underline.key),
                    iconPath: NotesIcon.textUnderlineIcon,
                    onPressed: () {
                      toggleList(Attribute.underline);
                    },
                  ),
                ),
                Expanded(
                  child: EditorIconButton(
                    isSelected: widget.controller
                        .getSelectionStyle()
                        .containsKey(Attribute.background.key),
                    iconPath: NotesIcon.highlightIcon,
                    onPressed: () {
                      backgroundColorFormat();
                    },
                  ),
                ),

                Expanded(
                  child: EditorIconButton(
                    isSelected: widget.controller
                        .getSelectionStyle()
                        .containsKey(Attribute.inlineCode.key),
                    iconPath: NotesIcon.inlineCodeIcon,
                    onPressed: () {
                      toggleList(Attribute.inlineCode);
                    },
                  ),
                ),

                Expanded(
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
                Expanded(
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

                Expanded(
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
              ],
            ),
          ],
        );
      },
    );
  }
}
