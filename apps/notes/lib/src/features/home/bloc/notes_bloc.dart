import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:intl/intl.dart';
import 'package:logger/logger.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_state.dart';
import 'package:mechanix_notes/src/features/home/data/notes_repository.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';

class NotesBloc extends Bloc<NotesEvent, NotesState> {
  final NotesRepository notesRepository;
  final logger = Logger();
  static const int pageSize = 20;

  bool _hasMorePages = true;
  bool _isLoadingChunk = false;

  // Data sources
  List<NoteMetaData> _allNotes = [];
  List<NoteMetaData> _allPinnedNotes = [];

  int _loadedPinnedCount = 0;
  int _unpinnedNoteIndex = 0;

  NotesBloc({required this.notesRepository}) : super(const NotesState()) {
    on<CreateNotes>(_createNote);
    on<UpdateNotes>(_updateNotes);
    on<DeleteNotes>(_deleteNotes);
    on<UpdateTag>(_updateTag);
    on<PinnedNotes>(_updatePinnedNotes);
    on<SelectNote>(_onSelectNote);
    on<DeselectNote>(_onDeselectNote);
    on<ClearSelection>(_onClearSelection);
    on<SelectAllNotes>(_onSelectAllNotes);
    on<CheckPinnedStatus>(_checkPinnedStatus);
    on<SearchEvent>(_searchNotes);
    on<InitializeNotes>(_onInitializeNotes);
    on<LoadNotes>(_onLoadNotes);
    on<LoadNextChunk>(_onLoadNextChunk);
  }

  // Reset all pagination state
  void _resetPagination() {
    _hasMorePages = true;
    _isLoadingChunk = false;
    _loadedPinnedCount = 0;
    _unpinnedNoteIndex = 0;
  }

  // Called initially when app or page loads
  Future<void> _onInitializeNotes(
    InitializeNotes event,
    Emitter<NotesState> emit,
  ) async {
    logger.i('Initializing notes BLoC');
    _resetPagination();
    add(LoadNotes());
  }

  // Fetch and prepare notes
  Future<void> _onLoadNotes(LoadNotes event, Emitter<NotesState> emit) async {
    try {
      emit(state.copyWith(isLoading: true));

      final notesList = await notesRepository.getAllNotes();

      _allNotes = notesList;
      _allPinnedNotes = _allNotes.where((n) => n.isPinned).toList();

      _resetPagination();
      final initialGroups = _loadFirstChunk();

      emit(
        state.copyWith(
          isLoading: false,
          groupedNotes: initialGroups,
          hasMorePages: _hasMorePages,
        ),
      );
    } catch (e) {
      logger.e('Failed to load notes: $e');
      emit(state.copyWith(isLoading: false, groupedNotes: []));
    }
  }

  // Lazy-load additional notes (on scroll)
  Future<void> _onLoadNextChunk(
    LoadNextChunk event,
    Emitter<NotesState> emit,
  ) async {
    if (_isLoadingChunk || !_hasMorePages) return;

    _isLoadingChunk = true;
    emit(state.copyWith(isLoadingMore: true));

    try {
      final currentGroups = List<GroupedNotes>.from(state.groupedNotes);
      int notesLoaded = 0;

      // Load remaining pinned notes first
      if (_loadedPinnedCount < _allPinnedNotes.length) {
        final remainingPinned = _allPinnedNotes.length - _loadedPinnedCount;
        final toLoad = remainingPinned > pageSize ? pageSize : remainingPinned;

        final nextPinnedChunk =
            _allPinnedNotes.skip(_loadedPinnedCount).take(toLoad).toList();

        final pinnedGroupIndex = currentGroups.indexWhere(
          (g) => g.label == "Pinned Notes",
        );

        if (pinnedGroupIndex != -1) {
          final existingPinned = currentGroups[pinnedGroupIndex].notes;
          currentGroups[pinnedGroupIndex] = GroupedNotes(
            label: "Pinned Notes",
            notes: [...existingPinned, ...nextPinnedChunk],
          );
        } else {
          currentGroups.add(
            GroupedNotes(label: "Pinned Notes", notes: nextPinnedChunk),
          );
        }

        _loadedPinnedCount += toLoad;
        notesLoaded = toLoad;
      }

      // Load unpinned (time-based) notes
      if (notesLoaded < pageSize) {
        final remainingSlots = pageSize - notesLoaded;
        _loadTimeBasedNotes(currentGroups, remainingSlots);
      }

      _hasMorePages = _hasMoreNotesToLoad();

      emit(
        state.copyWith(
          groupedNotes: currentGroups,
          isLoadingMore: false,
          hasMorePages: _hasMorePages,
        ),
      );
    } catch (e) {
      logger.e('Error loading chunk: $e');
      emit(state.copyWith(isLoadingMore: false));
    } finally {
      _isLoadingChunk = false;
    }
  }

