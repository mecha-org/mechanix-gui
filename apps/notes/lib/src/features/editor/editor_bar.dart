import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/models/note_hive.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_bloc.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_event.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_state.dart';
import 'package:mechanix_notes/src/features/editor/menu_options.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:tuple/tuple.dart';
import 'package:widgets/extensions/edge_insets.dart';
import 'package:widgets/widgets/navigation_bar/mechanix_navigation_bar.dart';

class EditorBar extends StatefulWidget {
  final QuillController controller;
  final NoteHive? note;
  const EditorBar({super.key, required this.controller, this.note});

  @override
  State<EditorBar> createState() => _EditorBarState();
}

class _EditorBarState extends State<EditorBar> {
  final TextEditingController _titleController = TextEditingController();

  final LayerLink optionsLayer = LayerLink();
  void _undoCall() {
    widget.controller.undo();
  }

  void _redoCall() {
    widget.controller.redo();
  }

  void _enableToolbar() {
    context.read<EditorBloc>().add(ToolbarToggle());
  }

  void _saveNotes(bool isPinned) {
    final title =
        _titleController.text.trim().isNotEmpty
            ? _titleController.text.trim()
            : "New Note";
    final content = jsonEncode(widget.controller.document.toDelta());
    final plainText = widget.controller.document.toPlainText();
    if (plainText.isNotEmpty && plainText != '\n') {
      if (widget.note != null) {
        context.read<NotesBloc>().add(
          UpdateNotes(
            id: widget.note!.id,
            content: content,
            title: title,
            plainText: plainText,
            isPinned: isPinned,
            tag: 'none',
          ),
        );
      } else {
        context.read<NotesBloc>().add(
          CreateNotes(title, content, plainText, isPinned, 'none'),
        );
      }
    }
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return MechanixNavigationBar(
      leadingWidth: 320,
      leadingWidget: Row(
        children: [
          BlocSelector<EditorBloc, EditorBlocState, bool>(
            selector: (state) => state.isPinned,
            builder: (context, isPinned) {
              return IconButton(
                icon: Image.asset(NotesIcon.backIcon, height: 20, width: 20),
                onPressed: () => _saveNotes(isPinned),
                padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 8),
              );
            },
          ),
          Expanded(
            child: TextFormField(
              autofocus: false,
              maxLines: 1,
              controller: _titleController,
              maxLength: 25,
              style: TextStyle(color: NotesColors.titleTextColor),
              decoration: InputDecoration(
                counterText: "",
                hintText: "New Note",
                hintStyle: TextStyle(color: NotesColors.titleTextColor),
                border: InputBorder.none,
                isCollapsed: true,
              ),
            ),
          ),
        ],
      ),
      actionsIconTheme: IconThemeData(size: 20),

      actionWidgets: [
        BlocSelector<EditorBloc, EditorBlocState, Tuple3<bool, bool, bool>>(
          selector:
              (state) =>
                  Tuple3(state.isUndo, state.isRedo, state.toolbarToggle),
          builder: (context, tuple) {
            final isUndo = tuple.item1;
            final isRedo = tuple.item2;
            final toolbarToggle = tuple.item3;

            return Row(
              children: [
                if (!toolbarToggle) ...[
                  IconButton(
                    onPressed: isUndo ? _undoCall : null,
                    icon: SizedBox(
                      height: 20,
                      width: 20,
                      child: Image.asset(
                        NotesIcon.undoIcon,
                        color:
                            isUndo
                                ? Colors.white
                                : Theme.of(context).disabledColor,
                      ),
                    ),
                  ).padRight(10),
                  IconButton(
                    onPressed: isRedo ? _redoCall : null,
                    icon: SizedBox(
                      height: 20,
                      width: 20,
                      child: Image.asset(
                        NotesIcon.redoIcon,
                        color:
                            isRedo
                                ? Colors.white
                                : Theme.of(context).disabledColor,
                      ),
                    ),
                  ).padRight(10),
                ],
                IconButton(
                  onPressed: _enableToolbar,
                  icon: SizedBox(
                    height: 20,
                    width: 20,
                    child: Image.asset(
                      toolbarToggle
                          ? NotesIcon.toolbarEnableIcon
                          : NotesIcon.toolbarDisableIcon,
                    ),
                  ),
                ),
                CompositedTransformTarget(
                  link: optionsLayer,
                  child: IconButton(
                    onPressed: () => _showOptions(context),
                    icon: SizedBox(
                      height: 20,
                      width: 20,
                      child: Image.asset(NotesIcon.threeDotIcon),
                    ),
                  ),
                ),
              ],
            );
          },
        ),
      ],
    );
  }

  void _showOptions(BuildContext context) {
    OverlayEntry? entry;

    entry = OverlayEntry(
      builder:
          (_) => MenuOptions(
            menuLink: optionsLayer,
            entry: entry,
            note: widget.note,
          ),
    );

    Overlay.of(context, rootOverlay: true).insert(entry);
  }
}
