import 'package:mechanix_notes/models/note_hive.dart';

abstract class NotesRepository {
  Future<List<NoteHive>> getNotes();
  Future<void> deleteNote(String id);
  Future<void> createNote(String title, String content,String plainText);
  Future<void> updateNote(String title, String content,String id, String plainText);
}