  // Build initial chunk (20 items)
  List<GroupedNotes> _loadFirstChunk() {
    final List<GroupedNotes> initialGroups = [];
    int notesLoaded = 0;

    if (_allPinnedNotes.isNotEmpty) {
      final pinnedToLoad =
          _allPinnedNotes.length > pageSize ? pageSize : _allPinnedNotes.length;

      final pinnedChunk = _allPinnedNotes.take(pinnedToLoad).toList();
      _loadedPinnedCount = pinnedChunk.length;
      notesLoaded = pinnedChunk.length;

      initialGroups.add(
        GroupedNotes(label: "Pinned Notes", notes: pinnedChunk),
      );
    }

    if (notesLoaded < pageSize) {
      final remainingSlots = pageSize - notesLoaded;
      _loadTimeBasedNotes(initialGroups, remainingSlots);
    }
    _hasMorePages = _hasMoreNotesToLoad();

    return initialGroups;
  }

  // Group notes by time
  void _loadTimeBasedNotes(List<GroupedNotes> currentGroups, int count) {
    int loaded = 0;

    while (loaded < count && _unpinnedNoteIndex < _allNotes.length) {
      final note = _allNotes[_unpinnedNoteIndex];
      _unpinnedNoteIndex++;

      final groupLabel = _getTimeLabelForNote(note);

      final existingIndex = currentGroups.indexWhere(
        (g) => g.label == groupLabel,
      );

      if (existingIndex != -1) {
        final updatedNotes = [...currentGroups[existingIndex].notes, note];
        currentGroups[existingIndex] = GroupedNotes(
          label: groupLabel,
          notes: updatedNotes,
        );
      } else {
        currentGroups.add(GroupedNotes(label: groupLabel, notes: [note]));
      }

      loaded++;
    }
  }

  // Check if there are more notes to load
  bool _hasMoreNotesToLoad() {
    if (_loadedPinnedCount < _allPinnedNotes.length) return true;
    if (_unpinnedNoteIndex < _allNotes.length) return true;
    return false;
  }

  // Label note based on date
  String _getTimeLabelForNote(NoteMetaData note) {
    final now = DateTime.now();
    final today = DateTime(now.year, now.month, now.day);
    final yesterday = today.subtract(const Duration(days: 1));
    final thisWeekStart = today.subtract(Duration(days: now.weekday - 1));
    final lastWeekStart = thisWeekStart.subtract(const Duration(days: 7));
    final thisMonthStart = DateTime(now.year, now.month, 1);
    final lastMonthStart = DateTime(now.year, now.month - 1, 1);

    final dateOnly = DateTime(
      note.updatedAt.year,
      note.updatedAt.month,
      note.updatedAt.day,
    );

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

    return DateFormat("MMMM yyyy").format(note.updatedAt);
  }

  // ---- CRUD + UI State Actions ----

  Future<void> _createNote(CreateNotes event, Emitter<NotesState> emit) async {
    try {
      await notesRepository.createNote(
        event.title,
        event.content,
        event.plainText,
        event.isPinned,
        event.tag,
      );
      add(LoadNotes());
    } catch (e) {
      logger.e('Note create failed: $e');
    }
  }

