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

  // Main list pagination
  bool _hasMorePages = true;
  bool _isLoadingChunk = false;
  List<NoteMetaData> _allNotes = [];
  int _noteIndex = 0;

  // Search pagination state
  bool _hasMoreSearchResults = true;
  bool _isLoadingSearchChunk = false;
  String _currentSearchQuery = '';
  int _searchLoadedCount = 0;
  List<SearchMetaData> _allSearchResults = [];

  NotesBloc({required this.notesRepository}) : super(const NotesState()) {
    on<CreateNotes>(_createNote);
    on<UpdateNotes>(_updateNotes);
    on<DeleteNotes>(_deleteNotes);
    on<SelectNote>(_onSelectNote);
    on<DeselectNote>(_onDeselectNote);
    on<ClearSelection>(_onClearSelection);
    on<SelectAllNotes>(_onSelectAllNotes);
    on<SearchEvent>(_searchNotes);
    on<LoadNextSearchChunk>(_onLoadNextSearchChunk);
    on<ClearSearch>(_onClearSearch);
    on<InitializeNotes>(_onInitializeNotes);
    on<LoadNotes>(_onLoadNotes);
    on<LoadNextChunk>(_onLoadNextChunk);
    on<DragUpdate>(_dragUpdate);
    on<SearchPageToggle>(_onSearchPageToggle);
  }

  // Reset all pagination state
  void _resetPagination() {
    _hasMorePages = true;
    _isLoadingChunk = false;
    _noteIndex = 0;
  }

  // Reset search pagination state
  void _resetSearchPagination() {
    _hasMoreSearchResults = true;
    _isLoadingSearchChunk = false;
    _searchLoadedCount = 0;
    _currentSearchQuery = '';
    _allSearchResults = [];
  }

  // Called initially when app or page loads
  void initialise() {
    _resetPagination();
    _resetSearchPagination();
    add(LoadNotes());
  }

  Future<void> _onInitializeNotes(
    InitializeNotes event,
    Emitter<NotesState> emit,
  ) async {
    logger.i('Initializing notes BLoC');
    _resetPagination();
    _resetSearchPagination();

    add(LoadNotes());
  }

  // Fetch and prepare notes
  Future<void> _onLoadNotes(LoadNotes event, Emitter<NotesState> emit) async {
    try {
      emit(state.copyWith(isLoading: true));

      final notesList = await notesRepository.getAllNotes();
      await Future.delayed(const Duration(seconds: 1));
      _allNotes = notesList;

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

      // Load time-based notes
      _loadTimeBasedNotes(currentGroups, pageSize);

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

  // ============ SEARCH PAGINATION METHODS ============

  Future<void> _searchNotes(SearchEvent event, Emitter<NotesState> emit) async {
    logger.i("Search started: ${event.searchQuery}");

    final query = event.searchQuery.trim().toLowerCase();

    // If query is empty, clear search
    if (query.isEmpty) {
      _resetSearchPagination();
      emit(
        state.copyWith(
          searchedNotes: [],
          hasMoreSearchResults: false,
          isSearchMode: false,
          isSearchLoading: false,
        ),
      );
      return;
    }

    // If it's a new search query, reset pagination and fetch results
    final isNewQuery = query != _currentSearchQuery;
    if (isNewQuery) {
      _resetSearchPagination();
      _currentSearchQuery = query;
    }

    try {
      emit(state.copyWith(isSearchLoading: true, isSearchMode: true));

      // Fetch all search results from repository (only on new query)
      if (isNewQuery) {
        _allSearchResults = await notesRepository.searchNotes(query);
      }

      // Load first page
      _searchLoadedCount = 0;
      final firstPage = _getSearchChunk(pageSize);

      emit(
        state.copyWith(
          searchedNotes: firstPage,
          isSearchLoading: false,
          hasMoreSearchResults: _searchLoadedCount < _allSearchResults.length,
          isSearchMode: true,
        ),
      );
    } catch (e) {
      logger.e('Search failed: $e');
      emit(
        state.copyWith(
          isSearchLoading: false,
          searchedNotes: [],
          hasMoreSearchResults: false,
        ),
      );
    }
  }

  Future<void> _onLoadNextSearchChunk(
    LoadNextSearchChunk event,
    Emitter<NotesState> emit,
  ) async {
    // Prevent multiple simultaneous loads
    if (_isLoadingSearchChunk || !_hasMoreSearchResults) return;

    // Ensure we have an active search query
    if (_currentSearchQuery.isEmpty || _allSearchResults.isEmpty) return;

    _isLoadingSearchChunk = true;
    emit(state.copyWith(isSearchLoadingMore: true));

    try {
      // Simulate slight delay for smoother UX (optional)
      await Future.delayed(const Duration(milliseconds: 100));

      // Get next chunk of search results
      final nextChunk = _getSearchChunk(pageSize);

      // Append new results to existing ones
      final updatedSearchResults = [...state.searchedNotes, ...nextChunk];

      emit(
        state.copyWith(
          searchedNotes: updatedSearchResults,
          isSearchLoadingMore: false,
          hasMoreSearchResults: _searchLoadedCount < _allSearchResults.length,
        ),
      );
    } catch (e) {
      logger.e('Error loading search chunk: $e');
      emit(state.copyWith(isSearchLoadingMore: false));
    } finally {
      _isLoadingSearchChunk = false;
    }
  }

  // Helper: Get next chunk of search results
  List<SearchMetaData> _getSearchChunk(int count) {
    if (_searchLoadedCount >= _allSearchResults.length) {
      _hasMoreSearchResults = false;
      return [];
    }

    final endIndex = (_searchLoadedCount + count).clamp(
      0,
      _allSearchResults.length,
    );

    final chunk = _allSearchResults.sublist(_searchLoadedCount, endIndex);
    _searchLoadedCount = endIndex;
    _hasMoreSearchResults = _searchLoadedCount < _allSearchResults.length;

    return chunk;
  }

  void _onClearSearch(ClearSearch event, Emitter<NotesState> emit) {
    _resetSearchPagination();
    emit(
      state.copyWith(
        searchedNotes: [],
        hasMoreSearchResults: false,
        isSearchMode: false,
        isSearchLoading: false,
        isSearchLoadingMore: false,
      ),
    );
  }

  // ============ EXISTING METHODS ============

  List<GroupedNotes> _loadFirstChunk() {
    final List<GroupedNotes> initialGroups = [];

    _loadTimeBasedNotes(initialGroups, pageSize);
    _hasMorePages = _hasMoreNotesToLoad();

    return initialGroups;
  }

  // Group notes by time
  void _loadTimeBasedNotes(List<GroupedNotes> currentGroups, int count) {
    int loaded = 0;

    while (loaded < count && _noteIndex < _allNotes.length) {
      final note = _allNotes[_noteIndex];
      _noteIndex++;

      final groupLabel = _getTimeLabelForNote(note);

      final existingIndex = currentGroups.indexWhere(
        (g) => g.label == groupLabel,
      );

      if (existingIndex != -1) {
        final updatedNotes = [...currentGroups[existingIndex].notes, note];
        currentGroups[existingIndex] = GroupedNotes(
          label: groupLabel,
          notes: updatedNotes,
          height: updatedNotes.map((a) => a.height).reduce((a, b) => a + b),
        );
      } else {
        currentGroups.add(
          GroupedNotes(
            label: groupLabel,
            notes: [note],
            height: [note].map((a) => a.height).reduce((a, b) => a + b),
          ),
        );
      }

      loaded++;
    }
  }

  // Check if there are more notes to load
  bool _hasMoreNotesToLoad() {
    if (_noteIndex < _allNotes.length) return true;
    return false;
  }

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
    if (now.difference(note.updatedAt).inHours < 3) {
      return "Recent";
    }
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
      final NoteMetaData note = await notesRepository.createNote(
        event.title,
        event.content,
        event.plainText,
      );

      // Add to _allNotes
      _allNotes.insert(0, note);

      final updatedGroups = List<GroupedNotes>.from(state.groupedNotes);

      // Add to time-based group
      final groupLabel = _getTimeLabelForNote(note);

      final existingIndex = updatedGroups.indexWhere(
        (g) => g.label == groupLabel,
      );

      if (existingIndex != -1) {
        final updatedNotes = [note, ...updatedGroups[existingIndex].notes];
        updatedGroups[existingIndex] = GroupedNotes(
          label: groupLabel,
          notes: updatedNotes,
          height: updatedNotes.map((a) => a.height).reduce((a, b) => a + b),
        );
      } else {
        updatedGroups.insert(
          0,
          GroupedNotes(
            label: groupLabel,
            notes: [note],
            height: [note].map((a) => a.height).reduce((a, b) => a + b),
          ),
        );
      }

      // Increment note index
      _noteIndex++;

      emit(state.copyWith(groupedNotes: updatedGroups));

      // If in search mode, refresh search results
      if (state.isSearchMode && _currentSearchQuery.isNotEmpty) {
        add(SearchEvent(_currentSearchQuery));
      }
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
      );
      add(LoadNotes());

      // If in search mode, refresh search results
      if (state.isSearchMode && _currentSearchQuery.isNotEmpty) {
        add(SearchEvent(_currentSearchQuery));
      }
    } catch (e) {
      logger.e("$e");
    }
  }

  Future<void> _deleteNotes(DeleteNotes event, Emitter<NotesState> emit) async {
    try {
      final List<String> ids =
          event.deleteIds.isNotEmpty ? event.deleteIds : state.selectedNoteIds;

      await notesRepository.deleteNote(ids);
      _updateGroupsAfterDelete(ids, emit);

      // If in search mode, also update search results locally
      if (state.isSearchMode && _currentSearchQuery.isNotEmpty) {
        final deletedSet = ids.toSet();

        // Remove from cached search results
        _allSearchResults.removeWhere((note) => deletedSet.contains(note.id));

        // Remove from displayed search results
        final updatedSearchResults =
            state.searchedNotes
                .where((note) => !deletedSet.contains(note.id))
                .toList();

        // Adjust loaded count
        _searchLoadedCount = updatedSearchResults.length;
        _hasMoreSearchResults = _searchLoadedCount < _allSearchResults.length;

        emit(
          state.copyWith(
            searchedNotes: updatedSearchResults,
            hasMoreSearchResults: _hasMoreSearchResults,
          ),
        );
      }
    } catch (e) {
      logger.e('Note delete failed: $e');
    }
  }

  // Helper: Update groups locally after deletion
  void _updateGroupsAfterDelete(
    List<String> deletedIds,
    Emitter<NotesState> emit,
  ) {
    final deletedSet = deletedIds.toSet();
    final updatedGroups = <GroupedNotes>[];

    // Update _allNotes
    _allNotes.removeWhere((note) => deletedSet.contains(note.id));

    // Adjust pagination counters for deleted notes
    int deletedFromLoaded = 0;

    for (final group in state.groupedNotes) {
      for (final note in group.notes) {
        if (deletedSet.contains(note.id)) {
          deletedFromLoaded++;
        }
      }
    }

    // Adjust loaded count
    _noteIndex = (_noteIndex - deletedFromLoaded).clamp(0, _allNotes.length);

    // Remove deleted notes from each group
    for (final group in state.groupedNotes) {
      final filteredNotes =
          group.notes.where((note) => !deletedSet.contains(note.id)).toList();

      if (filteredNotes.isNotEmpty) {
        updatedGroups.add(
          GroupedNotes(
            label: group.label,
            notes: filteredNotes,
            height: filteredNotes.map((a) => a.height).reduce((a, b) => a + b),
          ),
        );
      }
    }

    // **FIX: Load more notes if we deleted everything that was displayed**
    if (updatedGroups.isEmpty && _noteIndex < _allNotes.length) {
      _loadTimeBasedNotes(updatedGroups, pageSize);
    }

    _hasMorePages = _hasMoreNotesToLoad();

    emit(
      state.copyWith(groupedNotes: updatedGroups, hasMorePages: _hasMorePages),
    );
  }

  void _onSelectNote(SelectNote event, Emitter<NotesState> emit) {
    final newSelected = List<String>.from(state.selectedNoteIds);

    if (state.selectedNoteIds.contains(event.noteId)) {
      newSelected.remove(event.noteId);

      emit(
        state.copyWith(
          selectedNoteIds: newSelected,
          isSelectionMode: newSelected.isNotEmpty,
        ),
      );
    } else {
      newSelected.add(event.noteId);
      emit(state.copyWith(selectedNoteIds: newSelected, isSelectionMode: true));
    }
  }

  void _onDeselectNote(DeselectNote event, Emitter<NotesState> emit) {
    final newSelected = List<String>.from(state.selectedNoteIds)
      ..remove(event.noteId);
    final mode = newSelected.isNotEmpty;
    emit(state.copyWith(selectedNoteIds: newSelected, isSelectionMode: mode));
  }

  void _onClearSelection(ClearSelection event, Emitter<NotesState> emit) {
    emit(state.copyWith(selectedNoteIds: [], isSelectionMode: false));
  }

  void _onSelectAllNotes(SelectAllNotes event, Emitter<NotesState> emit) {
    final noteIds = <String>{};

    // Select from appropriate source based on mode
    if (state.isSearchMode) {
      noteIds.addAll(state.searchedNotes.map((e) => e.id));
    } else {
      for (final group in state.groupedNotes) {
        noteIds.addAll(group.notes.map((e) => e.id));
      }
    }

    emit(
      state.copyWith(selectedNoteIds: noteIds.toList(), isSelectionMode: true),
    );
  }

  void _dragUpdate(DragUpdate event, Emitter<NotesState> emit) {
    emit(state.copyWith(isDragging: event.isDragging));
  }

  void _onSearchPageToggle(SearchPageToggle event, Emitter<NotesState> emit) {
    emit(state.copyWith(isSearchPage: event.isSearchPage));
  }
}
