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
  final bool isPinned;
  final String tag;

  UpdateNotes({
    required this.id,
    required this.title,
    required this.content,
    required this.plainText,
    required this.isPinned,
    required this.tag,
  });
}

class CreateNotes extends NotesEvent {
  final String title;
  final String content;
  final String plainText;
  final bool isPinned;
  final String tag;

  CreateNotes(
    this.title,
    this.content,
    this.plainText,
    this.isPinned,
    this.tag,
  );
}

class DeleteNotes extends NotesEvent {
  final List<String> deleteIds;

  DeleteNotes({required this.deleteIds});
}

class UpdateTag extends NotesEvent {
  final List<String> noteIds;
  final String tag;

  UpdateTag({required this.noteIds, required this.tag});
}

class PinnedNotes extends NotesEvent {
  final List<String> noteIds;
  final bool isPinned;

  PinnedNotes({required this.noteIds, required this.isPinned});
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


class CheckPinnedStatus extends NotesEvent {}

class SearchEvent extends NotesEvent {
  final String searchQuery;

  SearchEvent(this.searchQuery);
}
