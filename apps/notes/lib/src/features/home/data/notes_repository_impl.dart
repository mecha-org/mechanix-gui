import 'package:hive/hive.dart';
import 'package:logger/logger.dart';
import 'package:mechanix_notes/models/note_hive.dart';
import 'package:mechanix_notes/src/features/home/data/notes_repository.dart';
import 'package:mechanix_notes/src/styles/constants.dart';
import 'package:uuid/uuid.dart';

class NotesRepositoryImpl extends NotesRepository {
  final logger = Logger();

  @override
  Future<void> createNote(title, content, plainText) async {
    try {
      await ensureHiveConnected();
      final uuid = Uuid();
      final newNote = NoteHive(
        id: uuid.v4(),
        title: title,
        content: content,
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
        plainText: plainText,
      );
      await Hive.box<NoteHive>(Constants.tableName).put(newNote.id, newNote);
      logger.i("Reach the end");
    } catch (e) {
      logger.e("Failed to add note $e");
    }
  }

  @override
  Future<List<NoteHive>> getNotes() async {
    try {
      await ensureHiveConnected();
      final box = Hive.box<NoteHive>(Constants.tableName);

      final notes = box.values.toList();

      return notes.isNotEmpty ? notes : [];
    } catch (e) {
      logger.e('Failed to fetch notes: $e');
      return [];
    }
  }

  @override
  Future<void> updateNote(
    String title,
    String content,
    String id,
    String plainText,
  ) async {
    await ensureHiveConnected();

    final notesBox = await Hive.openBox<NoteHive>(Constants.tableName);

    final key = await notesBox.keys.firstWhere(
      (key) => notesBox.get(key)?.id == id,
      orElse: () => null,
    );
    if (key != null) {
      final note = notesBox.get(key);

      if (note != null) {
        final updatedNote = NoteHive(
          id: note.id,
          title: title,
          content: content,
          createdAt: note.createdAt,
          updatedAt: DateTime.now(),
          plainText: plainText,
        );

        await notesBox.put(key, updatedNote);
      }
    } else {
      logger.e('Note with id $id not found.');
    }
  }

  @override
  Future<void> deleteNote(String id) async {
    await ensureHiveConnected();
    final notesBox = await Hive.openBox<NoteHive>(Constants.tableName);

    // Find the key of the note with the given id
    final keyToDelete = notesBox.keys.firstWhere(
      (key) => notesBox.get(key)?.id == id,
      orElse: () => null,
    );

    // If the note is found, delete it
    if (keyToDelete != null) {
      await notesBox.delete(keyToDelete);
    }
  }

  Future<void> ensureHiveConnected() async {
    if (!Hive.isBoxOpen(Constants.tableName)) {
      await Hive.openBox(Constants.tableName);
    }
  }
}
