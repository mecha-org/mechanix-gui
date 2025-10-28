
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
  final List<NoteMetaData> pinnedNotes;
  final List<GroupedNotes> groupedNotes;
  final List<NoteMetaData> notes;

  NotesResult({required this.pinnedNotes, required this.groupedNotes, required this.notes});
}
class GroupedNotes {
  final String label;          // e.g. "This Month", "August 2025"
  final List<NoteMetaData> notes;

  GroupedNotes({required this.label, required this.notes});
}

class NoteMetaData {
  final String id;
  final String title;
  final DateTime createdAt;
  final DateTime updatedAt;
  final bool isPinned;

  NoteMetaData({
    required this.id,
    required this.title,
    required this.createdAt,
    required this.updatedAt,
    required this.isPinned,
  });
}
