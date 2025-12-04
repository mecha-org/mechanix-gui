import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/src/commons/common_helper.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_bloc_provider.dart';
import 'package:mechanix_notes/src/features/editor/notes_editor.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_state.dart';
import 'package:mechanix_notes/src/features/search_notes/presentation/search_bar.dart';
import 'package:mechanix_notes/src/features/search_notes/presentation/search_highlight.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/navigation_bar/mechanix_navigation_bar_theme.dart';

class SearchNotes extends StatefulWidget {
  const SearchNotes({super.key});

  @override
  State<SearchNotes> createState() => _SearchNotesState();
}

class _SearchNotesState extends State<SearchNotes> {
  final ScrollController _scrollController = ScrollController();
  String searchQuery = '';

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
              ListView.builder(
                controller: _scrollController,
                itemCount: state.searchedNotes.length,
                prototypeItem: const SizedBox(height: 70),
                padding: const EdgeInsets.only(top: 0),
                itemBuilder: (context, index) {
                  final note = state.searchedNotes[index];
                  return GestureDetector(
                    onTap: () {
                      _openNote(context, note.id);
                    },
                    child: Container(
                      height: 58,
                      margin: const EdgeInsets.symmetric(
                        horizontal: 16,
                        vertical: 6,
                      ),
                      padding: const EdgeInsets.all(16),
                      decoration: BoxDecoration(
                        borderRadius: BorderRadius.circular(8),
                        color: NotesColors.cardColor,
                      ),
                      child: Row(
                        crossAxisAlignment: CrossAxisAlignment.center,
                        children: [
                          // MAIN TEXT – expands and takes remaining space
                          Expanded(
                            flex: 1,
                            child: SearchHighlight(
                              text: note.text,
                              query: searchQuery,
                              isTitle: note.isTitle,
                            ),
                          ),

                          const SizedBox(width: 8),

                          // COUNT – fixed size
                          Container(
                            padding: const EdgeInsets.symmetric(
                              horizontal: 6,
                              vertical: 2,
                            ),
                            decoration: BoxDecoration(
                              color: NotesColors.highlightTextColor.withValues(
                                alpha: 0.65,
                              ),
                              borderRadius: BorderRadius.circular(3),
                            ),
                            child: Text(
                              note.availableCount.toString(),
                              style: const TextStyle(
                                overflow: TextOverflow.ellipsis,
                              ),
                            ),
                          ),

                          const SizedBox(width: 8),

                          // DATE – wraps its content but never overflows
                          IntrinsicWidth(
                            child: Text(
                              CommonHelper.formatDateTime(note.updatedAt),
                              textAlign: TextAlign.end,
                              style: const TextStyle(
                                overflow: TextOverflow.ellipsis,
                                color: NotesColors.labelColor,
                                fontSize: 14,
                                fontWeight: FontWeight.w400,
                              ),
                            ),
                          ),
                        ],
                      ),
                    ),
                  );
                },
              ),
              Positioned(
                left: 0,
                right: 0,
                bottom: 30,
                child: Center(
                  child: SearchInputBar(
                    onChanged:
                        (val) => setState(() {
                          searchQuery = val;
                        }),
                  ),
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}

void _openNote(BuildContext context, String noteId) {
  Navigator.push(
    context,
    MaterialPageRoute(
      builder: (_) => EditorBlocProvider(child: NotesEditor(noteId: noteId)),
    ),
  );
}
