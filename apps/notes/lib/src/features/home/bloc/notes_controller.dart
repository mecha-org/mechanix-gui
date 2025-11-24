// import 'package:flutter/foundation.dart';
// import 'package:intl/intl.dart';
// import 'package:logger/logger.dart';
// import 'package:mechanix_notes/src/features/home/data/notes_repository.dart';
// import 'package:mechanix_notes/src/features/home/models/notes_model.dart';

// class NotesController extends ChangeNotifier {
//   final NotesRepository notesRepository;
//   final logger = Logger();
//   NotesController({required this.notesRepository});
//   static const int pageSize = 20;
//   int _currentPage = 1;
//   bool _hasMorePages = true;
//   bool _isLoadingChunk = false;

//   List<NoteMetaData> _allNotes = [];
//   List<NoteMetaData> _allPinnedNotes = [];

//   // Tracks how many pinned notes have been loaded so far
//   int _loadedPinnedCount = 0;
//   // Tracks position in unpinned notes for time-based grouping
//   int _unpinnedNoteIndex = 0;

//   // Observable state
//   final ValueNotifier<List<GroupedNotes>> groupedNotes = ValueNotifier([]);
//   final ValueNotifier<List<String>> selectedNoteIds = ValueNotifier([]);
//   final ValueNotifier<bool> isLoading = ValueNotifier(false);
//   final ValueNotifier<bool> isLoadingMore = ValueNotifier(false);
//   final ValueNotifier<bool> hasMorePages = ValueNotifier(true);

//   /// Reset pagination state
//   void _resetPagination() {
//     _currentPage = 1;
//     _hasMorePages = true;
//     _isLoadingChunk = false;
//     _loadedPinnedCount = 0;
//     _unpinnedNoteIndex = 0;
//     groupedNotes.value = [];
//   }

//   Future<void> initialize() async {
//     try {
//       logger.i('Initializing notes controller');
//       _resetPagination();
//       await loadNotes();
//       logger.i('Notes controller initialized successfully');
//     } catch (e) {
//       logger.e('Failed to initialize: $e');
//     }
//   }

//   Future<void> loadNotes() async {
//     try {
//       logger.i('Loading notes requested');
//       isLoading.value = true;
//       final stopwatch = Stopwatch()..start();
//       final notesList = await notesRepository.getAllNotes();
//       // final notesList = await compute(
//       //   _processNotesInBackground,
//       //   notesRepository,
//       // );
//       // _processNotesInBackground
//       stopwatch.stop();
//       // print('Notes loaded in ${stopwatch.elapsedMilliseconds} ms');

//       // Store all notes (already sorted by impl)
//       _allNotes = notesList;

//       // Separate pinned notes (maintain order from _allNotes)
//       _allPinnedNotes = _allNotes.where((note) => note.isPinned).toList();

//       // Reset pagination
//       _resetPagination();

//       // Load first chunk
//       _loadFirstChunk();

//       logger.i(
//         'Notes loaded successfully - Total: ${_allNotes.length}, '
//         'Pinned: ${_allPinnedNotes.length}',
//       );
//     } catch (e) {
//       logger.e('Failed to load notes: $e');
//       groupedNotes.value = [];
//     } finally {
//       isLoading.value = false;
//     }
//   }

//   /// Load first chunk of notes (always 20 notes)
//   void _loadFirstChunk() {
//     final List<GroupedNotes> initialGroups = [];
//     int notesLoaded = 0;

//     // Load pinned notes first (up to pageSize)
//     if (_allPinnedNotes.isNotEmpty) {
//       final pinnedToLoad =
//           _allPinnedNotes.length > pageSize ? pageSize : _allPinnedNotes.length;

//       final pinnedChunk = _allPinnedNotes.take(pinnedToLoad).toList();
//       _loadedPinnedCount = pinnedChunk.length;
//       notesLoaded = pinnedChunk.length;

//       initialGroups.add(
//         GroupedNotes(label: "Pinned Notes", notes: pinnedChunk),
//       );

//       logger.i('Loaded $pinnedToLoad pinned notes');
//     }

//     // Fill remaining slots with time-based grouped notes
//     if (notesLoaded < pageSize) {
//       final remainingSlots = pageSize - notesLoaded;
//       _loadTimeBasedNotes(initialGroups, remainingSlots);
//     }

//     groupedNotes.value = initialGroups;

//     // Check if there are more pages
//     _hasMorePages = _hasMoreNotesToLoad();
//     hasMorePages.value = _hasMorePages;
//     _currentPage = 2;

//     logger.i('First chunk loaded: $notesLoaded notes, hasMore=$_hasMorePages');
//   }

//   /// Load next chunk of notes (infinite scroll)
//   Future<void> loadNextChunk() async {
//     if (_isLoadingChunk || !_hasMorePages) {
//       logger.i(
//         'Skipping chunk: isLoading=$_isLoadingChunk, hasMore=$_hasMorePages',
//       );
//       return;
//     }

//     _isLoadingChunk = true;
//     isLoadingMore.value = true;

//     try {
//       logger.i(
//         'Loading chunk: page=$_currentPage, '
//         'loadedPinned=$_loadedPinnedCount/${_allPinnedNotes.length}, '
//         'unpinnedIndex=$_unpinnedNoteIndex/${_allNotes.length}',
//       );

//       final currentGroups = List<GroupedNotes>.from(groupedNotes.value);
//       int notesLoaded = 0;

//       // Phase 1: Load more pinned notes if available
//       if (_loadedPinnedCount < _allPinnedNotes.length) {
//         final remainingPinned = _allPinnedNotes.length - _loadedPinnedCount;
//         final toLoad = remainingPinned > pageSize ? pageSize : remainingPinned;

