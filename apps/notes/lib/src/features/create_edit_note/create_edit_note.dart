import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/logger.dart';
import 'package:mechanix_notes/models/note_hive.dart';
import 'package:mechanix_notes/src/commons/custom_app_bar.dart';
import 'package:mechanix_notes/src/features/create_edit_note/title_edit.dart';
import 'package:mechanix_notes/src/features/create_edit_note/content_edit.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:mechanix_notes/src/styles/constants.dart';

class CreateEditNote extends StatefulWidget {
  final NoteHive? note;

  const CreateEditNote({super.key, this.note});

  @override
  State<CreateEditNote> createState() => _CreateEditNoteState();
}

class _CreateEditNoteState extends State<CreateEditNote> {
  final _titleController = TextEditingController();
  final _titleFocusNode = FocusNode();
  final logger = Logger();

  bool toggleToolBar = true;

  String _contentDeltaJson = '';
  String _plainContent = '';
  int _contentCharCount = 0;

  @override
  void initState() {
    super.initState();

    if (widget.note != null) {
      _titleController.text = widget.note!.title;
      _contentDeltaJson = widget.note!.content;
      _contentCharCount = widget.note!.plainText.length;
      _plainContent = widget.note!.plainText;
    }
  }

  @override
  void dispose() {
    _titleController.dispose();
    _titleFocusNode.dispose();
    super.dispose();
  }

  Future<void> _saveNote(BuildContext ctx) async {
    final title = _titleController.text.trim();
    final content = _contentDeltaJson.trim();

    if (title.isEmpty || content.isEmpty) {
      ScaffoldMessenger.of(ctx).showSnackBar(
        const SnackBar(content: Text('Title and content cannot be empty.')),
      );
      return Navigator.pop(ctx);
    }

    final isEditing = widget.note != null;

    logger.i("Saving note. Title: $title, Content: $content");

    if (isEditing) {
      ctx.read<NotesBloc>().add(
        UpdateNotes(
          id: widget.note!.id,
          title: title,
          content: content,
          plainText: _plainContent,
        ),
      );
    } else {
      ctx.read<NotesBloc>().add(CreateNotes(title, content, _plainContent));
    }

    ScaffoldMessenger.of(ctx).showSnackBar(
      SnackBar(
        content: Text(
          isEditing ? 'Note updated successfully.' : 'Note saved successfully.',
        ),
      ),
    );

    Navigator.pop(ctx);
  }

  void _deleteNote(BuildContext ctx) {
    ctx.read<NotesBloc>().add(DeleteNotes(id: widget.note!.id));
    Navigator.pop(ctx);
  }

  @override
  Widget build(BuildContext context) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    final isEditing = widget.note != null;

    return Scaffold(
      appBar: CustomAppBar(
        title: isEditing ? 'Edit Note' : 'New Note',
        leftIcon: Image.asset(Images.back),
        leftIconOnTap: () => _saveNote(context),
        rightIcon1: isEditing ? Image.asset(Images.delete) : null,
        rightIcon1OnTap: isEditing ? () => _deleteNote(context) : null,
        rightIcon2: Icon(Icons.remove_red_eye),
        rightIcon2OnTap:
            () => setState(() {
              toggleToolBar = !toggleToolBar;
            }),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          children: [
            TitleEdit(controller: _titleController, focusNode: _titleFocusNode),
            const SizedBox(height: 12),
            // CONTENT EDITOR
            Expanded(
              child: Container(
                decoration: BoxDecoration(
                  borderRadius: BorderRadius.circular(12),
                  color: isDark ? Colors.grey[900] : Colors.grey[200],
                ),
                padding: const EdgeInsets.all(12),
                child: ContentEdit(
                  initialDeltaJson: _contentDeltaJson,
                  toggleToolBar: toggleToolBar,
                  onContentChanged: (newJson, plainText) {
                    _contentDeltaJson = newJson;
                    _plainContent = plainText;
                    setState(() {
                      _contentCharCount = plainText.length;
                    });
                  },
                ),
              ),
            ),

            const SizedBox(height: 12),
            Align(
              alignment: Alignment.bottomRight,
              child: Text(
                '$_contentCharCount characters',
                style: TextStyle(
                  fontSize: 12,
                  color: isDark ? Colors.grey[400] : Colors.grey[700],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
