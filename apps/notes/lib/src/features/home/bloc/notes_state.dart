import 'package:equatable/equatable.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';

class NotesState extends Equatable {
  // Main list state
  final bool isLoading;
  final bool isLoadingMore;
  final bool hasMorePages;
  final List<GroupedNotes> groupedNotes;
  final bool isDragging;
  // Search state
  final bool isSearchMode;
  final bool isSearchLoading;
  final bool isSearchLoadingMore;
  final bool hasMoreSearchResults;
  final List<SearchMetaData> searchedNotes;

  // Selection state
  final bool isSelectionMode;
  final List<String> selectedNoteIds;

  const NotesState({
    // Main list
    this.isLoading = false,
    this.isLoadingMore = false,
    this.hasMorePages = true,
    this.groupedNotes = const [],
    this.isDragging = false,
    // Search
    this.isSearchMode = false,
    this.isSearchLoading = false,
    this.isSearchLoadingMore = false,
    this.hasMoreSearchResults = false,
    this.searchedNotes = const [],

    // Selection
    this.isSelectionMode = false,
    this.selectedNoteIds = const [],
  });

  NotesState copyWith({
    bool? isLoading,
    bool? isLoadingMore,
    bool? hasMorePages,
    bool? isDragging,
    List<GroupedNotes>? groupedNotes,
    bool? isSearchMode,
    bool? isSearchLoading,
    bool? isSearchLoadingMore,
    bool? hasMoreSearchResults,
    List<SearchMetaData>? searchedNotes,
    bool? isSelectionMode,
    List<String>? selectedNoteIds,
  }) {
    return NotesState(
      isLoading: isLoading ?? this.isLoading,
      isLoadingMore: isLoadingMore ?? this.isLoadingMore,
      hasMorePages: hasMorePages ?? this.hasMorePages,
      isDragging: isDragging ?? this.isDragging,
      groupedNotes: groupedNotes ?? this.groupedNotes,
      isSearchMode: isSearchMode ?? this.isSearchMode,
      isSearchLoading: isSearchLoading ?? this.isSearchLoading,
      isSearchLoadingMore: isSearchLoadingMore ?? this.isSearchLoadingMore,
      hasMoreSearchResults: hasMoreSearchResults ?? this.hasMoreSearchResults,
      searchedNotes: searchedNotes ?? this.searchedNotes,
      isSelectionMode: isSelectionMode ?? this.isSelectionMode,
      selectedNoteIds: selectedNoteIds ?? this.selectedNoteIds,
    );
  }

  @override
  List<Object?> get props => [
    isLoading,
    isLoadingMore,
    hasMorePages,
    isDragging,
    groupedNotes,
    isSearchMode,
    isSearchLoading,
    isSearchLoadingMore,
    hasMoreSearchResults,
    searchedNotes,
    isSelectionMode,
    selectedNoteIds,
  ];
}
