import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/app_routes.dart';
import 'package:mechanix_notes/models/note_hive.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_state.dart';
import 'package:mechanix_notes/src/features/home/bottom_menu/bottom_menu.dart';
import 'package:widgets/mechanix.dart';
import 'presentation/note_list.dart';

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

  void onSelect(BuildContext context, String id, List<String> selectedNoteIds) {
    if (selectedNoteIds.contains(id)) {
      context.read<NotesBloc>().add(DeselectNote(id));
    } else {
      context.read<NotesBloc>().add(SelectNote(id));
    }
  }

  void onDeselect(BuildContext context) {
    context.read<NotesBloc>().add(ClearSelection());
  }

  void selectAll(
    BuildContext context,
    List<NoteHive> notes,
    List<String> selectedNoteIds,
  ) {
    final allSelected = selectedNoteIds.length == notes.length;
    if (allSelected) {
      context.read<NotesBloc>().add(ClearSelection());
    } else {
      context.read<NotesBloc>().add(SelectAllNotes());
    }
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
          appBar: MechanixNavigationBar(
            automaticallyImplyLeading: false,
            title: "Notes",
            titleSpacing: 20,
            elevation: 0,
            backgroundColor: Colors.transparent,
            actionWidgets: [
              if (state.isSelectionMode &&
                  state.selectedNoteIds.isNotEmpty) ...[
                Row(
                  children: [
                    Text(
                      "${state.selectedNoteIds.length} Selected",
                      style: TextStyle(color: Colors.white, fontSize: 14),
                    ).padRight(10),
                    if (state.selectedNoteIds.length != state.notes.length)
                      IconButton(
                        onPressed:
                            () => selectAll(
                              context,
                              state.notes,
                              state.selectedNoteIds,
                            ),
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
          ),
          body: Stack(
            children: [
              Positioned.fill(
                child: SingleChildScrollView(
                  physics: const BouncingScrollPhysics(),
                  padding: const EdgeInsets.all(16),
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      NoteList(
                        groupedNotes: [
                          if (state.pinnedNotes.isNotEmpty)
                            GroupedNotes(
                              label: "Pinned Notes",
                              notes: state.pinnedNotes,
                              icon: Image.asset(
                                NotesIcon.pinnedFilledIcon,
                                color: NotesColors.secondaryTextColor,
                                height: 18,
                                width: 18,
                              ).padRight(10),
                            ),
                          ...state.groupedNotes.map(
                            (e) => GroupedNotes(label: e.label, notes: e.notes),
                          ),
                        ],
                        isSelectionMode: state.isSelectionMode,
                        selectedNotes: state.selectedNoteIds,
                        onSelect:
                            (id) =>
                                onSelect(context, id, state.selectedNoteIds),
                        onDeselect: () => onDeselect(context),
                      ).padBottom(10),
                      // const SizedBox(height: 100),
                    ],
                  ),
                ),
              ),

              // Floating bottom menu
              if (state.isSelectionMode)
                Positioned(
                  left: 0,
                  right: 0,
                  bottom: 30,
                  child: Center(
                    child: BottomMenu(
                      isPinnedSelected: state.isPinnedSelected!,
                      isSelectionMode: state.isSelectionMode,
                      selectedNotes: state.selectedNoteIds,
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
