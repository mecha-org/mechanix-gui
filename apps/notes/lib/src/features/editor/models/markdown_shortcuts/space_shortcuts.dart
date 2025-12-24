import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/features/editor/models/markdown_shortcuts/space_shortcuts_event.dart';

final SpaceShortcutEvent formatBracesForTodoList = SpaceShortcutEvent(
  character: '[]',
  handler:
      (node, controller) => handleFormatBlockStyleBySpaceEvent(
        controller: controller,
        character: '[]',
        formatStyle: BlockFormatStyle.todo,
      ),
);

final SpaceShortcutEvent formatAsterikForBulletList = SpaceShortcutEvent(
  character: '*',
  handler:
      (node, controller) => handleFormatBlockStyleBySpaceEvent(
        controller: controller,
        character: '*',
        formatStyle: BlockFormatStyle.bullet,
      ),
);

final SpaceShortcutEvent formatHyphenForBulletList = SpaceShortcutEvent(
  character: '-',
  handler:
      (node, controller) => handleFormatBlockStyleBySpaceEvent(
        controller: controller,
        character: '-',
        formatStyle: BlockFormatStyle.bullet,
      ),
);

final SpaceShortcutEvent formatNumberedForOrderedList = SpaceShortcutEvent(
  character: '1.',
  handler:
      (node, controller) => handleFormatBlockStyleBySpaceEvent(
        controller: controller,
        character: '1.',
        formatStyle: BlockFormatStyle.ordered,
      ),
);

final SpaceShortcutEvent formatTripleBackTickForCodeBlock = SpaceShortcutEvent(
  character: '```',
  handler:
      (node, controller) => handleFormatBlockStyleBySpaceEvent(
        controller: controller,
        character: '```',
        formatStyle: BlockFormatStyle.codeblock,
      ),
);