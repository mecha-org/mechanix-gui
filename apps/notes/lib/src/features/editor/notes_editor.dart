import 'dart:io' as io;
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_bloc.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_event.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_state.dart';
import 'package:mechanix_notes/src/features/editor/content_editor.dart';
import 'package:mechanix_notes/src/features/editor/editor_bottom_bar.dart';
import 'package:mechanix_notes/src/features/editor/models/toolbar_models.dart';
import "package:path/path.dart" as path;
import 'package:flutter/material.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:widgets/extensions/edge_insets.dart';
import 'package:widgets/widgets.dart';
import 'package:widgets/widgets/floating_action_bar/mechanix_floating_action_bar.dart';

class NotesEditor extends StatefulWidget {
  final String? noteId;
  const NotesEditor({super.key, this.noteId});

  @override
  State<NotesEditor> createState() => _NotesEditorState();
}

class _NotesEditorState extends State<NotesEditor> {
  final FocusNode _focusNode = FocusNode();

  /// Flag to apply style to first new line
  bool _isFormatting = false;

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
    final bool isEditing = widget.noteId != null;

    if (isEditing) {
      context.read<EditorBloc>().add(LoadNoteContent(noteId: widget.noteId!));
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

  // Helper method to check if a line has restricted formatting
  bool _hasRestrictedFormatting(int offset) {
    try {
      final line = _controller.document.queryChild(offset).node;

      // Check for restricted attributes
      final restrictedAttributes = [
        Attribute.ul.key, // Bullet list
        Attribute.ol.key, // Numbered list
        Attribute.checked.key, // Checked checkbox
        Attribute.unchecked.key, // Unchecked checkbox
        Attribute.codeBlock.key, // Code block
      ];

      for (final attrKey in restrictedAttributes) {
        if (line?.style.attributes.containsKey(attrKey) ?? false) {
          return true;
        }
      }

      return false;
    } catch (e) {
      print('Error checking formatting: $e');
      return false;
    }
  }

  void _onControllerChange() {
    context.read<EditorBloc>().add(UndoUpdate(isUndo: _controller.hasUndo));
    context.read<EditorBloc>().add(RedoUpdate(isRedo: _controller.hasRedo));

    if (_isFormatting) return;

    final sel = _controller.selection;
    if (!sel.isCollapsed) return;

    final pos = sel.baseOffset;
    if (pos < 0) return;

    final plain = _controller.document.toPlainText();
    final firstNewLineIndex = plain.indexOf('\n');

    if (firstNewLineIndex == -1) return;

    if (pos == firstNewLineIndex + 1 && !_isFormatting) {
      // Check if the first line has restricted formatting
      if (_hasRestrictedFormatting(0)) {
        return;
      }
      setState(() {
        _isFormatting = true;
      });

      try {
        _controller.formatText(0, firstNewLineIndex, Attribute.h1);
      } finally {
        // ← ALWAYS RESET THE FLAG
        setState(() {
          _isFormatting = false;
        });
      }
    }
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
      bottomNavigationBar: EditorBottomBar(
        controller: _controller,
        focusNode: _focusNode,
        noteId: widget.noteId,
      ),
      body: Column(
        children: [
          // Main content
          BlocBuilder<EditorBloc, EditorBlocState>(
            buildWhen:
                (previous, current) => previous.isLoading != current.isLoading,
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
        ],
      ).padSymmetric(vertical: 15, horizontal: 24),
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
