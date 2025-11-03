import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/app_routes.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_state.dart';
import 'package:mechanix_notes/src/features/home/bottom_menu/bottom_menu.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';
import 'package:mechanix_notes/src/features/home/presentation/note_list.dart';
import 'package:tuple/tuple.dart';
import 'package:widgets/mechanix.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  final ScrollController _scrollController = ScrollController();

  @override
  void initState() {
    super.initState();

    context.read<NotesBloc>().add(LoadNotes());

    _scrollController.addListener(_onScroll);
  }

  void onSelect(BuildContext context, String id, List<String> selectedNoteIds) {
    if (selectedNoteIds.contains(id)) {
      context.read<NotesBloc>().add(DeselectNote(id));
    } else {
      context.read<NotesBloc>().add(SelectNote(id));
    }
  }

  void _onScroll() {
    if (!_scrollController.hasClients) return;
    final position = _scrollController.position;
    if (!position.hasPixels || !position.hasContentDimensions) return;

    final maxScroll = position.maxScrollExtent;
    final currentScroll = position.pixels;

    // Trigger near bottom
    if (currentScroll >= 0.8 * maxScroll) {
      context.read<NotesBloc>().add(LoadNextChunk());
      // notesController.loadNextChunk();
    }
  }

  void onDeselect(BuildContext context) {
    context.read<NotesBloc>().add(ClearSelection());
  }

  @override
  void dispose() {
    _scrollController.dispose();
    // notesController.dispose();
    super.dispose();
  }

  void selectAll() {
    context.read<NotesBloc>().add(SelectAllNotes());
  }

  void onDeleteRemoveSelection(
    BuildContext context,
    List<String> selectedNoteIds,
  ) {
    context.read<NotesBloc>().add(
      DeleteNotes(deleteIds: selectedNoteIds.toList()),
    );
    context.read<NotesBloc>().add(ClearSelection());
  }

  void onSearch() {
    Navigator.pushNamed(context, AppRoutes.searchNotes);
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<NotesBloc, NotesState>(
      buildWhen:
          (previous, current) =>
              previous.isSelectionMode != current.isSelectionMode,
      builder: (context, state) {
        return Scaffold(
          floatingActionButton:
              !state.isSelectionMode
                  ? Padding(
                    padding: const EdgeInsets.only(bottom: 36, right: 20),
                    child: SizedBox(
                      height: 64,
                      width: 64,
                      child: FloatingActionButton(
                        onPressed: () {
                          Navigator.pushNamed(
                            context,
                            AppRoutes.createEditNotes,
                          );
                        },
                        shape: RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(36),
                        ),
                        backgroundColor: NotesColors.floatingButtonColor,
                        child: SizedBox(
                          height: 28,
                          width: 28,
                          child: Image.asset(NotesIcon.addIcon),
                        ),
                      ),
                    ),
                  )
                  : null,
          appBar: PreferredSize(
            preferredSize: const Size.fromHeight(50),
            child: BlocSelector<NotesBloc, NotesState, List<String>>(
              selector: (state) => state.selectedNoteIds,
              builder: (context, selectedNoteIds) {
                return MechanixNavigationBar(
                  automaticallyImplyLeading: false,
                  title: "Notes",
                  titleSpacing: 20,
                  elevation: 0,
                  backgroundColor: Colors.transparent,
                  actionWidgets: [
                    if (state.isSelectionMode &&
                        selectedNoteIds.isNotEmpty) ...[
                      Row(
                        children: [
                          Text(
                            "${selectedNoteIds.length} Selected",
                            style: const TextStyle(
                              color: Colors.white,
                              fontSize: 14,
                            ),
                          ).padRight(10),
                          // if (selectedNoteIds.length != groupedNotes.length)
                          IconButton(
                            onPressed: selectAll,
                            icon: SizedBox(
                              height: 20,
                              width: 20,
                              child: Image.asset(NotesIcon.selectAllIcon),
                            ),
                          ),
                        ],
                      ).padRight(10),
                    ] else
                      IconButton(
                        onPressed: onSearch,
                        icon: Image.asset(NotesIcon.searchIcon),
                      ).padRight(10),
                  ],
                  titleStyle: const TextStyle(
                    fontSize: 24,
                    color: NotesColors.headerColor,
                  ),
                );
              },
            ),
          ),

          body: BlocSelector<NotesBloc, NotesState, List<String>>(
            selector: (state) => state.selectedNoteIds,
            builder: (context, selectedNoteIds) {
              return Stack(
                children: [
                  BlocSelector<
                    NotesBloc,
                    NotesState,
                    Tuple2<bool, List<GroupedNotes>>
                  >(
                    selector:
                        (state) => Tuple2(state.isLoading, state.groupedNotes),
                    builder: (context, data) {
                      if (data.item2.isEmpty) {
                        return Container(
                          padding: const EdgeInsets.symmetric(horizontal: 16),
                          height: 64,
                          child: MechanixPressableList(
                            itemPadding: const EdgeInsets.all(10),
                            onTap:
                                () => Navigator.pushNamed(
                                  context,
                                  AppRoutes.createEditNotes,
                                ),
                            leadingIcon: Image.asset(
                              NotesIcon.addIcon,
                              height: 18,
                              width: 18,
                            ),
                            isSelected: true,
                            title: "Add a new Note",
                            titleTextStyle: const TextStyle(
                              color: NotesColors.secondaryTextColor,
                              fontSize: 16,
                            ),
                          ),
                        );
                      }
                      //     if (!isLoading) {
                      return Positioned.fill(
                        child: Padding(
                          padding: const EdgeInsets.all(16),
                          child: NoteList(
                            controller: _scrollController,
                            isSelectionMode: state.isSelectionMode,
                            selectedNotes: selectedNoteIds,
                            onSelect:
                                (id) => onSelect(context, id, selectedNoteIds),
                            groupedNotes: data.item2,
                          ),
                        ),
                      );
                    },
                  ),

                  // ValueListenableBuilder<bool>(
                  //   valueListenable: notesController.isLoading,
                  //   builder: (context, isLoading, _) {
                  //     if (!isLoading) {
                  //       return ValueListenableBuilder<List<GroupedNotes>>(
                  //         valueListenable: notesController.groupedNotes,
                  //         builder: (context, groupedNotes, _) {
                  //           return Positioned.fill(
                  //             child: Padding(
                  //               padding: const EdgeInsets.all(16),
                  //               child: NoteList(
                  //                 controller: _scrollController,
                  //                 isSelectionMode: state.isSelectionMode,
                  //                 selectedNotes: selectedNoteIds,
                  //                 onSelect:
                  //                     (id) => onSelect(
                  //                       context,
                  //                       id,
                  //                       selectedNoteIds,
                  //                     ),
                  //                 groupedNotes: groupedNotes,
                  //               ),
                  //             ),
                  //           );
                  //         },
                  //       );
                  //     }
                  //     return Positioned.fill(
                  //       child: Center(
                  //         child: Container(
                  //           padding: const EdgeInsets.all(12),
                  //           decoration: BoxDecoration(
                  //             color: Colors.black54,
                  //             borderRadius: BorderRadius.circular(8),
                  //           ),
                  //           child: const SizedBox(
                  //             height: 20,
                  //             width: 20,
                  //             child: CircularProgressIndicator(
                  //               strokeWidth: 2,
                  //               valueColor: AlwaysStoppedAnimation<Color>(
                  //                 Colors.white,
                  //               ),
                  //             ),
                  //           ),
                  //         ),
                  //       ),
                  //     );
                  //   },
                  // ),

                  // Floating bottom menu
                  if (state.isSelectionMode)
                    Positioned(
                      left: 0,
                      right: 0,
                      bottom: 30,
                      child: Center(
                        child: BottomMenu(
                          selectedNotes: selectedNoteIds,
                          isSelectionMode: state.isSelectionMode,
                        ),
                      ),
                    ),
                ],
              );
            },
          ),
        );
      },
    );
  }
}
