import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/commons/icons.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_bloc.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_event.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_state.dart';
import 'package:mechanix_notes/src/features/editor/editor_menu.dart';
import 'package:mechanix_notes/src/features/editor/models/toolbar_models.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/alignment_toolbar.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/focus_preserve_button.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/text_editor_toolbar.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:widgets/extensions/color.dart';
import 'package:widgets/widgets.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottom_bar/mechanix_bottom_bar_theme.dart';
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
    final plainText = widget.controller.document.toPlainText();
    final trimmedText = plainText.trim();

    // Don't save if there's no actual content
    if (trimmedText.isEmpty) {
      textEditorController.close();
      alignEditorController.close();
      Navigator.pop(context);
      return;
    }

    String title = "";
    final content = jsonEncode(widget.controller.document.toDelta().toJson());

    // Split by newlines and find the first non-empty line
    final lines = plainText.split('\n');
    for (final line in lines) {
      final trimmedLine = line.trim();
      if (trimmedLine.isNotEmpty) {
        title = trimmedLine;
        break;
      }
    }

    // If no non-empty line found (shouldn't happen due to trimmedText check, but safe fallback)
    if (title.isEmpty) {
      title = trimmedText.replaceAll('\n', ' ').trim();
    }

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

    context.read<NotesBloc>().add(LoadNotes());
    textEditorController.close();
    alignEditorController.close();
    Navigator.pop(context);
  }

  void requestFocus() {
    if (!widget.focusNode.hasFocus) widget.focusNode.requestFocus();
  }

  void onPress(ToolbarEnum oldValue, ToolbarEnum newValue) {
    requestFocus();
    if (oldValue == ToolbarEnum.none) {
      context.read<EditorBloc>().add(SelectToolbar(activeToolbar: newValue));
    } else {
      if (oldValue == ToolbarEnum.align) {
        alignEditorController.close();
        context.read<EditorBloc>().add(
          SelectToolbar(activeToolbar: ToolbarEnum.text),
        );
      } else {
        textEditorController.close();
        context.read<EditorBloc>().add(
          SelectToolbar(activeToolbar: ToolbarEnum.align),
        );
      }
    }
  }

  void onClose(ToolbarEnum currentValue) {
    requestFocus();
    context.read<EditorBloc>().add(
      SelectToolbar(activeToolbar: ToolbarEnum.none),
    );
  }

  void _undoCall() {
    requestFocus();
    widget.controller.undo();
  }

  void _redoCall() {
    requestFocus();
    widget.controller.redo();
  }

  @override
  void dispose() {
    textEditorController.dispose();
    alignEditorController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return BlocSelector<EditorBloc, EditorBlocState, ToolbarEnum>(
      selector: (state) => state.selectedToolbar,
      builder: (context, toolbarSelected) {
        return MechanixBottomBar(
          theme:
              toolbarSelected != ToolbarEnum.none
                  ? MechanixBottomBarThemeData(
                    height: 90,
                    decoration: BoxDecoration(
                      color: context.secondaryContainer,
                      borderRadius: const BorderRadius.all(Radius.circular(0)),
                      // boxShadow: [
                      //   BoxShadow(
                      //     offset: Offset(0, 0),
                      //     color: Color(0x99000000),
                      //   ),
                      //   BoxShadow(
                      //     offset: Offset(0, 0),
                      //     color: Color(0x40000000),
                      //   ),
                      //   BoxShadow(
                      //     offset: Offset(0, 0),
                      //     color: Color(0x40000000),
                      //   ),
                      // ],
                    ),
                  )
                  : null,
          leadingWidget: [
            BottomBarButton(
              iconTheme: const MechanixBottomBarIconThemeData(
                buttonMargin: EdgeInsets.only(left: 5),
              ),
              iconWidget: const IconWidget(
                iconPath: NotesIcon.backIcon,
                iconHeight: 28,
                iconWidth: 28,
                boxHeight: 30,
                boxWidth: 30,
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
              floatingActionBarTheme: MechanixFloatingActionBarThemeData(
                width: double.infinity,
                decoration: BoxDecoration(
                  color: context.secondary,
                  borderRadius: const BorderRadius.only(
                    topLeft: Radius.circular(12),
                    topRight: Radius.circular(12),
                  ),
                ),
              ),
              onExtensionClose: () {
                onClose(toolbarSelected);
              },

              onPressed: () {
                onPress(toolbarSelected, ToolbarEnum.text);
              },
              isSelected: toolbarSelected == ToolbarEnum.text,
              iconTheme: MechanixBottomBarIconThemeData(
                activeButtonDecoration: BoxDecoration(
                  color: context.secondary,
                  borderRadius: BorderRadius.circular(8),
                ),
              ),
              dropdownPosition: DropdownPosition.topCenter,
              offset: const Offset(90, -8),
              floatingActionBarController: textEditorController,
              iconWidget: IconWidget(
                iconPath: NotesIcon.textStyleIcon,
                iconHeight: 28,
                iconWidth: 28,
                boxHeight: 44,
                boxWidth: 44,
                activeIconColor: context.primaryContainer,
                isActive: toolbarSelected == ToolbarEnum.text,
              ),

              extensionWidgets: [
                BottomBarButton.widget(
                  widget: TextEditorToolbar(
                    controller: widget.controller,
                    focusNode: widget.focusNode,
                  ),
                ),
              ],
            ),

            BottomBarButton.extension(
              floatingActionBarTheme: MechanixFloatingActionBarThemeData(
                width: double.infinity,
                decoration: BoxDecoration(
                  color: context.secondary,
                  borderRadius: const BorderRadius.only(
                    topLeft: Radius.circular(12),
                    topRight: Radius.circular(12),
                  ),
                ),
              ),

              offset: const Offset(30, -8),
              outsideClickDisabled: true,
              isSelected: toolbarSelected == ToolbarEnum.align,

              onExtensionClose: () => onClose(toolbarSelected),
              onPressed: () => onPress(toolbarSelected, ToolbarEnum.align),
              floatingActionBarController: alignEditorController,
              iconTheme: MechanixBottomBarIconThemeData(
                activeButtonDecoration: BoxDecoration(
                  color: context.secondary,
                  borderRadius: BorderRadius.circular(8),
                ),
              ),
              iconWidget: IconWidget(
                iconPath: NotesIcon.menuIcon,
                iconHeight: 28,
                iconWidth: 28,
                boxHeight: 44,
                boxWidth: 44,
                activeIconColor: context.primaryContainer,
                isActive: toolbarSelected == ToolbarEnum.align,
              ),
              extensionWidgets: [
                BottomBarButton.widget(
                  widget: AlignmentToolbar(
                    focusNode: widget.focusNode,
                    controller: widget.controller,
                  ),
                ),
              ],
            ),
            BottomBarButton.widget(
              widget: BlocSelector<EditorBloc, EditorBlocState, bool>(
                selector: (state) => state.isUndo,
                builder:
                    (context, isUndo) => FocusPreserveButton(
                      child: IconButton(
                        icon: IconWidget(
                          iconPath: NotesIcon.undoIcon,
                          iconHeight: 28,
                          iconWidth: 28,
                          boxHeight: 28,
                          boxWidth: 28,
                          iconColor:
                              isUndo ? null : Theme.of(context).disabledColor,
                        ),
                        iconSize: 44,
                        onPressed: isUndo ? _undoCall : null,
                      ),
                    ),
              ),
            ),
            BottomBarButton.widget(
              widget: BlocSelector<EditorBloc, EditorBlocState, bool>(
                selector: (state) => state.isRedo,
                builder:
                    (context, isRedo) => FocusPreserveButton(
                      child: IconButton(
                        icon: IconWidget(
                          iconPath: NotesIcon.redoIcon,
                          iconHeight: 28,
                          iconWidth: 28,
                          boxHeight: 28,
                          boxWidth: 28,
                          iconColor:
                              isRedo ? null : Theme.of(context).disabledColor,
                        ),
                        iconSize: 44,
                        onPressed: isRedo ? _redoCall : null,
                      ),
                    ),
              ),
            ),
          ],
          anchorWidget: [
            BottomBarButton.widget(
              widget: EditorMenu(
                noteId: widget.noteId,
                onTapFocus: requestFocus,
              ),
            ),
          ],
        );
      },
    );
  }
}