  Future<void> _updateNotes(UpdateNotes event, Emitter<NotesState> emit) async {
    try {
      await notesRepository.updateNote(
        event.title,
        event.content,
        event.id,
        event.plainText,
        event.isPinned,
        event.tag,
      );
      add(LoadNotes());
    } catch (e) {
      logger.e('Note update failed: $e');
    }
  }

  Future<void> _deleteNotes(DeleteNotes event, Emitter<NotesState> emit) async {
    try {
      await notesRepository.deleteNote(event.deleteIds);
      
      // Local update: Remove deleted notes from groups
      _updateGroupsAfterDelete(event.deleteIds, emit);
      
      add(SearchEvent(''));
    } catch (e) {
      logger.e('Note delete failed: $e');
    }
  }

  // Helper: Update groups locally after deletion
  void _updateGroupsAfterDelete(List<String> deletedIds, Emitter<NotesState> emit) {
    final deletedSet = deletedIds.toSet();
    final updatedGroups = <GroupedNotes>[];

    // Update _allNotes and _allPinnedNotes
    _allNotes.removeWhere((note) => deletedSet.contains(note.id));
    _allPinnedNotes.removeWhere((note) => deletedSet.contains(note.id));

    // Adjust pagination counters for deleted notes
    int deletedFromPinned = 0;
    int deletedFromUnpinned = 0;

    for (final group in state.groupedNotes) {
      for (final note in group.notes) {
        if (deletedSet.contains(note.id)) {
          if (group.label == "Pinned Notes") {
            deletedFromPinned++;
          } else {
            deletedFromUnpinned++;
          }
        }
      }
    }

    // Adjust loaded counts
    _loadedPinnedCount = (_loadedPinnedCount - deletedFromPinned).clamp(0, _allPinnedNotes.length);
    _unpinnedNoteIndex = (_unpinnedNoteIndex - deletedFromUnpinned).clamp(0, _allNotes.length);

    // Remove deleted notes from each group
    for (final group in state.groupedNotes) {
      final filteredNotes = group.notes
          .where((note) => !deletedSet.contains(note.id))
          .toList();

      // Only keep groups that still have notes
      if (filteredNotes.isNotEmpty) {
        updatedGroups.add(GroupedNotes(
          label: group.label,
          notes: filteredNotes,
        ));
      }
    }

    // Recalculate if there are more pages
    _hasMorePages = _hasMoreNotesToLoad();

    emit(state.copyWith(
      groupedNotes: updatedGroups,
      hasMorePages: _hasMorePages,
    ));
  }

  Future<void> _updateTag(UpdateTag event, Emitter<NotesState> emit) async {
    try {
      await notesRepository.updateTag(event.noteIds, event.tag);
      
      // Local update: Update tag in groups
      // _updateGroupsAfterTagChange(event.noteIds, event.tag, emit);
    } catch (e) {
      logger.e('Tag update failed: $e');
    }
  }

  // Helper: Update tags locally
  // void _updateGroupsAfterTagChange(
  //   List<String> noteIds,
  //   String tag,
  //   Emitter<NotesState> emit,
  // ) {
  //   final noteIdSet = noteIds.toSet();
  //   final updatedGroups = <GroupedNotes>[];

  //   // Update _allNotes
  //   for (var i = 0; i < _allNotes.length; i++) {
  //     if (noteIdSet.contains(_allNotes[i].id)) {
  //       final note = _allNotes[i];
  //       _allNotes[i] = NoteMetaData(
  //         id: note.id,
  //         title: note.title,
  //         isPinned: note.isPinned,
  //         createdAt: note.createdAt,
  //         updatedAt: note.updatedAt,
  //       );
  //     }
  //   }

  //   // Update _allPinnedNotes
  //   for (var i = 0; i < _allPinnedNotes.length; i++) {
  //     if (noteIdSet.contains(_allPinnedNotes[i].id)) {
  //       final note = _allPinnedNotes[i];
  //       _allPinnedNotes[i] = NoteMetaData(
  //         id: note.id,
  //         title: note.title,
          
  //         isPinned: note.isPinned,
  //         createdAt: note.createdAt,
  //         updatedAt: note.updatedAt,
  //       );
  //     }
  //   }

