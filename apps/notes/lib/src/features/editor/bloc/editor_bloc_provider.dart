import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_bloc.dart';
import 'package:flutter/material.dart';
import 'package:mechanix_notes/src/features/home/data/notes_repository.dart';

class EditorBlocProvider extends StatelessWidget {
  final Widget child;

  const EditorBlocProvider({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create:
          (_) => EditorBloc(notesRepository: context.read<NotesRepository>()),
      child: child,
    );
  }
}
