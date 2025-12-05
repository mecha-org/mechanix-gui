import 'package:mechanix_notes/src/features/editor/models/editor_details_models.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';

abstract class NotesRepository {
  Future<List<NoteMetaData>> getAllNotes();

  Future<NoteMetaData> createNote(
    String title,
    String content,
    String plainText,
  );
  Future<void> updateNote(
    String title,
    String content,
    String id,
    String plainText,
  );

  Future<void> deleteNote(List<String> deleteIds);

  Future<List<SearchMetaData>> searchNotes(String searchQuery);

  Future<EditorPayload?> findById(String noteId);
}
