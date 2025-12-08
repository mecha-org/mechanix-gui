import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_bloc.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_event.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_state.dart';
import 'package:mechanix_notes/src/features/editor/editor_menu.dart';
import 'package:mechanix_notes/src/features/editor/models/toolbar_models.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/alignment_toolbar.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/text_editor_toolbar.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:widgets/widgets.dart';
import 'package:widgets/widgets/bottomBar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottomBar/mechanix_bottom_bar_theme.dart';
import 'package:widgets/widgets/floating_action_bar/mechanix_floating_action_bar_theme.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';

class EditorBottomBar extends StatefulWidget {
  final QuillController controller;
  final String? noteId;
  final FocusNode focusNode;

  @override
  State<EditorBottomBar> createState() => _EditorBottomBarState();
  const EditorBottomBar({
    super.key,
    required this.controller,
    this.noteId,
    required this.focusNode,
  });
}

class _EditorBottomBarState extends State<EditorBottomBar> {
  FloatingActionBarController textEditorController =
      FloatingActionBarController();
  FloatingActionBarController alignEditorController =
      FloatingActionBarController();

  void _saveNotes(BuildContext context) {
    String title = "";
    final content = jsonEncode(widget.controller.document.toDelta().toJson());
    final plainText = widget.controller.document.toPlainText();
    final firstNewLineIndex = plainText.indexOf('\n');
    if (firstNewLineIndex != -1) {
      title = plainText.substring(0, firstNewLineIndex);
    }

    if (plainText.isNotEmpty && plainText != '\n') {
      if (widget.noteId != null && widget.noteId!.isNotEmpty) {
        context.read<NotesBloc>().add(
          UpdateNotes(
            id: widget.noteId!,
            content: content,
            title: title,
            plainText: plainText,
          ),
        );
      } else {
        context.read<NotesBloc>().add(CreateNotes(title, content, plainText));
      }
    }
    Navigator.pop(context);
  }

  void toolbarSelection(
    BuildContext context,
    ToolbarEnum value,
    ToolbarEnum currentValue,
  ) {
    print("toolbarSelection value: $value currentValue: $currentValue");

    if (value == currentValue) {
      context.read<EditorBloc>().add(
        SelectToolbar(activeToolbar: ToolbarEnum.none),
      );
    }

    if (value != currentValue) {
      context.read<EditorBloc>().add(SelectToolbar(activeToolbar: value));
      if (value == ToolbarEnum.align) {
        textEditorController.close();
      } else if (value == ToolbarEnum.text) {
        alignEditorController.close();
      }
    }
  }

  void _undoCall() {
    widget.controller.undo();
  }

  void _redoCall() {
    widget.controller.redo();
  }

