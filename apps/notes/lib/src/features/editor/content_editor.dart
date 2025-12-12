import 'package:flutter/material.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';
import 'package:mechanix_notes/src/commons/styles/quill_editor_styles.dart';
import 'package:mechanix_notes/src/features/editor/leading_widget/bullet_list_builder.dart';
import 'package:mechanix_notes/src/features/editor/leading_widget/number_list_builder.dart';
import 'package:mechanix_notes/src/features/editor/selection_options.dart';
import 'package:mechanix_notes/src/features/editor/toolbar/focus_preserve_button.dart';

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
          padding: const EdgeInsets.only(
            top: 15,
            bottom: 15,
            right: 24,
            left: 24,
          ),
          expands: false,
          maxContentWidth: double.infinity,
          scrollable: true,
          enableSelectionToolbar: true,
          customLeadingBlockBuilder: (Node node, LeadingConfig config) {
            final attr = config.attribute;

            // CHECKBOX LIST (UNCHECKED)
            if (attr.value == Attribute.unchecked.value) {
              return FocusPreserveButton(
                child: Container(
                  width: 16,
                  height: 21,
                  padding: const EdgeInsets.only(top: 8, right: 0),
                  child: SizedBox(
                    width: 16,
                    height: 16,
                    child: Checkbox(
                      hoverColor: Colors.transparent,
                      focusColor: Colors.transparent,
                      overlayColor: const WidgetStatePropertyAll(
                        Colors.transparent,
                      ),
                      value: false,
                      onChanged: (state) {
                        if (state == true) {
                          _applyStrikethrough(controller, node, true);
                        }
                        config.onCheckboxTap(state!);
                      },
                      checkColor: Colors.white,
                      fillColor: WidgetStateProperty.resolveWith<Color>((
                        Set<WidgetState> states,
                      ) {
                        if (states.contains(WidgetState.selected)) {
                          return NotesColors.secondaryCardColor;
                        }
                        return Colors.transparent;
                      }),
                    ),
                  ),
                ),
              );
            }

            // CHECKBOX LIST (CHECKED)
            if (attr.value == Attribute.checked.value) {
              return FocusPreserveButton(
                child: Padding(
                  padding: const EdgeInsets.only(top: 8, right: 0),
                  child: SizedBox(
                    width: 16,
                    height: 16,
                    child: Checkbox(
                      hoverColor: Colors.transparent,
                      focusColor: Colors.transparent,
                      overlayColor: const WidgetStatePropertyAll(
                        Colors.transparent,
                      ),
                      value: true,
                      onChanged: (state) {
                        if (state == false) {
                          // Remove strikethrough when unchecking
                          _applyStrikethrough(controller, node, false);
                        }
                        config.onCheckboxTap(state!);
                      },
                      checkColor: Colors.white,
                      fillColor: WidgetStateProperty.resolveWith<Color>((
                        Set<WidgetState> states,
                      ) {
                        if (states.contains(WidgetState.selected)) {
                          return NotesColors.secondaryCardColor;
                        }
                        return Colors.transparent;
                      }),
                    ),
                  ),
                ),
              );
            }

            // BULLET LIST
            if (attr.value == Attribute.ul.value) {
              return const BulletListBuilder();
            }

            // NUMBERED LIST
            if (attr.value == Attribute.ol.value) {
              final number =
                  config.getIndexNumberByIndent ?? config.count.toString();

              return NumberListBuilder(number: number);
            }

            return null;
          },

          contextMenuBuilder: (context, rawEditorState) {
            return AdaptiveTextSelectionToolbar(
              anchors: rawEditorState.contextMenuAnchors,
              children: const [SelectionOptions()],
            );
          },
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

  // Helper method to apply/remove strikethrough
  void _applyStrikethrough(QuillController controller, Node node, bool apply) {
    try {
      // Get the line's offset and length
      final offset = node.documentOffset;
      final length = node.length - 1; // -1 to exclude the newline character

      if (length > 0) {
        // Apply or remove strikethrough
        if (apply) {
          controller.formatText(offset, length, Attribute.strikeThrough);
        } else {
          controller.formatText(
            offset,
            length,
            Attribute.clone(Attribute.strikeThrough, null),
          );
        }
      }
    } catch (_) {}
  }
}
