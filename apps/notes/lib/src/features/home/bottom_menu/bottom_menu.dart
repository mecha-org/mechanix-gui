import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/commons/notes_fab_icon.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_state.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/floatingActionButton/mechanix_fab_items.dart';

class BottomMenu extends StatefulWidget {
  final bool isSelectionMode;
  // final bool isPinnedSelected;
  final List<String> selectedNotes;
  // final VoidCallback onDelete;

  const BottomMenu({
    super.key,
    required this.isSelectionMode,
    required this.selectedNotes,
    // required this.isPinnedSelected,
    // required this.onDelete,
  });

  @override
  State<BottomMenu> createState() => _BottomMenuState();
}

class _BottomMenuState extends State<BottomMenu> {
  final LayerLink tagLayer = LayerLink();

  void onDeleteRemoveSelection() {
    context.read<NotesBloc>().add(DeleteNotes(deleteIds: widget.selectedNotes));
    clearSelection();
  }

  void clearSelection() {
    context.read<NotesBloc>().add(ClearSelection());
  }

  @override
  Widget build(BuildContext context) {
    return BlocSelector<NotesBloc, NotesState, bool?>(
      selector: (state) => state.isPinnedSelected,
      builder: (context, isPinnedSelected) {
        return SizedBox(
          width: 278,
          height: 52,
          child: MechanixFloatingActionMenu(
            height: 52,
            backgroundColor: const Color(0xFF48494B),
            items: [
              MechanixFabItem(
                iconSize: 24,
                iconWidget: NotesFabIcon(
                  iconPath:
                      isPinnedSelected!
                          ? NotesIcon.unPinnedIcon
                          : NotesIcon.pinIcon,
                ),
                onTap: () {
                  context.read<NotesBloc>().add(
                    PinnedNotes(
                      noteIds: widget.selectedNotes,
                      isPinned: !isPinnedSelected,
                    ),
                  );
                },
              ),

              MechanixFabItem(
                iconWidget: const NotesFabIcon(iconPath: NotesIcon.deleteIcon),
                onTap: () => onDeleteRemoveSelection(),
              ),
              MechanixFabItem(
                onTap: () => clearSelection(),
                iconWidget: const NotesFabIcon(
                  iconPath: NotesIcon.clearSelectionIcon,
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}