  @override
  Widget build(BuildContext context) {
    return BlocSelector<EditorBloc, EditorBlocState, ToolbarEnum>(
      selector: (state) => state.selectedToolbar,
      builder: (context, toolbarSelected) {
        return MechanixBottomBar(
          theme:
              toolbarSelected != ToolbarEnum.none
                  ? const MechanixBottomBarThemeData(
                    iconTheme: MechanixBottomBarIconThemeData(
                      buttonMargin: EdgeInsets.all(10),
                    ),
                    decoration: BoxDecoration(
                      borderRadius: BorderRadius.all(Radius.circular(0)),
                    ),
                  )
                  : null,
          leadingWidget: [
            BottomBarButton(
              iconWidget: const IconWidget(
                iconPath: NotesIcon.backIcon,
                iconHeight: 28,
                iconWidth: 28,
                boxHeight: 30,
                boxWidth: 30,
                iconColor: Colors.white,
              ),
              onPressed: () => _saveNotes(context),
            ),
          ],

          anchorWidgetSpacing: 0,
          centerWidgetSpacing: 16,
          leadingWidgetSpacing: 20,
          centerWidget: [
            BottomBarButton.extension(
              outsideClickDisabled: true,
              floatingActionBarTheme: const MechanixFloatingActionBarThemeData(
                decoration: BoxDecoration(
                  color: NotesColors.floatingMenuColor,
                  borderRadius: BorderRadius.only(
                    topLeft: Radius.circular(8),
                    topRight: Radius.circular(8),
                  ),
                ),
              ),
              onExtensionClose:
                  () => toolbarSelection(
                    context,
                    ToolbarEnum.text,
                    toolbarSelected,
                  ),
              onPressed: () {
                toolbarSelection(context, ToolbarEnum.text, toolbarSelected);
              },
              dropdownPosition: DropdownPosition.topCenter,
              offset: const Offset(92, -8),

              // floatingActionBarController: textEditorController,
              menuButton: IconWidget(
                iconPath: NotesIcon.textStyleIcon,
                iconHeight: 28,
                iconWidth: 28,
                boxHeight: 44,
                boxWidth: 44,
                activeIconColor: NotesColors.secondaryTextColor,
                isActive: toolbarSelected == ToolbarEnum.text,
                iconColor: Colors.white,
              ),

              extensionWidgets: [
                BottomBarButton.widget(
                  widget: TextEditorToolbar(controller: widget.controller),
                ),
              ],
            ),

            BottomBarButton.extension(
              floatingActionBarTheme: const MechanixFloatingActionBarThemeData(
                decoration: BoxDecoration(
                  color: NotesColors.floatingMenuColor,
                  borderRadius: BorderRadius.only(
                    topLeft: Radius.circular(8),
                    topRight: Radius.circular(8),
                  ),
                ),
              ),
              offset: const Offset(32, -8),
              outsideClickDisabled: true,
              onExtensionClose:
                  () => toolbarSelection(
                    context,
                    ToolbarEnum.none,
                    toolbarSelected,
                  ),
              onPressed:
                  () => toolbarSelection(
                    context,
                    ToolbarEnum.align,
                    toolbarSelected,
                  ),
              // floatingActionBarController: alignEditorController,
              menuButton: GestureDetector(
                child: Container(
                  height: 44,
                  width: 44,
                  padding: const EdgeInsets.all(8),
                  decoration: BoxDecoration(
                    borderRadius: BorderRadius.circular(8),
                    color:
                        toolbarSelected == ToolbarEnum.align
                            ? NotesColors.cardColor.withValues(alpha: 0.5)
                            : Colors.transparent,
                  ),
                  child: Image.asset(NotesIcon.menuIcon, height: 28, width: 28),
                ),
              ),
              // IconWidget(
              //   iconPath: NotesIcon.menuIcon,
              //   iconHeight: 28,
              //   iconWidth: 28,
              //   boxHeight: 44,
              //   boxWidth: 44,
              //   activeIconColor: NotesColors.secondaryTextColor,
              //   isActive: true,
              //   iconColor:
              //       toolbarSelected == ToolbarEnum.align
              //           ? NotesColors.secondaryTextColor
              //           : Colors.white,
              // ),
              extensionWidgets: [
                BottomBarButton.widget(
                  widget: AlignmentToolbar(controller: widget.controller),
                ),
              ],
            ),
            BottomBarButton.widget(
              widget: BlocSelector<EditorBloc, EditorBlocState, bool>(
                selector: (state) => state.isUndo,
                builder:
                    (context, isUndo) => IconButton(
                      icon: Image.asset(
                        NotesIcon.undoIcon,
                        width: 28,
                        height: 28,
                        color:
                            isUndo
                                ? Colors.white
                                : Theme.of(context).disabledColor,
                      ),
                      iconSize: 44,
                      onPressed: isUndo ? _undoCall : null,
                    ),
              ),
            ),
            BottomBarButton.widget(
              widget: BlocSelector<EditorBloc, EditorBlocState, bool>(
                selector: (state) => state.isRedo,
                builder:
                    (context, isRedo) => IconButton(
                      icon: Image.asset(
                        NotesIcon.redoIcon,
                        width: 28,
                        height: 28,
                        color:
                            isRedo
                                ? Colors.white
                                : Theme.of(context).disabledColor,
                      ),
                      iconSize: 44,
                      onPressed: isRedo ? _redoCall : null,
                    ),
              ),
            ),
          ],
          anchorWidget: [
            BottomBarButton.widget(widget: EditorMenu(noteId: widget.noteId)),
          ],
        );
      },
    );
  }
}
