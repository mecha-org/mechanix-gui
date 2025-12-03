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
import 'package:widgets/widgets/navigation_bar/mechanix_navigation_bar_theme.dart';

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

  void _onScroll() {
    if (!_scrollController.hasClients) return;
    final position = _scrollController.position;
    if (!position.hasPixels || !position.hasContentDimensions) return;

    final maxScroll = position.maxScrollExtent;
    final currentScroll = position.pixels;
    // Trigger near bottom
    if (currentScroll >= 0.8 * maxScroll) {
      context.read<NotesBloc>().add(LoadNextChunk());
    }
  }

  @override
  void dispose() {
    _scrollController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return BlocSelector<NotesBloc, NotesState, bool>(
      selector: (state) => state.isSelectionMode,
      builder: (context, isSelectionMode) {
        return Scaffold(
          bottomNavigationBar:
              isSelectionMode
                  ? BottomMenu(isSelectionMode: isSelectionMode)
                  : null,
          appBar: PreferredSize(
            preferredSize: const Size.fromHeight(48),
            child: BlocSelector<NotesBloc, NotesState, List<String>>(
              selector: (state) => state.selectedNoteIds,
              builder: (context, selectedNoteIds) {
                return MechanixNavigationBar(
                  height: 48,
                  automaticallyImplyLeading: false,
                  theme: MechanixNavigationBarThemeData(
                    titleStyle:
                        isSelectionMode
                            ? const TextStyle(
                              color: NotesColors.tooltipColor,
                              fontSize: 20,
                              fontWeight: FontWeight.w500,
                            )
                            : const TextStyle(
                              fontSize: 28,
                              color: NotesColors.secondaryTextColor,
                              fontWeight: FontWeight.w600,
                              height: 1.3,
                            ),
                  ),

                  title:
                      isSelectionMode
                          ? "${selectedNoteIds.length} Selected"
                          : "Notes",
                );
              },
            ),
          ),

          body: Stack(
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
                    return SizedBox(
                      height: 64,
                      child: MechanixSelectableList(
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
                      ),
                    );
                  }
                  return NoteList(
                    controller: _scrollController,
                    isSelectionMode: isSelectionMode,
                    groupedNotes: data.item2,
                  );
                },
              ),
            ],
          ),
        );
      },
    );
  }
}
