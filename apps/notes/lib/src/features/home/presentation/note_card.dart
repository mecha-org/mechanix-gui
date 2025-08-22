import 'package:flutter/material.dart';
import 'package:mechanix_notes/models/note_hive.dart';
import 'package:mechanix_notes/src/commons/common_helper.dart';
import 'package:mechanix_notes/src/features/create_edit_note/create_edit_note.dart';

class NoteCard extends StatelessWidget {
  final NoteHive note;

  const NoteCard({super.key, required this.note});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final formattedDate = CommonHelper.formatDateTime(note.updatedAt);

    void openNote() {
      Navigator.push(
        context,
        MaterialPageRoute(builder: (_) => CreateEditNote(note: note)),
      );
    }

    return GestureDetector(
      onTap: () {
        openNote();
      },
      child: Container(
        padding: const EdgeInsets.all(16),
        decoration: BoxDecoration(
          color: Colors.black87,
          borderRadius: BorderRadius.circular(16),
          boxShadow: [
            BoxShadow(
              color: Colors.black12,
              blurRadius: 6,
              offset: const Offset(0, 3),
            ),
          ],
        ),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            if (note.title.trim().isNotEmpty)
              Text(
                note.title,
                style: theme.textTheme.titleMedium?.copyWith(
                  fontWeight: FontWeight.bold,
                ),
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
              ),
            const SizedBox(height: 8),
            Text(
              note.plainText,
              style: theme.textTheme.bodyMedium?.copyWith(
                color: theme.hintColor,
              ),
              maxLines: 2,
              overflow: TextOverflow.ellipsis,
            ),
            const SizedBox(height: 12),
            Align(alignment: Alignment.bottomRight, child: Text(formattedDate)),
          ],
        ),
      ),
    );
  }
}
