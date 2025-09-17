import 'package:equatable/equatable.dart';
import 'package:mechanix_notes/models/note_hive.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';

class NotesState extends Equatable {
  final bool loading;
  final bool? isPinnedSelected;
  final List<NoteHive> notes;
  final List<NoteHive> pinnedNotes;
  final bool isSelectionMode;
  final List<String> selectedNoteIds;
  final List<GroupedNotes> groupedNotes;
  final List<NoteHive>? searchedNotes;

  const NotesState({
    required this.notes,
    this.loading = false,
    this.isSelectionMode = false,
    this.selectedNoteIds = const [],
    this.pinnedNotes = const [],
    this.groupedNotes = const [],
    this.isPinnedSelected = false,
    this.searchedNotes = const [],
  });

  NotesState copyWith({
    bool? loading,
    List<NoteHive>? notes,
    List<NoteHive>? pinnedNotes,
    List<GroupedNotes>? groupedNotes,
    bool? isSelectionMode,
    List<String>? selectedNoteIds,
    bool? isPinnedSelected,
    List<NoteHive>? searchedNotes,
  }) {
    return NotesState(
      loading: loading ?? this.loading,
      notes: notes ?? this.notes,
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
    loading,
    notes,
    isSelectionMode,
    selectedNoteIds,
    pinnedNotes,
    isPinnedSelected,
    groupedNotes,
    searchedNotes,
  ];
}
