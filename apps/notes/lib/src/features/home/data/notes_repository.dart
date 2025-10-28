import 'package:mechanix_notes/models/note_hive.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';

abstract class NotesRepository {
  Future<NotesResult> getNotes();
  Future<void> deleteNote(List<String> deleteIds);
  Future<void> createNote(
    String title,
    String content,
    String plainText,
    bool isPinned,
    String tag,
  );
  Future<void> updateNote(
    String title,
    String content,
    String id,
    String plainText,
    bool isPinned,
    String tag,
  );

  Future<void> updateTag(List<String> noteIds, String tag);
  Future<void> pinnedNotes(List<String> noteIds, bool tag);
  Future<List<NoteMetaData>> searchNotes(String searchQuery);
  Future<NoteHive?> findById(String noteId);
}
