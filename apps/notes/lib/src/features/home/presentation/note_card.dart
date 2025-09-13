import 'package:flutter/material.dart';
import 'package:mechanix_notes/models/note_hive.dart';
import 'package:mechanix_notes/src/features/editor/notes_editor.dart';
import 'package:widgets/mechanix.dart';

class NoteCard extends StatefulWidget {
  final NoteHive note;

  const NoteCard({super.key, required this.note});

  @override
  State<NoteCard> createState() => _NoteCardState();
}

class _NoteCardState extends State<NoteCard> {
  bool isSelectionMode = false;

  void openNote() {
    Navigator.push(
      context,
      MaterialPageRoute(builder: (_) => NotesEditor(note: widget.note)),
    );
  }

  @override
  Widget build(BuildContext context) {
    return MechanixPressableList(
      onLongPress:
          () => setState(() {
            isSelectionMode = true;
          }),
      onTap: openNote,
      title: widget.note.title,
      selectionMode: isSelectionMode,
    );
  }
}

// class NoteCard extends StatelessWidget {
//   final NoteItem note;

//   const NoteCard({super.key, required this.note});

//   @override
//   Widget build(BuildContext context) {
//     final theme = Theme.of(context);
//     final formattedDate = CommonHelper.formatDateTime(note.updatedAt);

//     void openNote() {
//       Navigator.push(
//         context,
//         MaterialPageRoute(builder: (_) => CreateEditNote(note: note)),
//       );
//     }

//     return    MechanixPressableList(
//         onTap: openNote,
//     );
    //  GestureDetector(
    //   onTap: openNote,
    //   child: Container(
    //     padding: const EdgeInsets.all(16),
    //     decoration: BoxDecoration(
    //       color: Colors.black87,
    //       borderRadius: BorderRadius.circular(16),
    //       boxShadow: [
    //         BoxShadow(
    //           color: Colors.black12,
    //           blurRadius: 6,
    //           offset: const Offset(0, 3),
    //         ),
    //       ],
    //     ),
    //     child: Column(
    //       crossAxisAlignment: CrossAxisAlignment.start,
    //       children: [
    //         if (note.title.trim().isNotEmpty)
    //           Text(
    //             note.title,
    //             style: theme.textTheme.titleMedium?.copyWith(
    //               fontWeight: FontWeight.bold,
    //             ),
    //             maxLines: 1,
    //             overflow: TextOverflow.ellipsis,
    //           ),
    //         const SizedBox(height: 8),
    //         Text(
    //           note.plainText,
    //           style: theme.textTheme.bodyMedium?.copyWith(
    //             color: theme.hintColor,
    //           ),
    //           maxLines: 2,
    //           overflow: TextOverflow.ellipsis,
    //         ),
    //         const SizedBox(height: 12),
    //         Align(alignment: Alignment.bottomRight, child: Text(formattedDate)),
    //       ],
    //     ),
    //   ),
    // );
//   }
// }
