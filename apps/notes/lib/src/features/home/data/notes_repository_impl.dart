import 'dart:async';
import 'dart:convert';

import 'package:dbus/dbus.dart';
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

  void _uploadToDBusAsync(NoteHive note) {
    // Run on a separate isolate/thread without blocking
    unawaited(
      Future(() async {
        try {
          final client = DBusClient.session();
          final object = DBusRemoteObject(
            client,
            name: 'org.mechanix.MxSearch',
            path: DBusObjectPath('/org/mechanix/MxSearch'),
          );
          // Prepare UpsertMetadata as DBusDict entries
          final metadataEntries = <String, DBusValue>{
            'source': const DBusString('notes'),
            'unique_id': DBusString(note.id),
            'uri': DBusString('note://${note.id}'),
            'title': DBusString(note.title),
            'subtitle': const DBusString(''),
            'description': const DBusString("Content of Notes"),
            'keywords': DBusArray(
              DBusSignature('s'),
              [],
              // _extractKeywords(note.title, note.plainText),
            ),
            'icon': const DBusString(''),
            'thumbnail': const DBusString(''),
            'last_modified': DBusUint64(note.updatedAt.millisecondsSinceEpoch),
            'content': DBusString(note.content),
            'source_entry_path': DBusString(''),
          };

          // Create DBusDict from the entries
          final metadataDict = DBusDict.stringVariant(metadataEntries);

          // Call UpsertSources with array of metadata dictionaries
          final res = await object.callMethod(
            'org.mechanix.MxSearch',
            'UpsertSources',
            [
              DBusArray(DBusSignature('a{sv}'), [metadataDict]),
            ],
            replySignature: DBusSignature('b'),
          );
          print("res $res");
          logger.i("Successfully uploaded note metadata to D-Bus: ${note.id}");

          await client.close();
        } catch (e) {
          logger.w("Failed to upload to D-Bus (non-critical): $e");
        }
      }),
    );
  }

  @override
  Future<NoteMetaData> createNote(String title, content, plainText) async {
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

      final previewString = jsonEncode(preview.map((e) => e.toJson()).toList());

      final newNote = NoteHive(
        id: uuid.v4(),
        title: title,
        content: content, // full quill json
        preview: previewString,
        height: calculatedHeight,
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
        plainText: plainText,
      );

      await Hive.box<NoteHive>(Constants.tableName).put(newNote.id, newNote);
      _uploadToDBusAsync(newNote);
      return NoteMetaData(
        id: newNote.id,
        preview: preview,
        height: calculatedHeight,
        title: newNote.title,
        createdAt: newNote.createdAt,
        updatedAt: newNote.updatedAt,
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
      bool isFirstLine = true; // Track if we're on the first line (title)

      void flushLine() {
        // Skip the first line (title)
        if (isFirstLine) {
          isFirstLine = false;
          spans.clear();
          pendingLineAttr = null;
          return;
        }

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
        // Stop early if we've reached maxLines (but keep processing first line)
        if (!isFirstLine && lines.length >= maxLines) break;

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
            if (!isFirstLine && lines.length >= maxLines) break;

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
      if (spans.isNotEmpty && !isFirstLine && lines.length < maxLines) {
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
        final previewString = jsonEncode(
          preview.map((e) => e.toJson()).toList(),
        );

        if (note != null) {
          final updatedNote = NoteHive(
            id: note.id,
            title: title,
            content: content,
            createdAt: note.createdAt,
            preview: previewString,
            updatedAt: DateTime.now(),
            plainText: plainText,
            height: calculatedHeight,
          );

          await notesBox.put(key, updatedNote);
          _uploadToDBusAsync(updatedNote);
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
  Future<List<SearchMetaData>> searchNotes(String searchQuery) async {
    try {
      await ensureHiveConnected();
      final box = Hive.box<NoteHive>(Constants.tableName);

      final query = searchQuery.trim().toLowerCase();

      if (query.isEmpty) {
        logger.i("Empty search query");
        return [];
      }

      final List<SearchMetaData> searchResults = [];

      for (final note in box.values) {
        // Split plainText into lines
        final lines = note.plainText.split('\n');

        // Find the first non-empty line as title
        String titleText = '';
        String titleLower = '';
        int titleLineIndex = -1;

        for (int i = 0; i < lines.length; i++) {
          final trimmedLine = lines[i].trim();
          if (trimmedLine.isNotEmpty) {
            titleText = trimmedLine;
            titleLower = trimmedLine.toLowerCase();
            titleLineIndex = i;
            break;
          }
        }

        final hasTitle = titleText.isNotEmpty;

        // Content is everything after the title line
        String contentText = '';
        String contentLower = '';

        if (hasTitle && titleLineIndex < lines.length - 1) {
          // Join all lines after the title line
          contentText = lines.sublist(titleLineIndex + 1).join('\n').trim();
          contentLower = contentText.toLowerCase();
        }

        // Count occurrences in title and content
        final titleMatches =
            hasTitle ? _countOccurrences(titleLower, query) : 0;
        final contentMatches =
            contentText.isNotEmpty ? _countOccurrences(contentLower, query) : 0;
        final totalCount = titleMatches + contentMatches;

        // Skip if no matches found
        if (totalCount == 0) continue;

        // If matches found in title
        if (titleMatches > 0) {
          final text = _extractMatchingText(titleText, titleLower, query);
          searchResults.add(
            SearchMetaData(
              id: note.id,
              text: text,
              updatedAt: note.updatedAt,
              isTitle: true,
              availableCount: totalCount,
              titleMatchCount: titleMatches,
              contentMatchCount: contentMatches,
            ),
          );
        }

        // If matches found in content (and not already added as title-only result)
        if (contentMatches > 0 && titleMatches == 0) {
          final text = _extractMatchingText(contentText, contentLower, query);

          searchResults.add(
            SearchMetaData(
              id: note.id,
              text: text,
              updatedAt: note.updatedAt,
              isTitle: false,
              availableCount: totalCount,
              titleMatchCount: titleMatches,
              contentMatchCount: contentMatches,
            ),
          );
        }
      }

      // Ranking algorithm:
      // 1. Sort by total match count (descending)
      // 2. If equal, prioritize content matches over title matches
      // 3. If still equal, sort by updated date (most recent first)
      searchResults.sort((a, b) {
        // First: Compare total match counts
        final countComparison = b.availableCount.compareTo(a.availableCount);
        if (countComparison != 0) return countComparison;

        // Second: Prioritize content matches
        // Higher content match count = better rank
        final contentComparison = b.contentMatchCount.compareTo(
          a.contentMatchCount,
        );
        if (contentComparison != 0) return contentComparison;

        // Third: Sort by updated date (most recent first)
        return b.updatedAt.compareTo(a.updatedAt);
      });

      logger.i(
        "Found ${searchResults.length} search results matching '$query'",
      );

      return searchResults;
    } catch (e) {
      logger.e("Search failed: $e");
      return [];
    }
  }

  // Helper method to count occurrences of a substring
  int _countOccurrences(String text, String query) {
    if (query.isEmpty || text.isEmpty) return 0;

    int count = 0;
    int index = 0;
    final queryLength = query.length;

    while ((index = text.indexOf(query, index)) != -1) {
      count++;
      index += queryLength;
    }

    return count;
  }

  // Helper method to extract matching text with context
  String _extractMatchingText(
    String originalText,
    String lowerText,
    String query,
  ) {
    if (originalText.isEmpty || query.isEmpty) return originalText;

    const int leftChars = 10; // Characters to show on left
    const int rightChars = 20; // Characters to show on right
    const String ellipsis = "...";

    // Split into words and clean (removes \n and multiple spaces)
    final words =
        originalText
            .split(RegExp(r'\s+'))
            .where((word) => word.isNotEmpty)
            .toList();

    if (words.isEmpty) return originalText;

    // Reconstruct clean text with single spaces
    final cleanText = words.join(' ');
    final cleanLowerText = cleanText.toLowerCase();

    // Find first occurrence in clean text
    final matchIndex = cleanLowerText.indexOf(query);
    if (matchIndex == -1) return cleanText;

    // Calculate start and end positions
    final matchEnd = matchIndex + query.length;
    final start = (matchIndex - leftChars).clamp(0, cleanText.length);
    final end = (matchEnd + rightChars).clamp(0, cleanText.length);

    // Extract substring
    String result = cleanText.substring(start, end);

    // Add ellipsis
    if (start > 0) result = ellipsis + result;
    if (end < cleanText.length) result = result + ellipsis;

    return result.trim();
  }

  @override
  Future<EditorPayload?> findById(String noteId) async {
    try {
      await ensureHiveConnected();
      final box = Hive.box<NoteHive>(Constants.tableName);
      final note = box.get(noteId);

      if (note != null) {
        logger.i('Note found: $noteId');
        return EditorPayload(id: note.id, content: note.content);
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
