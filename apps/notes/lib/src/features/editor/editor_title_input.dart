import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/models/note_hive.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_bloc.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_state.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';

class EditorTitleInput extends StatefulWidget {
  final QuillController controller;
  final NoteHive? note;

  const EditorTitleInput({super.key, required this.controller, this.note});
  @override
  State<EditorTitleInput> createState() => _EditorTitleInputState();
}

class _EditorTitleInputState extends State<EditorTitleInput> {
  final TextEditingController _titleController = TextEditingController();

  @override
  void initState() {
    super.initState();
    if (widget.note?.id != null) {
      _titleController.text = widget.note?.title ?? '';
    }
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
    return Row(
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
    );
  }

  @override
  void dispose() {
    _titleController.dispose();
    super.dispose();
  }
}
