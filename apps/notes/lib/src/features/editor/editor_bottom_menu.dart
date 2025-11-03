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
import 'package:widgets/widgets/floatingActionButton/mechanix_fab.dart';
import 'package:widgets/widgets/floatingActionButton/mechanix_fab_items.dart';

class EditorBottomMenu extends StatelessWidget {
  final QuillController controller;
  final void Function(ToolbarEnum toolbarEnum) onToolbarSelection;
  const EditorBottomMenu({
    super.key,
    required this.controller,
    required this.onToolbarSelection,
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
        final toolbarToggle = tuple.item3;
        final selectedToolbar = tuple.item4;

        if (!toolbarToggle) return Container();

        return Positioned(
          left: 0,
          right: 0,
          bottom: 30,
          child: Center(
            child: SizedBox(
              width: 380,
              height: 52,
              child: MechanixFloatingActionMenu(
                height: 52,
                backgroundColor: NotesColors.floatingMenuColor,
                items: [
                  MechanixFabItem(
                    iconWidget: NotesFabIcon(
                      iconPath: NotesIcon.undoIcon,
                      color:
                          isUndo
                              ? Colors.white
                              : Theme.of(context).disabledColor,
                    ),
                    iconSize: 20,
                    onTap: isUndo ? _undoCall : null,
                  ),
                  MechanixFabItem(
                    iconWidget: NotesFabIcon(
                      iconPath: NotesIcon.redoIcon,
                      color:
                          isRedo
                              ? Colors.white
                              : Theme.of(context).disabledColor,
                    ),
                    iconSize: 20,
                    onTap: isRedo ? _redoCall : null,
                  ),
                  MechanixFabItem(
                    iconSize: 20,
                    iconWidget: NotesFabIcon(
                      iconPath: NotesIcon.textStyleIcon,
                      iconSize: 20,
                      color:
                          selectedToolbar == ToolbarEnum.text
                              ? Theme.of(context).disabledColor
                              : Colors.white,
                    ),
                    onTap: () => onToolbarSelection(ToolbarEnum.text),
                  ),
                  MechanixFabItem(
                    iconSize: 20,
                    iconWidget: NotesFabIcon(
                      iconPath: NotesIcon.menuIcon,
                      iconSize: 20,
                      color:
                          selectedToolbar == ToolbarEnum.align
                              ? Theme.of(context).disabledColor
                              : Colors.white,
                    ),
                    onTap: () => onToolbarSelection(ToolbarEnum.align),
                  ),
                  // MechanixFabItem(
                  //   iconSize: 20,
                  //   iconWidget: NotesFabIcon(
                  //     iconPath: NotesIcon.addIcon,
                  //     iconSize: 20,
                  //     color:
                  //         selectedToolbar == ToolbarEnum.add
                  //             ? Theme.of(context).disabledColor
                  //             : Colors.white,
                  //   ),
                  //   onTap: () => toolbarSelection(ToolbarEnum.add),
                  // ),
                ],
              ),
            ),
          ),
        );
      },
    );
  }
}
