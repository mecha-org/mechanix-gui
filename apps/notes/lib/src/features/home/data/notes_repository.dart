import 'package:mechanix_notes/src/features/editor/models/editor_details_models.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';

abstract class NotesRepository {
  Future<List<NoteMetaData>> getAllNotes();

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

  Future<void> deleteNote(List<String> deleteIds);

  Future<void> updateTag(List<String> noteIds, String tag);

  Future<void> pinnedNotes(List<String> noteIds, bool isPinned);

  Future<List<NoteMetaData>> searchNotes(String searchQuery);

  Future<EditorPayload?> findById(String noteId);
}
