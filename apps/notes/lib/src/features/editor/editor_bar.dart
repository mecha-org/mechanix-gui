import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_bloc.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_event.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_state.dart';
import 'package:mechanix_notes/src/features/editor/editor_title_input.dart';
import 'package:mechanix_notes/src/features/editor/menu_options.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';
import 'package:tuple/tuple.dart';
import 'package:widgets/extensions/edge_insets.dart';
import 'package:widgets/widgets.dart';
import 'package:widgets/widgets/navigation_bar/mechanix_navigation_bar.dart';
import 'package:widgets/widgets/navigation_bar/mechanix_navigation_bar_theme.dart';

class EditorBar extends StatefulWidget {
  final QuillController controller;
  final NoteMetaData? note;
  final FloatingActionBarController floatingBar;
  const EditorBar({
    super.key,
    required this.controller,
    this.note,
    required this.floatingBar,
  });

  @override
  State<EditorBar> createState() => _EditorBarState();
}

class _EditorBarState extends State<EditorBar> {
  final LayerLink optionsLayer = LayerLink();
  void _undoCall() {
    widget.controller.undo();
  }

  void _redoCall() {
    widget.controller.redo();
  }

  void _enableToolbar() {
    context.read<EditorBloc>().add(ToolbarToggle());
  }

  @override
  Widget build(BuildContext context) {
    return MechanixNavigationBar(
      automaticallyImplyLeading: false,
      leadingWidget: EditorTitleInput(
        floatingBar: widget.floatingBar,
        controller: widget.controller,
        note: widget.note,
      ),
      theme: const MechanixNavigationBarThemeData(
        leadingWidth: 320,
        actionsIconTheme: IconThemeData(size: 20),
      ),

      actionWidgets: [
        BlocSelector<EditorBloc, EditorBlocState, Tuple3<bool, bool, bool>>(
          selector:
              (state) =>
                  Tuple3(state.isUndo, state.isRedo, state.toolbarToggle),
          builder: (context, tuple) {
            final isUndo = tuple.item1;
            final isRedo = tuple.item2;
            final toolbarToggle = tuple.item3;

            return Row(
              children: [
                if (!toolbarToggle) ...[
                  IconButton(
                    onPressed: isUndo ? _undoCall : null,
                    icon: SizedBox(
                      height: 20,
                      width: 20,
                      child: Image.asset(
                        NotesIcon.undoIcon,
                        color:
                            isUndo
                                ? Colors.white
                                : Theme.of(context).disabledColor,
                      ),
                    ),
                  ).padRight(10),
                  IconButton(
                    onPressed: isRedo ? _redoCall : null,
                    icon: SizedBox(
                      height: 20,
                      width: 20,
                      child: Image.asset(
                        NotesIcon.redoIcon,
                        color:
                            isRedo
                                ? Colors.white
                                : Theme.of(context).disabledColor,
                      ),
                    ),
                  ).padRight(10),
                ],
                IconButton(
                  onPressed: _enableToolbar,
                  icon: SizedBox(
                    height: 20,
                    width: 20,
                    child: Image.asset(
                      toolbarToggle
                          ? NotesIcon.toolbarEnableIcon
                          : NotesIcon.toolbarDisableIcon,
                    ),
                  ),
                ),

                BlocSelector<EditorBloc, EditorBlocState, bool>(
                  selector: (state) => state.isPinned,
                  builder: (context, isPinned) {
                    return MenuOptions(isPinned: isPinned);
                  },
                ),
              ],
            );
          },
        ),
      ],
    );
  }
}
