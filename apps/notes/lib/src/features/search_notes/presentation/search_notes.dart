import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/src/commons/common_helper.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_bloc_provider.dart';
import 'package:mechanix_notes/src/features/editor/notes_editor.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_state.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';
import 'package:mechanix_notes/src/features/search_notes/presentation/search_bar.dart';
import 'package:mechanix_notes/src/features/search_notes/presentation/search_highlight.dart';
import 'package:widgets/mechanix.dart';

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
    context.read<NotesBloc>().add(ClearSearch());
    context.read<NotesBloc>().add(LoadNotes());
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return BlocSelector<NotesBloc, NotesState, List<SearchMetaData>>(
      selector: (state) => state.searchedNotes,
      builder: (context, searchedNotes) {
        return Column(
          children: [
            const SizedBox(height: 12),
            Expanded(
              child:
                  searchedNotes.isEmpty
                      ? Container(
                        alignment: Alignment.topLeft,
                        padding: const EdgeInsets.only(
                          left: 16,
                          bottom: 80,
                          top: 10,
                        ),
                        child: Text(
                          searchQuery.trim().length < 2
                              ? "Start typing to search notes..."
                              : "No Notes Found.",
                          style: TextStyle(
                            fontSize: 18,
                            color: context.onSurfaceVariant,
                            fontWeight: FontWeight.w400,
                          ),
                        ),
                      )
                      : Scrollbar(
                        controller: _scrollController,

                        child: ScrollConfiguration(
                          behavior: ScrollConfiguration.of(context).copyWith(
                            dragDevices: {
                              PointerDeviceKind.touch,
                              PointerDeviceKind.mouse,
                            },
                            scrollbars:
                                false, // Disable default, use Scrollbar widget instead
                          ),
                          child: ListView.builder(
                            physics: const AlwaysScrollableScrollPhysics(),
                            controller: _scrollController,
                            itemCount: searchedNotes.length,
                            prototypeItem: const SizedBox(height: 70),
                            padding: const EdgeInsets.only(top: 0),
                            itemBuilder: (context, index) {
                              final note = searchedNotes[index];

                              return MouseRegion(
                                cursor: SystemMouseCursors.click,
                                child: GestureDetector(
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
                                      color: context.secondary,
                                    ),
                                    child: Row(
                                      crossAxisAlignment:
                                          CrossAxisAlignment.center,
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
                                          ),
                                          decoration: BoxDecoration(
                                            color: context.primaryContainer
                                                .withValues(alpha: 0.8),
                                            borderRadius: BorderRadius.circular(
                                              3,
                                            ),
                                          ),
                                          child: Text(
                                            note.availableCount.toString(),
                                          ),
                                        ),
                                        const SizedBox(width: 8),
                                        IntrinsicWidth(
                                          child: Text(
                                            CommonHelper.formatDateTime(
                                              note.updatedAt,
                                            ),
                                            textAlign: TextAlign.end,
                                            style: TextStyle(
                                              color: context.onSecondaryFixed,
                                              fontSize: 14,
                                              fontWeight: FontWeight.w400,
                                            ),
                                          ),
                                        ),
                                      ],
                                    ),
                                  ),
                                ),
                              );
                            },
                          ),
                        ),
                      ),
            ),

            const SizedBox(height: 10),
            // SEARCH INPUT BAR (BOTTOM)
            Padding(
              padding: const EdgeInsets.only(bottom: 0),
              child: SearchInputBar(
                onChanged: (val) {
                  setState(() => searchQuery = val);
                },
              ),
            ),
          ],
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
