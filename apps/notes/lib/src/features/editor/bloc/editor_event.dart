import 'package:equatable/equatable.dart';
import 'package:mechanix_notes/src/features/home/models/toolbar_model.dart';

abstract class EditorEvent extends Equatable {
  @override
  List<Object> get props => [];
}

class InitializedEditor extends EditorEvent {
  final bool isPinned;

  InitializedEditor({required this.isPinned});
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
