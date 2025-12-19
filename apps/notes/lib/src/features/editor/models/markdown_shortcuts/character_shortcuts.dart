import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/features/editor/models/markdown_shortcuts/character_shortcuts_event.dart';
import 'package:mechanix_notes/src/features/editor/models/markdown_shortcuts/format_double_character.dart';

const _asterisk = '*';
const _underscore = '_';
const _inlineCode = '`';

final CharacterShortcutEvent formatBackTickForInlineCode =
    CharacterShortcutEvent(
      key: 'Format ` to inline',
      character: _inlineCode,
      handler:
          (controller) => handleFormatByWrappingWithSingleCharacter(
            controller: controller,
            character: _inlineCode,
            formatStyle: SingleCharacterFormatStyle.inline,
          ),
    );

final CharacterShortcutEvent formatAsterikForItalic = CharacterShortcutEvent(
  key: 'Format * to italic',
  character: _asterisk,
  handler:
      (controller) => handleFormatByWrappingWithSingleCharacter(
        controller: controller,
        character: _asterisk,
        formatStyle: SingleCharacterFormatStyle.italic,
      ),
);

final CharacterShortcutEvent formatDoubleAsterisksForBold =
    CharacterShortcutEvent(
      key: 'Format ** to bold',
      character: _asterisk,
      handler:
          (controller) => handleFormatByWrappingWithDoubleCharacter(
            controller: controller,
            character: _asterisk,
            formatStyle: DoubleCharacterFormatStyle.bold,
          ),
    );

final CharacterShortcutEvent formatDoubleUnderScoreForUnderline =
    CharacterShortcutEvent(
      key: 'Format __ to underscore',
      character: _underscore,
      handler:
          (controller) => handleFormatByWrappingWithDoubleCharacter(
            controller: controller,
            character: _underscore,
            formatStyle: DoubleCharacterFormatStyle.underline,
          ),
    );
