import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/logger.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_event.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_state.dart';
import 'package:mechanix_notes/src/features/editor/models/toolbar_models.dart';

class EditorBloc extends Bloc<EditorEvent, EditorBlocState> {
  final logger = Logger();
  EditorBloc() : super(const EditorBlocState(selectedToolbar: ToolbarEnum.none)) {
    on<InitializedEditor>(_initializeEditor);
    on<ToolbarToggle>(_enableToolbar);
    on<UndoUpdate>(_undoCall);
    on<RedoUpdate>(_redoCall);
    on<PinnedUpdate>(_pinnedCall);
    on<SelectToolbar>(_selectToolbar);
  }

  void _initializeEditor(
    InitializedEditor event,
    Emitter<EditorBlocState> emit,
  ) async {
    logger.i("Notes Editor Initialized");

    emit(
      state.copyWith(
        isPinned: event.isPinned,
        isRedo: false,
        isUndo: false,
        selectedToolbar: ToolbarEnum.none,
        toolbarToggle: true,
      ),
    );
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
