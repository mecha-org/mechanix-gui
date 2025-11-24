import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/commons/notes_fab_icon.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_bloc.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_state.dart';
import 'package:mechanix_notes/src/features/editor/models/toolbar_models.dart';
import 'package:tuple/tuple.dart';
import 'package:widgets/widgets/floating_action_bar/mechanix_floating_action_bar.dart';
import 'package:widgets/widgets/floating_action_bar/mechanix_floating_action_bar_theme.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';

class EditorBottomMenu extends StatelessWidget {
  final QuillController controller;
  final FloatingActionBarController barController;
  final void Function(ToolbarEnum toolbarEnum) onToolbarSelection;
  const EditorBottomMenu({
    super.key,
    required this.controller,
    required this.onToolbarSelection,
    required this.barController,
  });
  void _undoCall() {
    controller.undo();
  }

  void _redoCall() {
    controller.redo();
  }

  @override
  Widget build(BuildContext context) {
    return BlocSelector<
      EditorBloc,
      EditorBlocState,
      Tuple4<bool, bool, bool, ToolbarEnum>
    >(
      selector:
          (state) => Tuple4(
            state.isUndo,
            state.isRedo,
            state.toolbarToggle,
            state.selectedToolbar,
          ),
      builder: (context, tuple) {
        final isUndo = tuple.item1;
        final isRedo = tuple.item2;
        final selectedToolbar = tuple.item4;

        return Positioned(
          left: 0,
          right: 0,
          bottom: 30,
          child: MechanixFloatingActionBar(
            dropdownPosition: DropdownPosition.topCenter,
            isMenuButtonRequired: false,
            outsideClickDisabled: true,
            floatingActionBarController: barController,
            animationDuration: const Duration(milliseconds: 0),
            theme: MechanixFloatingActionBarThemeData(
              height: 52,
              width: 350,
              decoration: BoxDecoration(
                borderRadius: BorderRadius.circular(28),
                color: NotesColors.floatingToolbarcolor,
              ),
            ),
            menus: [
              IconButton(
                icon: NotesFabIcon(
                  iconPath: NotesIcon.undoIcon,
                  color:
                      isUndo ? Colors.white : Theme.of(context).disabledColor,
                ),
                iconSize: 20,
                onPressed: isUndo ? _undoCall : null,
              ),
              IconButton(
                icon: NotesFabIcon(
                  iconPath: NotesIcon.redoIcon,
                  color:
                      isRedo ? Colors.white : Theme.of(context).disabledColor,
                ),
                iconSize: 20,
                onPressed: isRedo ? _redoCall : null,
              ),
              IconButton(
                iconSize: 20,
                icon: NotesFabIcon(
                  iconPath: NotesIcon.textStyleIcon,
                  iconSize: 20,
                  color:
                      selectedToolbar == ToolbarEnum.text
                          ? Theme.of(context).disabledColor
                          : Colors.white,
                ),
                onPressed: () => onToolbarSelection(ToolbarEnum.text),
              ),
              IconButton(
                iconSize: 20,
                icon: NotesFabIcon(
                  iconPath: NotesIcon.menuIcon,
                  iconSize: 20,
                  color:
                      selectedToolbar == ToolbarEnum.align
                          ? Theme.of(context).disabledColor
                          : Colors.white,
                ),
                onPressed: () => onToolbarSelection(ToolbarEnum.align),
              ),
            ],
          ),
        );
      },
    );
  }
}