import 'package:hive/hive.dart';
import 'package:logger/logger.dart';
import 'package:mechanix_notes/models/note_hive.dart';
import 'package:mechanix_notes/src/features/editor/models/editor_details_models.dart';
import 'package:mechanix_notes/src/features/home/data/notes_repository.dart';
import 'package:mechanix_notes/src/constants/constants.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';
import 'package:intl/intl.dart';
import 'package:uuid/uuid.dart';

class NotesRepositoryImpl extends NotesRepository {
  final logger = Logger();

  @override
  Future<void> createNote(
    String title,
    content,
    plainText,
    bool isPinned,
    String tag,
  ) async {
    try {
      await ensureHiveConnected();
      final uuid = const Uuid();
      final newNote = NoteHive(
        id: uuid.v4(),
        title: title,
        content: content,
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
        plainText: plainText,
        isPinned: isPinned,
        tag: tag,
      );
      await Hive.box<NoteHive>(Constants.tableName).put(newNote.id, newNote);
      logger.i("Reach the end");
    } catch (e) {
      logger.e("Failed to add note $e");
    }
  }

  @override
  Future<NotesResult> getNotes() async {
    try {
      await ensureHiveConnected();
      final box = Hive.box<NoteHive>(Constants.tableName);

      if (box.isEmpty) {
        logger.i("No notes found.");
        return NotesResult(notes: [], pinnedNotes: [], groupedNotes: []);
      }

      // ✅ Extract only necessary fields into NoteMetaData
      final notes =
          box.values.map((note) {
            return NoteMetaData(
              id: note.id,
              title: note.title,
              createdAt: note.createdAt,
              updatedAt: note.updatedAt,
              isPinned: note.isPinned,
            );
          }).toList();

      // Sort by updatedAt descending
      notes.sort((a, b) => b.updatedAt.compareTo(a.updatedAt));

      final now = DateTime.now();
      final today = DateTime(now.year, now.month, now.day);
      final yesterday = today.subtract(const Duration(days: 1));
      final thisWeekStart = today.subtract(Duration(days: now.weekday - 1));
      final lastWeekStart = thisWeekStart.subtract(const Duration(days: 7));
      final thisMonthStart = DateTime(now.year, now.month, 1);
      final lastMonthStart = DateTime(now.year, now.month - 1, 1);

      final pinnedNotes = <NoteMetaData>[];
      final Map<String, List<NoteMetaData>> grouped = {};

      const groupOrder = [
        "Today",
        "Yesterday",
        "This Week",
        "Last Week",
        "This Month",
        "Last Month",
      ];

      for (final note in notes) {
        if (note.isPinned) pinnedNotes.add(note);

        final dateOnly = DateTime(
          note.updatedAt.year,
          note.updatedAt.month,
          note.updatedAt.day,
        );

        final label = _getTimeLabel(
          dateOnly,
          today,
          yesterday,
          thisWeekStart,
          lastWeekStart,
          thisMonthStart,
          lastMonthStart,
          note.updatedAt,
        );

        grouped.putIfAbsent(label, () => <NoteMetaData>[]).add(note);
      }

      final groupedNotes = <GroupedNotes>[];

      for (final groupLabel in groupOrder) {
        final groupNotes = grouped.remove(groupLabel);
        if (groupNotes != null && groupNotes.isNotEmpty) {
          groupedNotes.add(GroupedNotes(label: groupLabel, notes: groupNotes));
        }
      }

      if (grouped.isNotEmpty) {
        for (final entry in grouped.entries) {
          groupedNotes.add(GroupedNotes(label: entry.key, notes: entry.value));
        }
      }

      logger.i(
        "Fetched notes → pinned: ${pinnedNotes.length}, "
        "groups: ${groupedNotes.length}, total: ${notes.length}",
      );

      return NotesResult(
        pinnedNotes: pinnedNotes,
        groupedNotes: groupedNotes,
        notes: notes,
      );
    } catch (e) {
      logger.e('Failed to fetch notes: $e');
      return NotesResult(pinnedNotes: [], groupedNotes: [], notes: []);
    }
  }

  String _getTimeLabel(
    DateTime dateOnly,
    DateTime today,
    DateTime yesterday,
    DateTime thisWeekStart,
    DateTime lastWeekStart,
    DateTime thisMonthStart,
    DateTime lastMonthStart,
    DateTime originalDate,
  ) {
    if (dateOnly == today) return "Today";
    if (dateOnly == yesterday) return "Yesterday";

    if (dateOnly.isAfter(thisWeekStart.subtract(const Duration(days: 1))) &&
        dateOnly.isBefore(today)) {
      return "This Week";
    }

    if (dateOnly.isAfter(lastWeekStart.subtract(const Duration(days: 1))) &&
        dateOnly.isBefore(thisWeekStart)) {
      return "Last Week";
    }

    if (dateOnly.isAfter(thisMonthStart.subtract(const Duration(days: 1))) &&
        dateOnly.isBefore(today)) {
      return "This Month";
    }

    if (dateOnly.isAfter(lastMonthStart.subtract(const Duration(days: 1))) &&
        dateOnly.isBefore(thisMonthStart)) {
      return "Last Month";
    }

    // For older dates, use month-year format
    return DateFormat("MMMM yyyy").format(originalDate);
  }

