import 'package:flutter/material.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:widgets/mechanix.dart';

DefaultStyles quillEditorStyle(BuildContext context) {
  return DefaultStyles(
    bold: TextStyle(fontWeight: FontWeight.w700),
    italic: TextStyle(fontStyle: FontStyle.italic),
    underline: TextStyle(decoration: TextDecoration.underline),
    paragraph: DefaultTextBlockStyle(
      TextStyle(
        fontSize: 20,
        fontWeight: FontWeight.w400,
        fontFamily: "Overused Grotesk",
        color: context.onSurface,
        height: 1.45, // Word-like line spacing
        letterSpacing: 0.0,
      ),
      HorizontalSpacing(0, 0), // Standard margins
      VerticalSpacing(0, 0), // Paragraph spacing
      VerticalSpacing(0, 0), // No additional line spacing here
      null,
    ),

    // Heading 1 - Word-like style
    h1: DefaultTextBlockStyle(
      TextStyle(
        fontSize: 26,
        fontWeight: FontWeight.w700,
        color: context.onSurface,
        height: 1.3,
        fontFamily: "Overused Grotesk",
        letterSpacing: 0,
      ),
      HorizontalSpacing(0, 0), // Standard margins
      VerticalSpacing(10, 10),
      VerticalSpacing(0, 0),
      null,
    ),

    // Heading 2 - Word-like style
    h2: DefaultTextBlockStyle(
      TextStyle(
        fontSize: 22,
        fontWeight: FontWeight.w700,
        color: context.onSurface,
        fontFamily: "Overused Grotesk",
        height: 1.25,
        letterSpacing: 0,
      ),
      HorizontalSpacing(0, 0), // Standard margins
      VerticalSpacing(5, 5),
      VerticalSpacing(0, 0),
      null,
    ),

    // Heading 3 - Word-like style
    h3: DefaultTextBlockStyle(
      TextStyle(
        fontSize: 20,
        fontWeight: FontWeight.w600,
        fontFamily: "Overused Grotesk",
        color: context.onSurface,
        height: 1.2,
        letterSpacing: 0.2,
      ),
      HorizontalSpacing(0, 0),
      VerticalSpacing(2, 2),
      VerticalSpacing(0, 0),
      null,
    ),

    // Lists - Word-like formatting
    placeHolder: DefaultTextBlockStyle(
      TextStyle(
        fontSize: 18,
        fontWeight: FontWeight.w400,
        fontFamily: "Overused Grotesk",
        color: context.primaryContainer,
        height: 1.2,
        letterSpacing: 0.2,
      ),
      HorizontalSpacing(0, 0),
      VerticalSpacing(14, 8),
      VerticalSpacing(0, 0),
      null,
    ),
    // Quote block - Word-like style
    quote: DefaultTextBlockStyle(
      TextStyle(
        fontSize: 18,
        color: context.onSurface, // Slightly dimmed for quotes
        height: 1.2,
        fontFamily: "Overused Grotesk",
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
    code: DefaultTextBlockStyle(
      TextStyle(
        fontSize: 18,
        fontWeight: FontWeight.w500,
        color: context.colorScheme.onTertiaryFixedVariant,
        height: 1.35,
        letterSpacing: -0.4,
        fontFamily: 'Geist Mono',
      ),
      HorizontalSpacing(0, 0),
      VerticalSpacing(16, 16),
      VerticalSpacing(0, 0),

      BoxDecoration(
        color: context.secondary,
        borderRadius: BorderRadius.all(Radius.circular(10)),
        // border: Border.all(color: Color(0xFF444444), width: 1),
      ),
    ),

    // Inline code
    // Add inside DefaultStyles
    inlineCode: InlineCodeStyle(
      backgroundColor: context.secondaryContainer, // dark background
      radius: Radius.circular(4), // rounded edges
      style: TextStyle(
        fontWeight: FontWeight.w400,
        fontSize: 18,
        color: context.onSurface, // light text
        height: 1.45,
        fontFamily: 'Geist Mono',
      ),
    ),

    // Text alignment styles
    // align:  DefaultTextBlockStyle(
    //   TextStyle(fontSize: 18, color: context.onSurface, height: 1.2),
    //   HorizontalSpacing(12, 12),
    //   VerticalSpacing(6, 6),
    //   VerticalSpacing(0, 0),
    //   null,
    // ),
    lists: DefaultListBlockStyle(
      TextStyle(
        color: context.onSurface,
        fontWeight: FontWeight.w400,
        fontFamily: "Overused Grotesk",
      ),
      HorizontalSpacing(0, 0), // Standard margins
      VerticalSpacing(2, 2),
      VerticalSpacing(2, 2),
      null,
      null,
    ),

    strikeThrough: TextStyle(
      decoration: TextDecoration.lineThrough,
      decorationColor: context.outline,
      fontFamily: "Overused Grotesk",
      color: context.outline,
    ),

    // Link style
    link: TextStyle(
      color: Color(0xFF4A9EFF),
      decoration: TextDecoration.underline,
      decorationColor: Color(0xFF4A9EFF),
      fontFamily: "Overused Grotesk",
    ),

    // Color and size variations
    color: context.onSurface, // Default text color
    // Indent styles for nested content
    // indent:  DefaultTextBlockStyle(
    //   TextStyle(fontSize: 16, color: context.onSurface, height: 1.2),
    //   HorizontalSpacing(36, 12), // Increased left margin for indent
    //   VerticalSpacing(6, 6),
    //   VerticalSpacing(0, 0),
    //   null,
    // ),
  );
}
