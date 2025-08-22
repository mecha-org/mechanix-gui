import 'package:equatable/equatable.dart';

abstract class NotesEvent extends Equatable {
  @override
  List<Object> get props => [];
}

class LoadNotes extends NotesEvent {}

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
  final String id;

  DeleteNotes({required this.id});
}
