import 'package:mechanix_notes/models/note_hive.dart';

class Note {
  final String id;
  final String title;
  final String content;
  final DateTime createdAt;
  final DateTime lastEditedAt;


  Note({
    required this.id,
    required this.title,
    required this.content,
    required this.createdAt,
    required this.lastEditedAt,
  });

  Note copyWith({
    String? title,
    String? content,
  }) {
    return Note(
      id: id,
      title: title ?? this.title,
      content: content ?? this.content,
      createdAt: createdAt,
      lastEditedAt: lastEditedAt
    );
  }
}
class NotesResult {
  final List<NoteHive> pinnedNotes;
  final List<GroupedNotes> groupedNotes;
  final List<NoteHive> notes;

  NotesResult({required this.pinnedNotes, required this.groupedNotes, required this.notes});
}
class GroupedNotes {
  final String label;          // e.g. "This Month", "August 2025"
  final List<NoteHive> notes;

  GroupedNotes({required this.label, required this.notes});
}