//         final nextPinnedChunk =
//             _allPinnedNotes.skip(_loadedPinnedCount).take(toLoad).toList();

//         // Find and update pinned group
//         final pinnedGroupIndex = currentGroups.indexWhere(
//           (g) => g.label == "Pinned Notes",
//         );

//         if (pinnedGroupIndex != -1) {
//           final existingPinned = currentGroups[pinnedGroupIndex].notes;
//           currentGroups[pinnedGroupIndex] = GroupedNotes(
//             label: "Pinned Notes",
//             notes: [...existingPinned, ...nextPinnedChunk],
//           );
//         }

//         _loadedPinnedCount += toLoad;
//         notesLoaded = toLoad;
//         logger.i(
//           'Loaded $toLoad more pinned notes (total: $_loadedPinnedCount)',
//         );
//       }

//       // Fill remaining slots with time-based grouped notes
//       if (notesLoaded < pageSize) {
//         final remainingSlots = pageSize - notesLoaded;
//         _loadTimeBasedNotes(currentGroups, remainingSlots);
//       }

//       groupedNotes.value = currentGroups;

//       // Check if more pages exist
//       _hasMorePages = _hasMoreNotesToLoad();
//       hasMorePages.value = _hasMorePages;
//       _currentPage++;

//       logger.i('Chunk loaded successfully, hasMore=$_hasMorePages');
//     } catch (e) {
//       logger.e('Error loading chunk: $e');
//     } finally {
//       _isLoadingChunk = false;
//       isLoadingMore.value = false;
//     }
//   }

//   /// Load time-based grouped notes
//   void _loadTimeBasedNotes(List<GroupedNotes> currentGroups, int count) {
//     int loaded = 0;

//     while (loaded < count && _unpinnedNoteIndex < _allNotes.length) {
//       final note = _allNotes[_unpinnedNoteIndex];
//       _unpinnedNoteIndex++;

//       // Group this note by time
//       final groupLabel = _getTimeLabelForNote(note);

//       // Find existing group or create new one
//       final existingGroupIndex = currentGroups.indexWhere(
//         (g) => g.label == groupLabel,
//       );

//       if (existingGroupIndex != -1) {
//         // Append to existing group
//         final existingNotes = currentGroups[existingGroupIndex].notes;
//         currentGroups[existingGroupIndex] = GroupedNotes(
//           label: groupLabel,
//           notes: [...existingNotes, note],
//         );
//       } else {
//         // Create new group
//         currentGroups.add(GroupedNotes(label: groupLabel, notes: [note]));
//       }

//       loaded++;
//     }

//     logger.i(
//       'Loaded $loaded time-based notes (index now: $_unpinnedNoteIndex)',
//     );
//   }

//   /// Get time label for a single note
//   String _getTimeLabelForNote(NoteMetaData note) {
//     final now = DateTime.now();
//     final today = DateTime(now.year, now.month, now.day);
//     final yesterday = today.subtract(const Duration(days: 1));
//     final thisWeekStart = today.subtract(Duration(days: now.weekday - 1));
//     final lastWeekStart = thisWeekStart.subtract(const Duration(days: 7));
//     final thisMonthStart = DateTime(now.year, now.month, 1);
//     final lastMonthStart = DateTime(now.year, now.month - 1, 1);

//     final dateOnly = DateTime(
//       note.updatedAt.year,
//       note.updatedAt.month,
//       note.updatedAt.day,
//     );

//     return _getTimeLabel(
//       dateOnly,
//       today,
//       yesterday,
//       thisWeekStart,
//       lastWeekStart,
//       thisMonthStart,
//       lastMonthStart,
//       note.updatedAt,
//     );
//   }

//   /// Check if there are more notes to load
//   bool _hasMoreNotesToLoad() {
//     // More pinned notes to load
//     if (_loadedPinnedCount < _allPinnedNotes.length) {
//       return true;
//     }

//     // More notes to load for time-based groups
//     if (_unpinnedNoteIndex < _allNotes.length) {
//       return true;
//     }

//     return false;
//   }

//   String _getTimeLabel(
//     DateTime dateOnly,
//     DateTime today,
//     DateTime yesterday,
//     DateTime thisWeekStart,
//     DateTime lastWeekStart,
//     DateTime thisMonthStart,
//     DateTime lastMonthStart,
//     DateTime originalDate,
//   ) {
//     if (dateOnly == today) return "Today";
//     if (dateOnly == yesterday) return "Yesterday";

//     if (dateOnly.isAfter(thisWeekStart.subtract(const Duration(days: 1))) &&
//         dateOnly.isBefore(today)) {
//       return "This Week";
//     }

//     if (dateOnly.isAfter(lastWeekStart.subtract(const Duration(days: 1))) &&
//         dateOnly.isBefore(thisWeekStart)) {
//       return "Last Week";
//     }

//     if (dateOnly.isAfter(thisMonthStart.subtract(const Duration(days: 1))) &&
//         dateOnly.isBefore(today)) {
//       return "This Month";
//     }

//     if (dateOnly.isAfter(lastMonthStart.subtract(const Duration(days: 1))) &&
//         dateOnly.isBefore(thisMonthStart)) {
//       return "Last Month";
//     }

//     return DateFormat("MMMM yyyy").format(originalDate);
//   }

//   void selectAllNotes() {
//     final noteIds = <String>{};

//     for (final group in groupedNotes.value) {
//       noteIds.addAll(group.notes.map((e) => e.id));
//     }

//     selectedNoteIds.value = noteIds.toList();
//   }
// }
