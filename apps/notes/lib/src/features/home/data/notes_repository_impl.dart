import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:hive/hive.dart';
import 'package:logger/logger.dart';
import 'package:mechanix_notes/models/note_hive.dart';
import 'package:mechanix_notes/src/features/editor/models/editor_details_models.dart';
import 'package:mechanix_notes/src/features/home/data/note_height_calculator.dart';
import 'package:mechanix_notes/src/features/home/data/notes_repository.dart';
import 'package:mechanix_notes/src/constants/constants.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';
import 'package:uuid/uuid.dart';

List<NoteMetaData> _processNotesInBackground(List<NoteMetaData> notes) {
  notes.sort((a, b) => b.updatedAt.compareTo(a.updatedAt));
  return notes;
}

class NotesRepositoryImpl extends NotesRepository {
  final logger = Logger();

  @override
  Future<NoteMetaData> createNote(
    String title,
    content,
    plainText,
    bool isPinned,
    String tag,
  ) async {
    try {
      await ensureHiveConnected();

      final uuid = const Uuid();

      // Build and store optimized preview format
      final preview = convertDeltaToLines(
        jsonDecode(content),
        maxLines: NoteHeightCalculator.maxPreviewLines,
      );
      final calculatedHeight = NoteHeightCalculator.calculateNoteHeight(
        title,
        preview,
      );

      final newNote = NoteHive(
        id: uuid.v4(),
        title: title,
        content: content, // full quill json
        preview: jsonEncode(preview.map((e) => e.toJson()).toList()),
        height: calculatedHeight,
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
        plainText: plainText,
        isPinned: isPinned,
        tag: tag,
      );

      await Hive.box<NoteHive>(Constants.tableName).put(newNote.id, newNote);

      return NoteMetaData(
        id: newNote.id,
        preview: preview,
        height: calculatedHeight,
        title: newNote.title,
        createdAt: newNote.createdAt,
        updatedAt: newNote.updatedAt,
        isPinned: newNote.isPinned,
      );
    } catch (e) {
      logger.e("Failed to create note: $e");
      rethrow;
    }
  }

  List<NoteLine> convertDeltaToLines(dynamic deltaJson, {int maxLines = 2}) {
    try {
      // Normalize deltaJson to a List of maps
      final List ops =
          (deltaJson is Map) ? (deltaJson["ops"] as List) : (deltaJson as List);

      List<NoteLine> lines = [];
      List<NoteSpan> spans = [];
      Map<String, dynamic>? pendingLineAttr;
      String? previousLineType; // Track previous line type for grouping

      void flushLine() {
        // Stop processing if we've reached maxLines
        if (lines.length >= maxLines) return;

        final attr = pendingLineAttr ?? {};
        String type = "text";
        bool? checked;

        if (attr["code-block"] == true) type = "code";
        if (attr["header"] == 1) type = "h1";
        if (attr["header"] == 2) type = "h2";
        if (attr["blockquote"] == true) type = "quote";
        if (attr["list"] == "bullet") type = "bullet";
        if (attr["list"] == "ordered") type = "number";

        // Handle checkboxes with checked state
        if (attr["list"] == "checked") {
          type = "checkbox";
          checked = true;
        }
        if (attr["list"] == "unchecked") {
          type = "checkbox";
          checked = false;
        }

        // Check if we should merge with previous line (for code blocks)
        if (type == "code" &&
            previousLineType == "code" &&
            lines.isNotEmpty &&
            lines.last.type == "code") {
          print("merging");
          // Add newline span to separate lines within code block
          lines.last.spans.add(NoteSpan(text: "\n"));
          // Merge current spans into last code block
          lines.last.spans.addAll(spans);
        } else {
          // Create new line
          lines.add(
            NoteLine(type: type, spans: List.from(spans), checked: checked),
          );
        }

        previousLineType = type;
        spans.clear();
        pendingLineAttr = null;
      }

      for (final rawOp in ops) {
        // Stop early if we've reached maxLines
        if (lines.length >= maxLines) break;

        // Force op → Map<String, dynamic>
        final op = Map<String, dynamic>.from(rawOp);

        // Force attributes → Map<String, dynamic>
        final attr =
            op["attributes"] != null
                ? Map<String, dynamic>.from(op["attributes"])
                : <String, dynamic>{};

        final insert = op["insert"];

        // Simple newline
        if (insert == "\n") {
          pendingLineAttr = attr;
          flushLine();
          continue;
        }

        // Multi-line text inside one insert
        if (insert is String && insert.contains("\n")) {
          final parts = insert.split("\n");

          for (int i = 0; i < parts.length; i++) {
            if (lines.length >= maxLines) break;

            if (parts[i].isNotEmpty) {
              spans.add(
                NoteSpan(
                  text: parts[i],
                  bold: attr["bold"] == true,
                  italic: attr["italic"] == true,
                  underline: attr["underline"] == true,
                  strike: attr["strike"] == true,
                  color: attr["color"],
                  background: attr["background"],
                ),
              );
            }

            if (i != parts.length - 1) {
              pendingLineAttr = attr;
              flushLine();
            }
          }

          continue;
        }

        // Normal inline text
        spans.add(
          NoteSpan(
            text: insert.toString(),
            bold: attr["bold"] == true,
            italic: attr["italic"] == true,
            underline: attr["underline"] == true,
            strike: attr["strike"] == true,
            color: attr["color"],
            background: attr["background"],
          ),
        );
      }

      // Flush last line if needed (and if we haven't reached maxLines)
      if (spans.isNotEmpty && lines.length < maxLines) {
        flushLine();
      }

      return lines;
    } catch (e, st) {
      print("❌ convertDeltaToLines failed: $e\n$st");
      return [];
    }
  }

