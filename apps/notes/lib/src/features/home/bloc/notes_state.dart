import 'package:equatable/equatable.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';

class NotesState extends Equatable {
  final bool isLoading;
  final bool isLoadingMore;
  final bool? isPinnedSelected;
  // final List<NoteMetaData> notes;
  final List<NoteMetaData> pinnedNotes;
  final bool isSelectionMode;
  final List<String> selectedNoteIds;
  final List<GroupedNotes> groupedNotes;
  final List<NoteMetaData> searchedNotes;
  final bool hasMorePages;

  const NotesState({
    // required this.notes,
    this.isLoading = false,
    this.isLoadingMore= false,
    this.isSelectionMode = false,
    this.selectedNoteIds = const [],
    this.pinnedNotes = const [],
    this.groupedNotes = const [],
    this.isPinnedSelected = false,
    this.searchedNotes = const [],
    this.hasMorePages = true,
  });

  NotesState copyWith({
    bool? isLoading,
    bool? isLoadingMore,
    List<NoteMetaData>? pinnedNotes,
    bool? hasMorePages,
    List<GroupedNotes>? groupedNotes,
    bool? isSelectionMode,
    List<String>? selectedNoteIds,
    bool? isPinnedSelected,
    List<NoteMetaData>? searchedNotes,
  }) {
    return NotesState(
      isLoading: isLoading ?? this.isLoading,
      isLoadingMore: isLoadingMore ?? this.isLoadingMore,
      hasMorePages: hasMorePages ?? this.hasMorePages,
      // notes: notes ?? this.notes,
      pinnedNotes: pinnedNotes ?? this.pinnedNotes,
      isSelectionMode: isSelectionMode ?? this.isSelectionMode,
      selectedNoteIds: selectedNoteIds ?? this.selectedNoteIds,
      groupedNotes: groupedNotes ?? this.groupedNotes,
      isPinnedSelected: isPinnedSelected ?? this.isPinnedSelected,
      searchedNotes: searchedNotes ?? this.searchedNotes,
    );
  }

  @override
  List<Object?> get props => [
    isLoading,
    isLoadingMore,
    isPinnedSelected,
    // notes,
    pinnedNotes,
    isSelectionMode,
    selectedNoteIds,
    groupedNotes,
    searchedNotes,
    hasMorePages,
  ];
}
