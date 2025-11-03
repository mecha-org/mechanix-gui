import 'package:equatable/equatable.dart';
import 'package:mechanix_notes/src/features/editor/models/toolbar_models.dart';

abstract class EditorEvent extends Equatable {
  @override
  List<Object> get props => [];
}

class ToolbarToggle extends EditorEvent {}

class UndoUpdate extends EditorEvent {
  final bool isUndo;

  UndoUpdate({required this.isUndo});
}

class RedoUpdate extends EditorEvent {
  final bool isRedo;

  RedoUpdate({required this.isRedo});
}

class PinnedUpdate extends EditorEvent {
  final bool isPinned;

  PinnedUpdate({required this.isPinned});
}

class SelectToolbar extends EditorEvent {
  final ToolbarEnum activeToolbar;

  SelectToolbar({required this.activeToolbar});
}

class LoadNoteContent extends EditorEvent {
  final String noteId;
  LoadNoteContent({required this.noteId});
}
