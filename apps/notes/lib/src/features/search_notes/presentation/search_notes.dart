import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_state.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';
import 'package:mechanix_notes/src/features/home/presentation/note_list.dart';
import 'package:mechanix_notes/src/features/search_notes/presentation/search_bar.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/navigation_bar/mechanix_navigation_bar_theme.dart';

class SearchNotes extends StatefulWidget {
  const SearchNotes({super.key});

  @override
  State<SearchNotes> createState() => _SearchNotesState();
}

class _SearchNotesState extends State<SearchNotes> {
  final ScrollController _scrollController = ScrollController();

  @override
  void initState() {
    super.initState();
    context.read<NotesBloc>().add(ClearSearch());
    _scrollController.addListener(_onScroll);
  }

  @override
  void dispose() {
    _scrollController.removeListener(_onScroll);
    _scrollController.dispose();
    super.dispose();
  }

  void _onScroll() {
    // Trigger load more when scrolled to 90% of the list
    if (_scrollController.position.pixels >=
        _scrollController.position.maxScrollExtent * 0.9) {
      final state = context.read<NotesBloc>().state;
      if (state.isSearchMode &&
          state.hasMoreSearchResults &&
          !state.isSearchLoadingMore) {
        context.read<NotesBloc>().add(LoadNextSearchChunk());
      }
    }
  }

  void onBackClick() {
    Navigator.pop(context);
    context.read<NotesBloc>().add(ClearSearch());
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<NotesBloc, NotesState>(
      builder: (context, state) {
        return Scaffold(
          appBar: const MechanixNavigationBar(
            title: "Search Notes",
            theme: MechanixNavigationBarThemeData(
              titleSpacing: 2,
              titleStyle: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.normal,
              ),
            ),
          ),
          body: Stack(
            children: [
              Positioned.fill(
                child:
                    state.searchedNotes.isEmpty
                        ? const Padding(
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
                        : NoteList(
                          controller: _scrollController,
                          selectedNotes: const [],
                          isSelectionMode: false,
                          groupedNotes: [
                            GroupedNotes(label: '', notes: state.searchedNotes),
                          ],
                        ),
              ),

              const Positioned(
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
