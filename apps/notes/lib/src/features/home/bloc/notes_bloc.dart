import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/logger.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_state.dart';
import 'package:mechanix_notes/src/features/home/data/notes_repository.dart';

class NotesBloc extends Bloc<NotesEvent, NotesState> {
  final NotesRepository notesRepository;

  final logger = Logger();
  NotesBloc({required this.notesRepository}) : super(NotesState(notes: [])) {
    on<CreateNotes>(_createNote);
    on<LoadNotes>(_loadNotes);
    on<UpdateNotes>(_updateNotes);
    on<DeleteNotes>(_deleteNotes);
    on<UpdateTag>(tagNotes);
    on<PinnedNotes>(pinnedNotes);
    on<SelectNote>(_onSelectNote);
    on<DeselectNote>(_onDeselectNote);
    on<ClearSelection>(_onClearSelection);
    on<SelectAllNotes>(_onSelectAllNotes);
    on<CheckPinnedStatus>(_checkPinnedStatus);
    on<SearchEvent>(_searchNotes);
  }

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
      logger.e('note create failed $e');
    }
  }

  Future<void> _loadNotes(LoadNotes event, Emitter<NotesState> emit) async {
    try {
      logger.i('get notes requested');
      final notesList = await notesRepository.getNotes();

      emit(
        state.copyWith(
          notes: notesList.notes,
          pinnedNotes: notesList.pinnedNotes,
          groupedNotes: notesList.groupedNotes,
        ),
      );
      logger.i('notes loaded');
    } catch (e) {
      emit(state.copyWith(notes: []));
    }
  }

  Future<void> _updateNotes(UpdateNotes event, Emitter<NotesState> emit) async {
    try {
      logger.i('update notes requested');

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
      logger.e('note update failed $e');
    }
  }

  Future<void> _deleteNotes(DeleteNotes event, Emitter<NotesState> emit) async {
    try {
      await notesRepository.deleteNote(event.deleteIds);
      add(LoadNotes());
    } catch (e) {
      logger.e('note delete failed $e');
    }
  }

  Future<void> tagNotes(UpdateTag event, Emitter<NotesState> emit) async {
    try {
      await notesRepository.updateTag(event.noteIds, event.tag);
      add(LoadNotes());
    } catch (e) {
      logger.e('note delete failed $e');
    }
  }

  Future<void> pinnedNotes(PinnedNotes event, Emitter<NotesState> emit) async {
    try {
      await notesRepository.pinnedNotes(event.noteIds, event.isPinned);
      add(LoadNotes());
      add(ClearSelection());
    } catch (e) {
      logger.e('note delete failed $e');
    }
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
    final allIds = state.notes.map((n) => n.id).toList();
    emit(state.copyWith(selectedNoteIds: allIds, isSelectionMode: true));
    add(CheckPinnedStatus());
  }

  void _checkPinnedStatus(CheckPinnedStatus event, Emitter<NotesState> emit) {
    bool? isPinnedSelected;

    final selectedNotes = state.notes.where(
      (note) => state.selectedNoteIds.contains(note.id),
    );
    isPinnedSelected = selectedNotes.every((note) => note.isPinned);
    emit(state.copyWith(isPinnedSelected: isPinnedSelected));
  }

  void _searchNotes(SearchEvent event, Emitter<NotesState> emit) {
    logger.i("notes search started ${event.searchQuery}");

    final query = event.searchQuery.trim().toLowerCase();

    if (query.isNotEmpty) {
      final searchedNotes =
          state.notes
              .where(
                (note) =>
                    (note.plainText.toLowerCase().contains(query) ||
                        note.title.toLowerCase().contains(query)),
              )
              .toList();

      emit(state.copyWith(searchedNotes: searchedNotes));
    } else {
      emit(state.copyWith(searchedNotes: []));
    }
  }
}