  @override
  Future<List<NoteMetaData>> getAllNotes() async {
    try {
      await ensureHiveConnected();
      final box = Hive.box<NoteHive>(Constants.tableName);

      if (box.isEmpty) {
        logger.i("No notes found");
        return [];
      }

      // Extract metadata
      final notes =
          box.values.map((note) {
            final List data = jsonDecode(note.preview);

            return NoteMetaData(
              id: note.id,
              height: note.height,
              title: note.title,
              createdAt: note.createdAt,
              updatedAt: note.updatedAt,
              isPinned: note.isPinned,
              preview: data.map((e) => NoteLine.fromJson(e)).toList(),
            );
          }).toList();

      // Sort by updatedAt descending
      // notes.sort((a, b) => b.updatedAt.compareTo(a.updatedAt));
      final sortedNotes = compute(_processNotesInBackground, notes);

      logger.i("Fetched all notes → total: ${notes.length}, ");

      return sortedNotes;
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
    bool isPinned,
    String tag,
  ) async {
    try {
      await ensureHiveConnected();
      final notesBox = Hive.box<NoteHive>(Constants.tableName);

      final preview = convertDeltaToLines(
        jsonDecode(content),
        maxLines: NoteHeightCalculator.maxPreviewLines,
      );
      final calculatedHeight = NoteHeightCalculator.calculateNoteHeight(
        title,
        preview,
      );

      final key = notesBox.keys.firstWhere(
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
            preview: jsonEncode(preview.map((e) => e.toJson()).toList()),
            updatedAt: DateTime.now(),
            plainText: plainText,
            height: calculatedHeight,
            isPinned: isPinned,
            tag: tag,
          );

          await notesBox.put(key, updatedNote);
          logger.i('Note updated successfully: $id');
        }
      } else {
        logger.e('Note with id $id not found');
      }
    } catch (e) {
      logger.e('Failed to update note: $e');
    }
  }

  @override
  Future<void> deleteNote(List<String> deleteIds) async {
    try {
      await ensureHiveConnected();
      final notesBox = Hive.box<NoteHive>(Constants.tableName);

      final keysToDelete =
          notesBox.keys.where((key) {
            final note = notesBox.get(key);
            return note != null && deleteIds.contains(note.id);
          }).toList();

      if (keysToDelete.isNotEmpty) {
        await notesBox.deleteAll(keysToDelete);
        logger.i('Deleted ${keysToDelete.length} note(s)');
      } else {
        logger.w('No notes found to delete');
      }
    } catch (e) {
      logger.e('Failed to delete notes: $e');
    }
  }

  @override
  Future<void> updateTag(List<String> noteIds, String tag) async {
    try {
      await ensureHiveConnected();
      final notesBox = Hive.box<NoteHive>(Constants.tableName);
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
              height: note.height,
              preview: note.preview,
            );
          }
        } else {
          logger.w('Note with id $id not found');
        }
      }

      if (updates.isNotEmpty) {
        await notesBox.putAll(updates);
        logger.i('Updated tags for ${updates.length} note(s)');
      }
    } catch (e) {
      logger.e('Failed to update tags: $e');
      rethrow;
    }
  }

  @override
  Future<void> pinnedNotes(List<String> noteIds, bool isPinned) async {
    try {
      await ensureHiveConnected();
      final notesBox = Hive.box<NoteHive>(Constants.tableName);

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
              height: note.height,
              tag: note.tag,
              preview: note.preview,
            );
          }
        } else {
          logger.w('Note with id $id not found');
        }
      }

      if (updates.isNotEmpty) {
        await notesBox.putAll(updates);
        logger.i('Updated pin status for ${updates.length} note(s)');
      }
    } catch (e) {
      logger.e('Failed to update pin status: $e');
    }
  }

  @override
  Future<List<NoteMetaData>> searchNotes(String searchQuery) async {
    try {
      await ensureHiveConnected();
      final box = Hive.box<NoteHive>(Constants.tableName);

      final query = searchQuery.trim().toLowerCase();

      if (query.isEmpty) {
        logger.i("Empty search query");
        return [];
      }

      final searchedNotes =
          box.values
              .where(
                (note) =>
                    note.title.toLowerCase().contains(query) ||
                    note.plainText.toLowerCase().contains(query),
              )
              .map((note) {
                final List data = jsonDecode(note.preview);
                return NoteMetaData(
                  id: note.id,
                  height: note.height,
                  title: note.title,
                  createdAt: note.createdAt,
                  updatedAt: note.updatedAt,
                  isPinned: note.isPinned,
                  preview: data.map((e) => NoteLine.fromJson(e)).toList(),
                );
              })
              .toList();
      print("searchedNotes: $searchedNotes");
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
      await ensureHiveConnected();
      final box = Hive.box<NoteHive>(Constants.tableName);
      final note = box.get(noteId);

      if (note != null) {
        logger.i('Note found: $noteId');
        return EditorPayload(
          id: note.id,
          content: note.content,
          isPinned: note.isPinned,
        );
      }

      logger.w('Note not found: $noteId');
      return null;
    } catch (e) {
      logger.e("Failed to find note by ID: $e");
      return null;
    }
  }

  Future<void> ensureHiveConnected() async {
    if (!Hive.isBoxOpen(Constants.tableName)) {
      await Hive.openBox<NoteHive>(Constants.tableName);
    }
  }
}