  @override
  Future<void> updateNote(
    String title,
    String content,
    String id,
    String plainText,
    bool isPinned,
    String tag,
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
          isPinned: isPinned,
          tag: tag,
        );

        await notesBox.put(key, updatedNote);
      }
    } else {
      logger.e('Note with id $id not found.');
    }
  }

  @override
  Future<void> deleteNote(List<String> deleteIds) async {
    await ensureHiveConnected();
    final notesBox = Hive.box<NoteHive>(Constants.tableName);

    // Collect the box keys of notes that match deleteIds
    final keysToDelete =
        notesBox.keys.where((key) {
          final note = notesBox.get(key);
          return note != null && deleteIds.contains(note.id); // UUID is string
        }).toList();

    // Delete all matching notes
    if (keysToDelete.isNotEmpty) {
      await notesBox.deleteAll(keysToDelete);
    }
  }

  Future<void> ensureHiveConnected() async {
    if (!Hive.isBoxOpen(Constants.tableName)) {
      await Hive.openBox(Constants.tableName);
    }
  }

  @override
  Future<void> updateTag(List<String> noteIds, String tag) async {
    await ensureHiveConnected();

    final notesBox = await Hive.openBox<NoteHive>(Constants.tableName);
    final now = DateTime.now();

    final updates = <dynamic, NoteHive>{};

    for (final id in noteIds) {
      final key = notesBox.keys.firstWhere(
        (k) => notesBox.get(k)?.id == id,
        orElse: () => null,
      );

      if (key != null) {
        final note = notesBox.get(key);
        if (note != null) {
          updates[key] = NoteHive(
            id: note.id,
            title: note.title,
            content: note.content,
            createdAt: note.createdAt,
            updatedAt: now,
            plainText: note.plainText,
            isPinned: note.isPinned,
            tag: tag,
          );
        }
      } else {
        logger.w('Note with id $id not found.');
      }
    }

    if (updates.isNotEmpty) {
      await notesBox.putAll(updates);
    }
  }

  @override
  Future<void> pinnedNotes(List<String> noteIds, bool isPinned) async {
    await ensureHiveConnected();

    final notesBox = await Hive.openBox<NoteHive>(Constants.tableName);

    final updates = <dynamic, NoteHive>{};

    for (final id in noteIds) {
      final key = notesBox.keys.firstWhere(
        (k) => notesBox.get(k)?.id == id,
        orElse: () => null,
      );

      if (key != null) {
        final note = notesBox.get(key);
        if (note != null) {
          updates[key] = NoteHive(
            id: note.id,
            title: note.title,
            content: note.content,
            createdAt: note.createdAt,
            updatedAt: note.updatedAt,
            plainText: note.plainText,
            isPinned: isPinned,
            tag: note.tag,
          );
        }
      } else {
        logger.w('Note with id $id not found.');
      }
    }

    if (updates.isNotEmpty) {
      await notesBox.putAll(updates);
    }
  }

  @override
  Future<List<NoteMetaData>> searchNotes(String searchQuery) async {
    try {
      await ensureHiveConnected();
      final box = Hive.box<NoteHive>(Constants.tableName);

      final query = searchQuery.trim().toLowerCase();

      if (query.isEmpty) {
        logger.i("Empty search query → no results");
        return [];
      }

      final searchedNotes =
          box.values
              .where(
                (note) =>
                    note.title.toLowerCase().contains(query) ||
                    (note.plainText.toLowerCase().contains(query)),
              )
              .map(
                (note) => NoteMetaData(
                  id: note.id,
                  title: note.title,
                  createdAt: note.createdAt,
                  updatedAt: note.updatedAt,
                  isPinned: note.isPinned,
                ),
              )
              .toList();

      searchedNotes.sort((a, b) => b.updatedAt.compareTo(a.updatedAt));

      logger.i("Found ${searchedNotes.length} notes matching '$query'");

      return searchedNotes;
    } catch (e) {
      logger.e("Search failed: $e");
      return [];
    }
  }

  @override
  Future<EditorPayload?> findById(String noteId) async {
    try {
      logger.i('searching the selected notes');
      await ensureHiveConnected();

      final box = Hive.box<NoteHive>(Constants.tableName);
      final note = box.get(noteId);

      if (note != null) {
        return EditorPayload(
          id: note.id,
          content: note.content,
          isPinned: note.isPinned,
        );
      }
      return null;
    } catch (e) {
      logger.e("Failed to find note by ID: $e");
      return null;
    }
  }
}
