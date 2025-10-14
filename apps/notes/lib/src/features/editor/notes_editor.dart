import 'dart:convert';
import 'dart:io' as io;
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/models/note_hive.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_bloc.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_event.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_state.dart';
import 'package:mechanix_notes/src/features/editor/content_editor.dart';
import 'package:mechanix_notes/src/features/editor/editor_bar.dart';
import 'package:mechanix_notes/src/features/editor/editor_bottom_menu.dart';
import 'package:mechanix_notes/src/features/editor/toolbar_selection.dart';
import 'package:mechanix_notes/src/features/home/models/toolbar_model.dart';
import "package:path/path.dart" as path;
import 'package:flutter/material.dart';
import 'package:flutter_quill/flutter_quill.dart';

class NotesEditor extends StatefulWidget {
  final NoteHive? note;
  const NotesEditor({super.key, this.note});

  @override
  State<NotesEditor> createState() => _NotesEditorState();
}

class _NotesEditorState extends State<NotesEditor> {
  final FocusNode _focusNode = FocusNode();
  bool _isLoading = false;

  final QuillController _controller = QuillController(
    document: Document(),
    selection: TextSelection.collapsed(offset: 0),
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
    bool isEditing = widget.note != null;

    context.read<EditorBloc>().add(
      InitializedEditor(isPinned: widget.note?.isPinned ?? false),
    );

    if (isEditing) {
      _initializeControllerAsync();
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

  void _initializeControllerAsync() {
    setState(() => _isLoading = true);

    final doc = Document.fromJson(jsonDecode(widget.note!.content));
    _controller.document = doc;

    if (mounted) {
      setState(() => _isLoading = false);
    }
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
        preferredSize: Size.fromHeight(50),
        child: EditorBar(controller: _controller, note: widget.note),
      ),
      body:
          _isLoading
              ? Center(
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    CircularProgressIndicator(),
                    SizedBox(height: 16),
                    Text(
                      'Loading Notes...',
                      style: TextStyle(color: Colors.grey[600], fontSize: 14),
                    ),
                  ],
                ),
              )
              : GestureDetector(
                behavior: HitTestBehavior.translucent,
                onTap: () {
                  toolbarSelection(ToolbarEnum.none);
                },
                child: Stack(
                  children: [
                    // Main content
                    ContentEditor(
                      controller: _controller,
                      focusNode: _focusNode,
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
                    EditorBottomMenu(
                      controller: _controller,
                      onToolbarSelection: toolbarSelection,
                    ),
                  ],
                ),
              ),
    );
  }

  @override
  void dispose() {
    _controller.removeListener(_onControllerChange);
    _controller.dispose();
    _focusNode.dispose();
    super.dispose();
  }
}
