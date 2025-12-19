/*
MIT License

Copyright (c) 2024 Flutter Quill project and open source contributors.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

import 'package:flutter_quill/flutter_quill.dart';

enum BlockFormatStyle { todo, bullet, ordered, header, codeblock }

bool handleFormatBlockStyleBySpaceEvent({
  required QuillController controller,
  required String character,
  required BlockFormatStyle formatStyle,
}) {
  assert(
    character.trim().isNotEmpty && character != '\n',
    'Expected character that cannot be empty, a whitespace or a new line. Got $character',
  );
  if (formatStyle == BlockFormatStyle.todo) {
    _updateSelectionForKeyPhrase(character, Attribute.unchecked, controller);
    return true;
  } else if (formatStyle == BlockFormatStyle.bullet) {
    _updateSelectionForKeyPhrase(character, Attribute.ul, controller);
    return true;
  } else if (formatStyle == BlockFormatStyle.ordered) {
    _updateSelectionForKeyPhrase(character, Attribute.ol, controller);
    return true;
  } else if (formatStyle == BlockFormatStyle.header) {
    var headerAttribute = Attribute.header as Attribute<int?>;
    final count = _count(character, '#');
    if (count == 1) {
      headerAttribute = Attribute.h1;
    } else if (count == 2) {
      headerAttribute = Attribute.h2;
    } else if (count == 3) {
      headerAttribute = Attribute.h3;
    }
    _updateSelectionForKeyPhrase(character, headerAttribute, controller);
    return true;
  } else if (formatStyle == BlockFormatStyle.codeblock) {
    _updateSelectionForKeyPhrase(character, Attribute.codeBlock, controller);
    return true;
  }

  return false;
}

void _updateSelectionForKeyPhrase(
  String phrase,
  Attribute attribute,
  QuillController controller,
) {
  controller.replaceText(
    controller.selection.baseOffset - phrase.length,
    phrase.length,
    '\n',
    null,
  );
  _moveCursor(-phrase.length, controller);
  controller
    ..formatSelection(attribute)
    // Remove the added newline.
    ..replaceText(controller.selection.baseOffset + 1, 1, '', null);
}

void _moveCursor(int chars, QuillController controller) {
  final selection = controller.selection;
  controller.updateSelection(
    controller.selection.copyWith(
      baseOffset: selection.baseOffset + chars,
      extentOffset: selection.baseOffset + chars,
    ),
    ChangeSource.local,
  );
}

int _count(String char, String matchChar) {
  var count = 0;
  for (var i = 0; i < char.length; i++) {
    if (char[i] == matchChar) {
      count++;
    } else {
      break;
    }
  }
  return count;
}
