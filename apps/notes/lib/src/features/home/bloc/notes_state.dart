import 'package:equatable/equatable.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';

class NotesState extends Equatable {
  // Main list state
  final bool isLoading;
  final bool isLoadingMore;
  final bool hasMorePages;
  final List<GroupedNotes> groupedNotes;
  
  // Search state
  final bool isSearchMode;
  final bool isSearchLoading;
  final bool isSearchLoadingMore;
  final bool hasMoreSearchResults;
  final List<NoteMetaData> searchedNotes;
  
  // Selection state
  final bool isSelectionMode;
  final List<String> selectedNoteIds;
  final bool? isPinnedSelected;
  
  // Other state
  final List<NoteMetaData> pinnedNotes;

  const NotesState({
    // Main list
    this.isLoading = false,
    this.isLoadingMore = false,
    this.hasMorePages = true,
    this.groupedNotes = const [],
    
    // Search
    this.isSearchMode = false,
    this.isSearchLoading = false,
    this.isSearchLoadingMore = false,
    this.hasMoreSearchResults = false,
    this.searchedNotes = const [],
    
    // Selection
    this.isSelectionMode = false,
    this.selectedNoteIds = const [],
    this.isPinnedSelected = false,
    
    // Other
    this.pinnedNotes = const [],
  });

  NotesState copyWith({
    bool? isLoading,
    bool? isLoadingMore,
    bool? hasMorePages,
    List<GroupedNotes>? groupedNotes,
    bool? isSearchMode,
    bool? isSearchLoading,
    bool? isSearchLoadingMore,
    bool? hasMoreSearchResults,
    List<NoteMetaData>? searchedNotes,
    bool? isSelectionMode,
    List<String>? selectedNoteIds,
    bool? isPinnedSelected,
    List<NoteMetaData>? pinnedNotes,
  }) {
    return NotesState(
      isLoading: isLoading ?? this.isLoading,
      isLoadingMore: isLoadingMore ?? this.isLoadingMore,
      hasMorePages: hasMorePages ?? this.hasMorePages,
      groupedNotes: groupedNotes ?? this.groupedNotes,
      isSearchMode: isSearchMode ?? this.isSearchMode,
      isSearchLoading: isSearchLoading ?? this.isSearchLoading,
      isSearchLoadingMore: isSearchLoadingMore ?? this.isSearchLoadingMore,
      hasMoreSearchResults: hasMoreSearchResults ?? this.hasMoreSearchResults,
      searchedNotes: searchedNotes ?? this.searchedNotes,
      isSelectionMode: isSelectionMode ?? this.isSelectionMode,
      selectedNoteIds: selectedNoteIds ?? this.selectedNoteIds,
      isPinnedSelected: isPinnedSelected ?? this.isPinnedSelected,
      pinnedNotes: pinnedNotes ?? this.pinnedNotes,
    );
  }

  @override
  List<Object?> get props => [
    isLoading,
    isLoadingMore,
    hasMorePages,
    groupedNotes,
    isSearchMode,
    isSearchLoading,
    isSearchLoadingMore,
    hasMoreSearchResults,
    searchedNotes,
    isSelectionMode,
    selectedNoteIds,
    isPinnedSelected,
    pinnedNotes,
  ];
}