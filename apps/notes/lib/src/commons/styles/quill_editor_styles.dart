import 'package:flutter/material.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';

final quillEditorStyle = DefaultStyles(
  // leading: DefaultTextBlockStyle(
  //   TextStyle(
  //     fontSize: 16,
  //     color: NotesColors.editorTextColor,
  //     height: 1.2, // Word-like line spacing
  //     letterSpacing: 0.0,
  //   ),
  //   HorizontalSpacing(12, 12), // Standard margins
  //   VerticalSpacing(1.2, 1.2), // Paragraph spacing
  //   VerticalSpacing(0, 0), // No additional line spacing here
  //   null,
  // ),
  
  paragraph: const DefaultTextBlockStyle(
    TextStyle(
      fontSize: 16,
      color: NotesColors.editorTextColor,
      height: 1.2, // Word-like line spacing
      letterSpacing: 0.0,
    ),
    HorizontalSpacing(12, 12), // Standard margins
    VerticalSpacing(1.2, 1.2), // Paragraph spacing
    VerticalSpacing(0, 0), // No additional line spacing here
    null,
  ),

  // Heading 1 - Word-like style
  h1: const DefaultTextBlockStyle(
    TextStyle(
      fontSize: 24,
      fontWeight: FontWeight.bold,
      color: NotesColors.editorTextColor,
      height: 1.2,
      letterSpacing: 0.5,
    ),
    HorizontalSpacing(12, 12),
    VerticalSpacing(18, 12), // More space before, less after
    VerticalSpacing(0, 0),
    null,
  ),

  // Heading 2 - Word-like style
  h2: const DefaultTextBlockStyle(
    TextStyle(
      fontSize: 20,
      fontWeight: FontWeight.bold,
      color: NotesColors.editorTextColor,
      height: 1.2,
      letterSpacing: 0.3,
    ),
    HorizontalSpacing(12, 12),
    VerticalSpacing(16, 10),
    VerticalSpacing(0, 0),
    null,
  ),

  // Heading 3 - Word-like style
  h3: const DefaultTextBlockStyle(
    TextStyle(
      fontSize: 18,
      fontWeight: FontWeight.w600,
      color: NotesColors.editorTextColor,
      height: 1.2,
      letterSpacing: 0.2,
    ),
    HorizontalSpacing(12, 12),
    VerticalSpacing(14, 8),
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
    HorizontalSpacing(12, 12),
    VerticalSpacing(14, 8),
    VerticalSpacing(0, 0),
    null,
  ),
  // Quote block - Word-like style
  quote: const DefaultTextBlockStyle(
    TextStyle(
      fontSize: 16,
      color: NotesColors.editorTextColor, // Slightly dimmed for quotes
      height: 1.2,
      fontStyle: FontStyle.italic,
    ),
    HorizontalSpacing(24, 12), // Indented left margin
    VerticalSpacing(8, 8),
    VerticalSpacing(0, 0),
    BoxDecoration(
      border: Border(left: BorderSide(color: Color(0xFF666666), width: 3)),
    ),
  ),

  // Code block - Word-like style
  code: const DefaultTextBlockStyle(
    TextStyle(
      fontSize: 14,
      color: NotesColors.editorTextColor,
      height: 1.2,
      backgroundColor: Color(0xFF2A2A2A),
    ),
    HorizontalSpacing(16, 16),
    VerticalSpacing(8, 8),
    VerticalSpacing(0, 0),
    BoxDecoration(
      color: Color(0xFF2A2A2A),
      borderRadius: BorderRadius.all(Radius.circular(4)),
      // border: Border.all(color: Color(0xFF444444), width: 1),
    ),
  ),

  // Inline code
  // Add inside DefaultStyles
  inlineCode: InlineCodeStyle(
    backgroundColor: const Color(0xFF2A2A2A), // dark background
    radius: const Radius.circular(4), // rounded edges
    style: const TextStyle(
      fontSize: 14,
      color: NotesColors.editorTextColor, // light text
      height: 1.2,
    ),
    header1: const TextStyle(
      fontSize: 22,
      color: NotesColors.editorTextColor,
      fontWeight: FontWeight.w600,
    ),
    header2: const TextStyle(
      fontSize: 20,
      color: NotesColors.editorTextColor,
      fontWeight: FontWeight.w600,
    ),
    header3: const TextStyle(
      fontSize: 18,
      color: NotesColors.editorTextColor,
      fontWeight: FontWeight.w600,
    ),
    header4: const TextStyle(
      fontSize: 16,
      color: NotesColors.editorTextColor,
      fontWeight: FontWeight.w600,
    ),
    header5: const TextStyle(
      fontSize: 14,
      color: NotesColors.editorTextColor,
      fontWeight: FontWeight.w600,
    ),
    header6: const TextStyle(
      fontSize: 13,
      color: NotesColors.editorTextColor,
      fontWeight: FontWeight.w600,
    ),
  ),

  // Text alignment styles
  align: const DefaultTextBlockStyle(
    TextStyle(fontSize: 16, color: NotesColors.editorTextColor, height: 1.2),
    HorizontalSpacing(12, 12),
    VerticalSpacing(6, 6),
    VerticalSpacing(0, 0),
    null,
  ),

  // Inline text styles - Word-like formatting
  // bold: TextStyle(fontWeight: FontWeight.bold, color: NotesColors.editorTextColor),

  // italic: TextStyle(fontStyle: FontStyle.italic, color: NotesColors.editorTextColor),
  underline: const TextStyle(
    decoration: TextDecoration.underline,
    decorationColor: NotesColors.editorTextColor,
    color: NotesColors.editorTextColor,
  ),

  strikeThrough: const TextStyle(
    decoration: TextDecoration.lineThrough,
    decorationColor: NotesColors.editorTextColor,
    color: NotesColors.editorTextColor,
  ),

  // Link style
  link: const TextStyle(
    color: Color(0xFF4A9EFF),
    decoration: TextDecoration.underline,
    decorationColor: Color(0xFF4A9EFF),
  ),

  // Color and size variations
  color: NotesColors.editorTextColor, // Default text color
  // Indent styles for nested content
  indent: const DefaultTextBlockStyle(
    TextStyle(fontSize: 16, color: NotesColors.editorTextColor, height: 1.2),
    HorizontalSpacing(36, 12), // Increased left margin for indent
    VerticalSpacing(6, 6),
    VerticalSpacing(0, 0),
    null,
  ),
);
