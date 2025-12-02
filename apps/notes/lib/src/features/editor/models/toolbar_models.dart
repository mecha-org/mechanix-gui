import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/commons/icons.dart';

enum ToolbarEnum { text, align,  none }

class ToolbarItem {
  final Attribute attribute;
  final String key;
  final String iconPath;

  const ToolbarItem({
    required this.attribute,
    required this.key,
    required this.iconPath,
  });
}

List<ToolbarItem> kTextEditingToolbarItems = [
  ToolbarItem(
    attribute: Attribute.bold,
    key: Attribute.bold.key,
    iconPath: NotesIcon.boldIcon,
  ),
  ToolbarItem(
    attribute: Attribute.italic,
    key: Attribute.italic.key,
    iconPath: NotesIcon.italicIcon,
  ),
  ToolbarItem(
    attribute: Attribute.underline,
    key: Attribute.underline.key,
    iconPath: NotesIcon.textUnderlineIcon,
  ),
  ToolbarItem(
    attribute: Attribute.strikeThrough,
    key: Attribute.strikeThrough.key,
    iconPath: NotesIcon.strikeThroughIcon,
  ),
];
