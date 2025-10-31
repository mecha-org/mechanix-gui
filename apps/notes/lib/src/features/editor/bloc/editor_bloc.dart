import 'dart:convert';

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:logger/logger.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_event.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_state.dart';
import 'package:mechanix_notes/src/features/editor/models/toolbar_models.dart';
import 'package:mechanix_notes/src/features/home/data/notes_repository.dart';

class EditorBloc extends Bloc<EditorEvent, EditorBlocState> {
  final logger = Logger();
  final NotesRepository notesRepository;

  EditorBloc({required this.notesRepository})
    : super(const EditorBlocState(selectedToolbar: ToolbarEnum.none)) {
    on<ToolbarToggle>(_enableToolbar);
    on<UndoUpdate>(_undoCall);
    on<RedoUpdate>(_redoCall);
    on<PinnedUpdate>(_pinnedCall);
    on<SelectToolbar>(_selectToolbar);
    on<LoadNoteContent>(_onLoadNoteContent);
  }

  Future<void> _onLoadNoteContent(
    LoadNoteContent event,
    Emitter<EditorBlocState> emit,
  ) async {
    try {
      emit(state.copyWith(isLoading: true));

      final note = await notesRepository.findById(event.noteId);
      if (note == null) return;

      final content = jsonDecode(note.content);
      emit(
        state.copyWith(
          isLoading: false,
          isPinned: note.isPinned,
          document: Document.fromJson(content),
        ),
      );
    } catch (e) {
      emit(state.copyWith(isLoading: false));
    }
  }

  void _enableToolbar(ToolbarToggle event, Emitter<EditorBlocState> emit) {
    emit(
      state.copyWith(
        toolbarToggle: !state.toolbarToggle,
        selectedToolbar: ToolbarEnum.none,
      ),
    );
  }

  void _undoCall(UndoUpdate event, Emitter<EditorBlocState> emit) {
    if (event.isUndo != state.isUndo) {
      emit(state.copyWith(isUndo: !state.isUndo));
    }
  }

  void _redoCall(RedoUpdate event, Emitter<EditorBlocState> emit) {
    if (event.isRedo != state.isRedo) {
      emit(state.copyWith(isRedo: !state.isRedo));
    }
  }

  void _pinnedCall(PinnedUpdate event, Emitter<EditorBlocState> emit) {
    logger.i('pinned call event: ${event.isPinned} state: ${state.isPinned}');
    if (event.isPinned != state.isPinned) {
      emit(state.copyWith(isPinned: event.isPinned));
    }
  }

  void _selectToolbar(SelectToolbar event, Emitter<EditorBlocState> emit) {
    logger.i(
      'select toolbar event: ${event.activeToolbar} state: ${state.selectedToolbar}',
    );
    if (event.activeToolbar == state.selectedToolbar) {
      emit(state.copyWith(selectedToolbar: ToolbarEnum.none));
      return;
    }
    emit(state.copyWith(selectedToolbar: event.activeToolbar));
  }
}