  //   // Update groups
  //   for (final group in state.groupedNotes) {
  //     final updatedNotes = group.notes.map((note) {
  //       if (noteIdSet.contains(note.id)) {
  //         return NoteMetaData(
  //           id: note.id,
  //           title: note.title,
          
  //           isPinned: note.isPinned,
  //           createdAt: note.createdAt,
  //           updatedAt: note.updatedAt,
  //         );
  //       }
  //       return note;
  //     }).toList();

  //     updatedGroups.add(GroupedNotes(
  //       label: group.label,
  //       notes: updatedNotes,
  //     ));
  //   }

  //   emit(state.copyWith(groupedNotes: updatedGroups));
  // }

  Future<void> _updatePinnedNotes(
    PinnedNotes event,
    Emitter<NotesState> emit,
  ) async {
    try {
      await notesRepository.pinnedNotes(event.noteIds, event.isPinned);
      
      // Local update: Move notes between pinned/unpinned groups
      _updateGroupsAfterPinChange(event.noteIds, event.isPinned, emit);
      
      add(ClearSelection());
    } catch (e) {
      logger.e('Pinned notes update failed: $e');
    }
  }

  // Helper: Update groups locally after pin/unpin
  void _updateGroupsAfterPinChange(
    List<String> noteIds,
    bool isPinned,
    Emitter<NotesState> emit,
  ) {
    final noteIdSet = noteIds.toSet();
    final updatedGroups = <GroupedNotes>[];
    final notesToAddToPinned = <NoteMetaData>[];

    // Track notes being moved from loaded sections
    int movedFromPinnedSection = 0;
    int movedFromUnpinnedSection = 0;

    // Update _allNotes
    for (var i = 0; i < _allNotes.length; i++) {
      if (noteIdSet.contains(_allNotes[i].id)) {
        final note = _allNotes[i];
        _allNotes[i] = NoteMetaData(
          id: note.id,
          title: note.title,
          isPinned: isPinned,
          createdAt: note.createdAt,
          updatedAt: note.updatedAt,
        );
      }
    }

    // Update _allPinnedNotes and track changes
    if (isPinned) {
      // Add to pinned
      final newPinned = _allNotes.where((n) => noteIdSet.contains(n.id)).toList();
      _allPinnedNotes.addAll(newPinned);
    } else {
      // Remove from pinned
      final removedCount = _allPinnedNotes.where((n) => noteIdSet.contains(n.id)).length;
      _allPinnedNotes.removeWhere((n) => noteIdSet.contains(n.id));
      
      // If we removed pinned notes that were already loaded, adjust the counter
      if (removedCount > 0) {
        _loadedPinnedCount = (_loadedPinnedCount - removedCount).clamp(0, _allPinnedNotes.length);
      }
    }

    // Process groups
    for (final group in state.groupedNotes) {
      if (isPinned) {
        // When pinning: Keep notes in their original location AND add to pinned section
        final updatedNotes = group.notes.map((note) {
          if (noteIdSet.contains(note.id)) {
            // Track where notes are coming from
            if (group.label == "Pinned Notes") {
              movedFromPinnedSection++;
            } else {
              movedFromUnpinnedSection++;
              // Collect notes to add to pinned section
              notesToAddToPinned.add(NoteMetaData(
                id: note.id,
                title: note.title,
                isPinned: true,
                createdAt: note.createdAt,
                updatedAt: note.updatedAt,
              ));
            }
            
            // Update the note in its current location with isPinned = true
            return NoteMetaData(
              id: note.id,
              title: note.title,
              isPinned: true,
              createdAt: note.createdAt,
              updatedAt: note.updatedAt,
            );
          }
          return note;
        }).toList();

        updatedGroups.add(GroupedNotes(
          label: group.label,
          notes: updatedNotes,
        ));
      } else {
        // When unpinning: Remove from pinned section, keep in time-based sections
        if (group.label == "Pinned Notes") {
          // Remove unpinned notes from Pinned Notes section
          final remainingPinned = group.notes
              .where((note) => !noteIdSet.contains(note.id))
              .toList();
          
          if (remainingPinned.isNotEmpty) {
            updatedGroups.add(GroupedNotes(
              label: group.label,
              notes: remainingPinned,
            ));
          }
          
          movedFromPinnedSection += group.notes.length - remainingPinned.length;
        } else {
          // Update isPinned status in time-based groups
          final updatedNotes = group.notes.map((note) {
            if (noteIdSet.contains(note.id)) {
              return NoteMetaData(
                id: note.id,
                title: note.title,
                isPinned: false,
                createdAt: note.createdAt,
                updatedAt: note.updatedAt,
              );
            }
            return note;
          }).toList();

          updatedGroups.add(GroupedNotes(
            label: group.label,
            notes: updatedNotes,
          ));
        }
      }
    }

    // Adjust pagination counters
    if (isPinned) {
      // Notes moved to pinned section
      _loadedPinnedCount += movedFromUnpinnedSection;
      // Don't adjust _unpinnedNoteIndex since notes stay in their time-based location
    }

    // Add newly pinned notes to the Pinned Notes section
    if (isPinned && notesToAddToPinned.isNotEmpty) {
      final pinnedGroupIndex = updatedGroups.indexWhere(
        (g) => g.label == "Pinned Notes",
      );

      if (pinnedGroupIndex != -1) {
        final existingPinned = updatedGroups[pinnedGroupIndex].notes;
        updatedGroups[pinnedGroupIndex] = GroupedNotes(
          label: "Pinned Notes",
          notes: [...notesToAddToPinned, ...existingPinned],
        );
      } else {
        updatedGroups.insert(
          0,
          GroupedNotes(label: "Pinned Notes", notes: notesToAddToPinned),
        );
      }
    }

    // Recalculate if there are more pages
    _hasMorePages = _hasMoreNotesToLoad();

    emit(state.copyWith(
      groupedNotes: updatedGroups,
      hasMorePages: _hasMorePages,
    ));
  }

