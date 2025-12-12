import 'package:flutter/material.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';

final quillEditorStyle = DefaultStyles(
  bold: const TextStyle(fontWeight: FontWeight.w700),
  italic: const TextStyle(fontStyle: FontStyle.italic),
  underline: const TextStyle(decoration: TextDecoration.underline),
  paragraph: const DefaultTextBlockStyle(
    TextStyle(
      fontSize: 18,
      fontWeight: FontWeight.w400,
      color: NotesColors.titleTextColor,
      height: 1.45, // Word-like line spacing
      letterSpacing: 0.0,
    ),
    HorizontalSpacing(0, 0), // Standard margins
    VerticalSpacing(0, 0), // Paragraph spacing
    VerticalSpacing(0, 0), // No additional line spacing here
    null,
  ),

  // Heading 1 - Word-like style
  h1: const DefaultTextBlockStyle(
    TextStyle(
      fontSize: 24,
      fontWeight: FontWeight.w700,
      color: NotesColors.titleTextColor,
      height: 1.3,
      letterSpacing: 0,
    ),
    HorizontalSpacing(0, 0), // Standard margins
    VerticalSpacing(10, 10),
    VerticalSpacing(0, 0),
    null,
  ),

  // Heading 2 - Word-like style
  h2: const DefaultTextBlockStyle(
    TextStyle(
      fontSize: 20,
      fontWeight: FontWeight.w700,
      color: NotesColors.titleTextColor,
      height: 1.25,
      letterSpacing: 0,
    ),
    HorizontalSpacing(0, 0), // Standard margins
    VerticalSpacing(5, 5),
    VerticalSpacing(0, 0),
    null,
  ),

  // Heading 3 - Word-like style
  h3: const DefaultTextBlockStyle(
    TextStyle(
      fontSize: 18,
      fontWeight: FontWeight.w600,
      color: NotesColors.titleTextColor,
      height: 1.2,
      letterSpacing: 0.2,
    ),
    HorizontalSpacing(0, 0),
    VerticalSpacing(2, 2),
    VerticalSpacing(0, 0),
    null,
  ),

  // Lists - Word-like formatting
  placeHolder: const DefaultTextBlockStyle(
    TextStyle(
      fontSize: 16,
      fontWeight: FontWeight.w400,
      color: NotesColors.secondaryTextColor,
      height: 1.2,
      letterSpacing: 0.2,
    ),
    HorizontalSpacing(0, 0),
    VerticalSpacing(14, 8),
    VerticalSpacing(0, 0),
    null,
  ),
  // Quote block - Word-like style
  quote: const DefaultTextBlockStyle(
    TextStyle(
      fontSize: 16,
      color: NotesColors.titleTextColor, // Slightly dimmed for quotes
      height: 1.2,
      fontStyle: FontStyle.italic,
    ),
    HorizontalSpacing(0, 0), // Indented left margin
    VerticalSpacing(8, 8),
    VerticalSpacing(0, 0),
    BoxDecoration(
      border: Border(left: BorderSide(color: Color(0xFF666666), width: 3)),
    ),
  ),

  // Code block - Word-like style
  code: const DefaultTextBlockStyle(
    TextStyle(
      fontSize: 16,
      fontWeight: FontWeight.w500,
      color: NotesColors.codeBlockTextColor,
      height: 1.35,
      letterSpacing: -0.4,
      fontFamily: 'Geist Mono',
    ),
    HorizontalSpacing(0, 0),
    VerticalSpacing(16, 16),
    VerticalSpacing(0, 0),

    BoxDecoration(
      color: NotesColors.cardColor,
      borderRadius: BorderRadius.all(Radius.circular(10)),
      // border: Border.all(color: Color(0xFF444444), width: 1),
    ),
  ),

  // Inline code
  // Add inside DefaultStyles
  inlineCode: InlineCodeStyle(
    backgroundColor: NotesColors.floatingMenuColor, // dark background
    radius: const Radius.circular(4), // rounded edges
    style: const TextStyle(
      fontWeight: FontWeight.w400,
      fontSize: 16,
      color: NotesColors.titleTextColor, // light text
      height: 1.45,
      fontFamily: 'Geist Mono',
    ),
  ),

  // Text alignment styles
  // align: const DefaultTextBlockStyle(
  //   TextStyle(fontSize: 16, color: NotesColors.titleTextColor, height: 1.2),
  //   HorizontalSpacing(12, 12),
  //   VerticalSpacing(6, 6),
  //   VerticalSpacing(0, 0),
  //   null,
  // ),
  lists: const DefaultListBlockStyle(
    TextStyle(color: NotesColors.titleTextColor, fontWeight: FontWeight.w400),
    HorizontalSpacing(0, 0), // Standard margins
    VerticalSpacing(5, 5),
    VerticalSpacing(5, 5),
    null,
    null,
  ),

  strikeThrough: const TextStyle(
    decoration: TextDecoration.lineThrough,
    decorationColor: NotesColors.strikeThroughColor,
    color: NotesColors.strikeThroughColor,
  ),

  // Link style
  link: const TextStyle(
    color: Color(0xFF4A9EFF),
    decoration: TextDecoration.underline,
    decorationColor: Color(0xFF4A9EFF),
  ),

  // Color and size variations
  color: NotesColors.titleTextColor, // Default text color
  // Indent styles for nested content
  // indent: const DefaultTextBlockStyle(
  //   TextStyle(fontSize: 16, color: NotesColors.titleTextColor, height: 1.2),
  //   HorizontalSpacing(36, 12), // Increased left margin for indent
  //   VerticalSpacing(6, 6),
  //   VerticalSpacing(0, 0),
  //   null,
  // ),
);
