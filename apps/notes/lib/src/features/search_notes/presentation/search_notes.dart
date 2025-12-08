import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/src/commons/common_helper.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_bloc_provider.dart';
import 'package:mechanix_notes/src/features/editor/notes_editor.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_state.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';
import 'package:mechanix_notes/src/features/search_notes/presentation/search_bar.dart';
import 'package:mechanix_notes/src/features/search_notes/presentation/search_highlight.dart';

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
    return BlocSelector<NotesBloc, NotesState, List<SearchMetaData>>(
      selector: (state) => state.searchedNotes,
      builder: (context, searchedNotes) {
        return Container(
          padding: const EdgeInsets.only(top: 12, bottom: 16),
          child: Stack(
            children: [
              if (searchedNotes.isEmpty)
                Center(
                  child: Padding(
                    padding: const EdgeInsets.only(bottom: 80),
                    child: Text(
                      searchQuery.isEmpty
                          ? "Start typing to search notes..."
                          : "No Notes Found.",
                      style: const TextStyle(
                        fontSize: 16,
                        color: NotesColors.labelColor,
                        fontWeight: FontWeight.w400,
                      ),
                    ),
                  ),
                ),

              if (searchedNotes.isNotEmpty)
                ListView.builder(
                  controller: _scrollController,
                  itemCount: searchedNotes.length,
                  prototypeItem: const SizedBox(height: 70),
                  padding: const EdgeInsets.only(top: 0),
                  itemBuilder: (context, index) {
                    final note = searchedNotes[index];
                    return GestureDetector(
                      onTap: () => _openNote(context, note.id),
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
                            Expanded(
                              child: SearchHighlight(
                                text: note.text,
                                query: searchQuery,
                                isTitle: note.isTitle,
                              ),
                            ),
                            const SizedBox(width: 8),
                            Container(
                              padding: const EdgeInsets.symmetric(
                                horizontal: 6,
                                vertical: 2,
                              ),
                              decoration: BoxDecoration(
                                color: NotesColors.highlightTextColor
                                    .withValues(alpha: 0.65),
                                borderRadius: BorderRadius.circular(3),
                              ),
                              child: Text(note.availableCount.toString()),
                            ),
                            const SizedBox(width: 8),
                            IntrinsicWidth(
                              child: Text(
                                CommonHelper.formatDateTime(note.updatedAt),
                                textAlign: TextAlign.end,
                                style: const TextStyle(
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
                    onChanged: (val) {
                      setState(() {
                        searchQuery = val;
                      });
                    },
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
