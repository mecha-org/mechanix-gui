import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/logger.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_event.dart';

import 'package:mechanix_notes/src/features/editor/bloc/editor_state.dart';

class EditorBloc extends Bloc<EditorEvent, EditorBlocState> {
  final logger = Logger();
  EditorBloc()
    : super(
        EditorBlocState(
          isEditing: false,
          linkLayer: LayerLink(),
          optionsLayer: LayerLink(),
        ),
      ) {
    on<InitializedEditor>(_initializeEditor);
    on<ToolbarToggle>(_enableToolbar);
    on<UndoUpdate>(_undoCall);
    on<RedoUpdate>(_redoCall);
    on<PinnedUpdate>(_pinnedCall);
  }

  void _initializeEditor(
    InitializedEditor event,
    Emitter<EditorBlocState> emit,
  ) async {
    logger.i("Notes Editor Initialized");
  }

  void _enableToolbar(ToolbarToggle event, Emitter<EditorBlocState> emit) {
    emit(
      state.copyWith(
        toolbarToggle: !state.toolbarToggle,
        selectedToolbar: !state.toolbarToggle ? null : state.selectedToolbar,
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
    if (event.isPinned != state.isPinned) {
      emit(state.copyWith(isPinned: event.isPinned));
    }
  }
}
