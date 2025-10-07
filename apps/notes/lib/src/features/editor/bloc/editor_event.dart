import 'package:equatable/equatable.dart';

abstract class EditorEvent extends Equatable {
  @override
  List<Object> get props => [];
}

class InitializedEditor extends EditorEvent {}

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
