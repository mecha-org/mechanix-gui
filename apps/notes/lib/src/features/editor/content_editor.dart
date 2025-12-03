import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/commons/styles/quill_editor_styles.dart';

class ContentEditor extends StatelessWidget {
  final QuillController controller;
  final FocusNode focusNode;

  const ContentEditor({
    super.key,
    required this.controller,
    required this.focusNode,
  });

  @override
  Widget build(BuildContext context) {
    return Expanded(
      child: QuillEditor(
        scrollController: ScrollController(),
        controller: controller,
        focusNode: focusNode,
        config: QuillEditorConfig(
          expands: false,
          scrollable: true,
          enableSelectionToolbar: false,
          spaceShortcutEvents: [
            formatHyphenToBulletList,
            formatHeaderToHeaderStyle,
            formatHeader2ToHeaderStyle,
            formatHeader3ToHeaderStyle,
            formatOrderedNumberToList,
          ],
          characterShortcutEvents: [
            formatStrikeToStrikethrough,
            formatDoubleUnderscoresToBold,
            formatAsterisksToItalic,
            formatDoubleAsterisksToBold,
          ],

          customStyles: quillEditorStyle,
          enableScribble: false,
          autoFocus: false,
          enableInteractiveSelection: true,
        ),
      ),
    );
  }
}