  void _onSelectNote(SelectNote event, Emitter<NotesState> emit) {
    final newSelected = List<String>.from(state.selectedNoteIds)
      ..add(event.noteId);
    emit(state.copyWith(selectedNoteIds: newSelected, isSelectionMode: true));
    add(CheckPinnedStatus());
  }

  void _onDeselectNote(DeselectNote event, Emitter<NotesState> emit) {
    final newSelected = List<String>.from(state.selectedNoteIds)
      ..remove(event.noteId);
    final mode = newSelected.isNotEmpty;
    emit(state.copyWith(selectedNoteIds: newSelected, isSelectionMode: mode));
    add(CheckPinnedStatus());
  }

  void _onClearSelection(ClearSelection event, Emitter<NotesState> emit) {
    emit(state.copyWith(selectedNoteIds: [], isSelectionMode: false));
  }

  void _onSelectAllNotes(SelectAllNotes event, Emitter<NotesState> emit) {
    final noteIds = <String>{};

    for (final group in state.groupedNotes) {
      noteIds.addAll(group.notes.map((e) => e.id));
    }

    emit(
      state.copyWith(selectedNoteIds: noteIds.toList(), isSelectionMode: true),
    );
    add(CheckPinnedStatus());
  }

  void _checkPinnedStatus(CheckPinnedStatus event, Emitter<NotesState> emit) {
    // Extract pinned note IDs for quick lookup
    final pinnedNoteIds = _allPinnedNotes.map((n) => n.id).toSet();

    // Check if all selected notes are pinned
    final isPinnedSelected = state.selectedNoteIds.every(
      (id) => pinnedNoteIds.contains(id),
    );

    emit(state.copyWith(isPinnedSelected: isPinnedSelected));
  }

  Future<void> _searchNotes(SearchEvent event, Emitter<NotesState> emit) async {
    logger.i("Search started: ${event.searchQuery}");

    final query = event.searchQuery.trim().toLowerCase();
    if (query.isNotEmpty) {
      final searchedNotes = await notesRepository.searchNotes(query);
      emit(state.copyWith(searchedNotes: searchedNotes));
    } else {
      emit(state.copyWith(searchedNotes: []));
    }
  }
}