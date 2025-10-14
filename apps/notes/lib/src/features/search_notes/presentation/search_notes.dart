import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_state.dart';
import 'package:mechanix_notes/src/features/home/presentation/note_list.dart';
import 'package:mechanix_notes/src/features/search_notes/presentation/search_bar.dart';
import 'package:widgets/mechanix.dart';

class SearchNotes extends StatefulWidget {
  const SearchNotes({super.key});

  @override
  State<SearchNotes> createState() => _SearchNotesState();
}

class _SearchNotesState extends State<SearchNotes> {
  @override
  void initState() {
    super.initState();
    context.read<NotesBloc>().add(SearchEvent(''));
  }

  void onBackClick() {
    Navigator.pop(context);
    context.read<NotesBloc>().add(SearchEvent(''));
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<NotesBloc, NotesState>(
      builder: (context, state) {
        return Scaffold(
          appBar: MechanixNavigationBar(
            title: "Search Notes",
            leadingWidget: IconButton(
              icon: Image.asset(NotesIcon.backIcon, height: 20, width: 20),
              onPressed: () => Navigator.pop(context),
              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 8),
            ),
            titleStyle: const TextStyle(
              fontSize: 18,
              fontWeight: FontWeight.normal,
            ),
          ),
          body: Stack(
            children: [
              Positioned.fill(
                child: SingleChildScrollView(
                  padding: const EdgeInsets.all(16),
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      if (state.searchedNotes.isEmpty)
                        const Padding(
                          padding: EdgeInsets.only(top: 120),
                          child: Center(
                            child: Text(
                              "No Notes Found",
                              style: TextStyle(
                                fontSize: 16,
                                color: Colors.grey,
                              ),
                            ),
                          ),
                        )
                      else
                        NoteList(
                          selectedNotes: [],
                          isSelectionMode: false,
                          groupedNotes: [
                            GroupedNotes(label: '', notes: state.searchedNotes),
                          ],
                        ),
                      const SizedBox(
                        height: 80,
                      ), // padding so list doesn't hide behind bar
                    ],
                  ),
                ),
              ),
              Positioned(
                left: 0,
                right: 0,
                bottom: 30,
                child: Center(child: SearchInputBar()),
              ),
            ],
          ),
        );
      },
    );
  }
}
