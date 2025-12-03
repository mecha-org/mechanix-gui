import 'dart:convert';

import 'package:flutter/widgets.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_bloc.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_event.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_state.dart';
import 'package:mechanix_notes/src/features/editor/models/toolbar_models.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';
import 'package:tuple/tuple.dart';
import 'package:widgets/widgets.dart';
import 'package:widgets/widgets/bottomBar/bottom_bar_button_type.dart';

class EditorBottomBar extends StatelessWidget {
  final QuillController controller;
  final NoteMetaData? note;
  final FocusNode focusNode;

  const EditorBottomBar({
    super.key,
    required this.controller,
    this.note,
    required this.focusNode,
  });

  void _saveNotes(BuildContext context) {
    String title = "";
    final content = jsonEncode(controller.document.toDelta().toJson());
    final plainText = controller.document.toPlainText();
    final firstNewLineIndex = plainText.indexOf('\n');
    if (firstNewLineIndex != -1) {
      title = plainText.substring(0, firstNewLineIndex);
    }

    if (plainText.isNotEmpty && plainText != '\n') {
      if (note != null) {
        context.read<NotesBloc>().add(
          UpdateNotes(
            id: note!.id,
            content: content,
            title: title,
            plainText: plainText,
            isPinned: false,
            tag: 'none',
          ),
        );
      } else {
        context.read<NotesBloc>().add(
          CreateNotes(title, content, plainText, false, 'none'),
        );
      }
    }
    Navigator.pop(context);
  }

  void toolbarSelection(BuildContext context, ToolbarEnum value) {
    if (focusNode.hasFocus) {
      focusNode.unfocus();
    }
    context.read<EditorBloc>().add(SelectToolbar(activeToolbar: value));
  }

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
        
        return MechanixBottomBar(
          leadingWidget: [
            BottomBarButton(
              onPressed: () => _saveNotes(context),
              iconPath: NotesIcon.backIcon,
            ),
          ],
          centerWidget: [
            BottomBarButton(
              onPressed: () => toolbarSelection(context, ToolbarEnum.text),
              iconPath: NotesIcon.textStyleIcon,
            ),
            BottomBarButton(
              onPressed: () => toolbarSelection(context, ToolbarEnum.align),
              iconPath: NotesIcon.menuIcon,
            ),
            BottomBarButton(
              onPressed: isUndo ? _undoCall : () {},
              iconPath: NotesIcon.undoIcon,
            ),
            BottomBarButton(
              onPressed: isRedo ? _redoCall : () {},
              iconPath: NotesIcon.redoIcon,
            ),
          ],
          anchorWidget: [
            BottomBarButton(
              onPressed: () => {},
              iconPath: NotesIcon.threeDotIcon,
            ),
          ],
        );
      },
    );
  }
}
