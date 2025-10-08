import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_bloc.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_state.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/additional_toolbar.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/alignment_toolbar.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/text_editor_toolbar.dart';
import 'package:mechanix_notes/src/features/home/models/toolbar_model.dart';

class ToolbarSelection extends StatelessWidget {
  final FocusNode focusNode;
  final QuillController controller;

  const ToolbarSelection({
    super.key,
    required this.focusNode,
    required this.controller,
  });

  @override
  Widget build(BuildContext context) {
    return BlocSelector<EditorBloc, EditorBlocState, ToolbarEnum>(
      selector: (state) => state.selectedToolbar,
      builder: (context, selectedToolbar) {
        // Determine which toolbar to show
        Widget toolbar;
        switch (selectedToolbar) {
          case ToolbarEnum.align:
            toolbar = AlignmentToolbar(
              controller: controller,
              focusNode: focusNode,
            );
            break;
          case ToolbarEnum.text:
            toolbar = TextEditorToolbar(
              controller: controller,
              focusNode: focusNode,
            );
            break;
          case ToolbarEnum.add:
            toolbar = AdditionalToolbar(
              controller: controller,
              focusNode: focusNode,
            );
            break;
          case ToolbarEnum.none:
            toolbar = const SizedBox.shrink();
        }

        return Positioned(
          left: 0,
          right: 0,
          bottom: 90,
          child: Center(
            child: Container(
              margin: const EdgeInsets.symmetric(horizontal: 16),
              child: toolbar,
            ),
          ),
        );
      },
    );
  }
}
