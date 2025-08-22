import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/app_routes.dart';
import 'package:mechanix_notes/src/commons/custom_app_bar.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_state.dart';
import 'package:mechanix_notes/src/features/home/presentation/note_card.dart';
import 'package:mechanix_notes/src/styles/constants.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  @override
  void initState() {
    super.initState();
    context.read<NotesBloc>().add(LoadNotes());
  }

  @override
  @override
  Widget build(BuildContext context) {
    return BlocListener<NotesBloc, NotesState>(
      listener: (context, state) {
        
        // if (state is NotesErrorState) {
        //   ScaffoldMessenger.of(context).showSnackBar(
        //     SnackBar(
        //       content: Text('Error: ${state.message}'),
        //       backgroundColor: Colors.red,
        //     ),
        //   );
        // }

        // You can add more state-based actions here if needed
        // Example: NotesSuccessState, NotesDeletedState, etc.
      },
      child: Scaffold(
        appBar: CustomAppBar(
          title: "Notes",
          rightIcon1: Image.asset(Images.add),
          rightIcon1OnTap: () {
            Navigator.pushNamed(context, AppRoutes.createEditNotes);
          },
        ),
        body: Padding(
          padding: const EdgeInsets.all(16),
          child: BlocBuilder<NotesBloc, NotesState>(
            builder: (context, state) {
              return ListView.separated(
                itemCount: state.notes.length,
                physics: const BouncingScrollPhysics(),
                separatorBuilder: (_, __) => const SizedBox(height: 16),
                itemBuilder: (context, index) {
                  final note = state.notes[index];
                  return NoteCard(note: note);
                },
              );
            },
          ),
        ),
      ),
    );
  }
}


