import 'package:equatable/equatable.dart';
import 'package:mechanix_notes/models/note_hive.dart';

class NotesState extends Equatable {
  final bool loading;
  final List<NoteHive> notes;

  const NotesState({this.loading = false, required this.notes});

  NotesState copyWith({bool? loading, List<NoteHive>? notes}) {
    return NotesState(
      loading: loading ?? this.loading,
      notes: notes ?? this.notes,
    );
  }

  @override
  List<Object?> get props => [loading, notes];
}
