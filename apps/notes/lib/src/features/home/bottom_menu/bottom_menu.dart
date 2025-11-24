import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/commons/notes_fab_icon.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_state.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/floating_action_bar/mechanix_floating_action_bar_theme.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';

class BottomMenu extends StatefulWidget {
  final bool isSelectionMode;
  final List<String> selectedNotes;

  const BottomMenu({
    super.key,
    required this.isSelectionMode,
    required this.selectedNotes,
  });

  @override
  State<BottomMenu> createState() => _BottomMenuState();
}

class _BottomMenuState extends State<BottomMenu> {
  final FloatingActionBarController fabController =
      FloatingActionBarController();
  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) {
        fabController.open();
      }
    });
  }

  void onDeleteRemoveSelection() {
    fabController.close();
    context.read<NotesBloc>().add(DeleteNotes(deleteIds: widget.selectedNotes));
    clearSelection();
  }

  void clearSelection() {
    fabController.close();
    context.read<NotesBloc>().add(ClearSelection());
  }

  @override
  void dispose() {
    fabController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return BlocSelector<NotesBloc, NotesState, bool?>(
      selector: (state) => state.isPinnedSelected,
      builder: (context, isPinnedSelected) {
        return SizedBox(
          width: 278,
          height: 52,
          child: MechanixFloatingActionBar(
            isMenuButtonRequired: false,
            outsideClickDisabled: true,
            theme: MechanixFloatingActionBarThemeData(
              height: 52,
              width: 278,
              decoration: BoxDecoration(
                borderRadius: BorderRadius.circular(28),
                color: NotesColors.floatingMenuColor,
              ),
            ),
            floatingActionBarController: fabController,
            dropdownPosition: DropdownPosition.center,
            menus: [
              IconButton(
                onPressed: () {
                  context.read<NotesBloc>().add(
                    PinnedNotes(
                      noteIds: widget.selectedNotes,
                      isPinned: !isPinnedSelected,
                    ),
                  );
                },
                icon: NotesFabIcon(
                  iconPath:
                      isPinnedSelected!
                          ? NotesIcon.unPinnedIcon
                          : NotesIcon.pinIcon,
                ),
              ),

              IconButton(
                onPressed: () => onDeleteRemoveSelection(),
                icon: const NotesFabIcon(iconPath: NotesIcon.deleteIcon),
              ),
              IconButton(
                onPressed: () => clearSelection(),
                icon: const NotesFabIcon(
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
