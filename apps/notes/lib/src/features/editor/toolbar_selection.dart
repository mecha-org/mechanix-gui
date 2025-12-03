import 'package:flutter/material.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/alignment_toolbar.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/text_editor_toolbar.dart';
import 'package:mechanix_notes/src/features/editor/models/toolbar_models.dart';

class ToolbarSelection extends StatelessWidget {
  final FocusNode focusNode;
  final QuillController controller;
  final ToolbarEnum selectedToolbar;

  const ToolbarSelection({
    super.key,
    required this.focusNode,
    required this.controller,
    required this.selectedToolbar,
  });

  Widget selectToolbar() {
    switch (selectedToolbar) {
      case ToolbarEnum.align:
        return AlignmentToolbar(controller: controller, focusNode: focusNode);
      case ToolbarEnum.text:
        return TextEditorToolbar(controller: controller, focusNode: focusNode);
      case ToolbarEnum.none:
        return const SizedBox.shrink();
    }
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      child: Container(
        margin: const EdgeInsets.symmetric(horizontal: 16),
        child: selectToolbar(),
      ),
    );
  }
}
