import 'package:flutter/material.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/commons/styles/styles.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_bloc.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_event.dart';
import 'package:mechanix_notes/src/features/editor/editor_icon_button.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/toolbar_container.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/toolbar_row.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/src/features/editor/models/toolbar_models.dart';

class AlignmentToolbar extends StatefulWidget {
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
  State<AlignmentToolbar> createState() => _AlignmentToolbarState();
}

class _AlignmentToolbarState extends State<AlignmentToolbar> {
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

  void removeFocus() {
    if (widget.focusNode.hasFocus) {
      widget.focusNode.unfocus();
    }
  }

  void toggleList(Attribute attribute) {
    removeFocus();
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

  void toggleIndent({bool increase = true}) {
    removeFocus();
    widget.controller.indentSelection(increase);
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
        return Center(
          child: ToolbarContainer(
            width: 540,
            height: 44,
            child: [
              ToolbarRow(
                child: [
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
                        'unchecked',
                      ),
                      iconPath: NotesIcon.checkboxListIcon,
                      onPressed: () {
                        toggleList(Attribute.unchecked);
                      },
                      border: const Border(right: borderSideStyle),
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
            ],
          ),
        );
      },
    );
  }
}
