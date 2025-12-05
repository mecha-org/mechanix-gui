import 'package:equatable/equatable.dart';

abstract class NotesEvent extends Equatable {
  @override
  List<Object> get props => [];
}

class InitializeNotes extends NotesEvent {}

class LoadNotes extends NotesEvent {}

class LoadNextChunk extends NotesEvent {}

class SelectAllNotes extends NotesEvent {}

class UpdateNotes extends NotesEvent {
  final String id;
  final String title;
  final String content;
  final String plainText;

  UpdateNotes({
    required this.id,
    required this.title,
    required this.content,
    required this.plainText,
  });
}

class CreateNotes extends NotesEvent {
  final String title;
  final String content;
  final String plainText;

  CreateNotes(this.title, this.content, this.plainText);
}

class DeleteNotes extends NotesEvent {
  final List<String> deleteIds;

  DeleteNotes({required this.deleteIds});
}

class SelectNote extends NotesEvent {
  final String noteId;
  SelectNote(this.noteId);
}

class DeselectNote extends NotesEvent {
  final String noteId;
  DeselectNote(this.noteId);
}

class ClearSelection extends NotesEvent {}

class SearchEvent extends NotesEvent {
  final String searchQuery;

  SearchEvent(this.searchQuery);
}

class LoadNextSearchChunk extends NotesEvent {
  LoadNextSearchChunk();
}

// Clear search and reset to normal view
class ClearSearch extends NotesEvent {
  ClearSearch();
}

class DragUpdate extends NotesEvent {
  final bool isDragging;
  DragUpdate(this.isDragging);
}
