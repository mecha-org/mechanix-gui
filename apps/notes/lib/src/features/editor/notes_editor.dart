import 'dart:io' as io;
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_bloc.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_event.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_state.dart';
import 'package:mechanix_notes/src/features/editor/content_editor.dart';
import 'package:mechanix_notes/src/features/editor/editor_bar.dart';
import 'package:mechanix_notes/src/features/editor/editor_bottom_menu.dart';
import 'package:mechanix_notes/src/features/editor/models/toolbar_models.dart';
import 'package:mechanix_notes/src/features/editor/toolbar_selection.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';
import "package:path/path.dart" as path;
import 'package:flutter/material.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:widgets/widgets/floating_action_bar/mechanix_floating_action_bar.dart';

class NotesEditor extends StatefulWidget {
  final NoteMetaData? note;
  const NotesEditor({super.key, this.note});

  @override
  State<NotesEditor> createState() => _NotesEditorState();
}

class _NotesEditorState extends State<NotesEditor> {
  final FocusNode _focusNode = FocusNode();
  final FloatingActionBarController floatingBar = FloatingActionBarController();
  final QuillController _controller = QuillController(
    document: Document(),
    selection: const TextSelection.collapsed(offset: 0),
    config: QuillControllerConfig(
      requireScriptFontFeatures: false,
      clipboardConfig: QuillClipboardConfig(
        enableExternalRichPaste: true,
        onImagePaste: (imageBytes) async {
          final newFileName =
              'image-file-${DateTime.now().toIso8601String()}.png';
          final newPath = path.join(io.Directory.systemTemp.path, newFileName);
          final file = await io.File(
            newPath,
          ).writeAsBytes(imageBytes, flush: true);
          return file.path;
        },
      ),
    ),
  );

  @override
  void initState() {
    super.initState();
    final isEditing = widget.note != null;

    if (isEditing) {
      context.read<EditorBloc>().add(LoadNoteContent(noteId: widget.note!.id));
    } else {
      _openKeyboardAfterLoad();
    }
    _controller.addListener(_onControllerChange);
  }

  void _openKeyboardAfterLoad() {
    WidgetsBinding.instance.addPostFrameCallback((_) async {
      if (!mounted) return;

      await Future.delayed(const Duration(milliseconds: 300));
      _focusNode.requestFocus();
    });
  }

  void _onControllerChange() {
    context.read<EditorBloc>().add(UndoUpdate(isUndo: _controller.hasUndo));
    context.read<EditorBloc>().add(RedoUpdate(isRedo: _controller.hasRedo));
  }

  void toolbarSelection(ToolbarEnum value) {
    if (_focusNode.hasFocus) {
      _focusNode.unfocus();
    }
    context.read<EditorBloc>().add(SelectToolbar(activeToolbar: value));
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: PreferredSize(
        preferredSize: const Size.fromHeight(50),
        child: EditorBar(
          controller: _controller,
          note: widget.note,
          floatingBar: floatingBar,
        ),
      ),
      body: GestureDetector(
        behavior: HitTestBehavior.translucent,
        onTap: () {
          toolbarSelection(ToolbarEnum.none);
        },
        child: Stack(
          children: [
            // Main content
            BlocBuilder<EditorBloc, EditorBlocState>(
              buildWhen:
                  (previous, current) =>
                      previous.isLoading != current.isLoading,
              builder: (context, state) {
                if (state.isLoading) {
                  return const Center(child: CircularProgressIndicator());
                }

                if (state.document != null &&
                    _controller.document != state.document) {
                  // Load document into controller only once
                  _controller.document = state.document!;
                }

                return ContentEditor(
                  controller: _controller,
                  focusNode: _focusNode,
                );
              },
            ),

            // Toolbar
            BlocSelector<EditorBloc, EditorBlocState, ToolbarEnum>(
              selector: (state) => state.selectedToolbar,
              builder:
                  (context, selectedToolbar) => ToolbarSelection(
                    selectedToolbar: selectedToolbar,
                    focusNode: _focusNode,
                    controller: _controller,
                  ),
            ),

            BlocListener<EditorBloc, EditorBlocState>(
              listenWhen:
                  (previous, current) =>
                      previous.toolbarToggle != current.toolbarToggle,
              listener: (context, state) {
                if (state.toolbarToggle) {
                  floatingBar.open();
                } else {
                  floatingBar.close();
                }
              },
              child: EditorBottomMenu(
                barController: floatingBar,
                controller: _controller,
                onToolbarSelection: toolbarSelection,
              ),
            ),
          ],
        ),
      ),
    );
  }

  @override
  void dispose() {
    floatingBar.dispose();
    _controller.removeListener(_onControllerChange);
    _controller.dispose();
    _focusNode.dispose();
    super.dispose();
  }
}
