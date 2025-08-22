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
  }

  Future<void> _createNote(CreateNotes event, Emitter<NotesState> emit) async {
    try {
      await notesRepository.createNote(
        event.title,
        event.content,
        event.plainText
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
      emit(state.copyWith(notes: notesList));
      logger.i("notes loaded");
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
        event.plainText
      );
      add(LoadNotes());
    } catch (e) {
      logger.e('note update failed $e');
    }
  }

  Future<void> _deleteNotes(DeleteNotes event, Emitter<NotesState> emit) async {
    try {
      await notesRepository.deleteNote(event.id);
      add(LoadNotes());
    } catch (e) {
      logger.e('note delete failed $e');
    }
  }
}
