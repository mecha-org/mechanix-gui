import 'package:flutter/material.dart';
import 'package:logger/logger.dart';
import 'package:mechanix_notes/models/note_hive.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/features/create_edit_note/title_edit.dart';
import 'package:mechanix_notes/src/features/create_edit_note/content_edit.dart';
import 'package:widgets/mechanix.dart';

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

  // Future<void> _saveNote(BuildContext ctx) async {
  //   final title = _titleController.text.trim();
  //   final content = _contentDeltaJson.trim();

  //   if (title.isEmpty || content.isEmpty) {
  //     ScaffoldMessenger.of(ctx).showSnackBar(
  //       const SnackBar(content: Text('Title and content cannot be empty.')),
  //     );
  //     return Navigator.pop(ctx);
  //   }

  //   final isEditing = widget.note != null;

  //   logger.i("Saving note. Title: $title, Content: $content");

  //   if (isEditing) {
  //     ctx.read<NotesBloc>().add(
  //       UpdateNotes(
  //         id: widget.note!.id,
  //         content: content,
  //         title: title,
  //         plainText: _plainContent,
  //       ),
  //     );
  //   } else {
  //     ctx.read<NotesBloc>().add(CreateNotes(title, content, _plainContent));
  //   }

  //   ScaffoldMessenger.of(ctx).showSnackBar(
  //     SnackBar(
  //       content: Text(
  //         isEditing ? 'Note updated successfully.' : 'Note saved successfully.',
  //       ),
  //     ),
  //   );

  //   Navigator.pop(ctx);
  // }

  // void _deleteNote(BuildContext ctx) {
  //   ctx.read<NotesBloc>().add(DeleteNotes(id: widget.note!.id));
  //   Navigator.pop(ctx);
  // }

  @override
  Widget build(BuildContext context) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    // final isEditing = widget.note != null;

    return Scaffold(
      appBar: MechanixNavigationBar(
        title: "New Note",
        titleStyle: context.textTheme.labelLarge,
        actionWidgets: [
          IconButton(
            onPressed: () {},
            icon: SizedBox(
              height: 20,
              width: 20,
              child: Image.asset(NotesIcon.undoIcon),
            ),
          ),
          IconButton(
            onPressed: () {},
            icon: SizedBox(
              height: 20,
              width: 20,
              child: Image.asset(NotesIcon.redoIcon),
            ),
          ),
          IconButton(
            onPressed: () {},
            icon: SizedBox(
              height: 20,
              width: 20,
              child: Image.asset(NotesIcon.toolbarDisableIcon),
            ),
          ),
          IconButton(onPressed: () {}, icon: Icon(Icons.more_vert)),
        ],
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
