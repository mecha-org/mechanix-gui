import 'package:flutter/material.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';

class NotePreview extends StatelessWidget {
  final List<NoteLine> lines;
  final int maxLines;

  const NotePreview({super.key, required this.lines, this.maxLines = 4});

  @override
  Widget build(BuildContext context) {
    return _buildPreview(lines);
  }

  Widget _buildPreview(List<NoteLine> lines) {
    return Text.rich(
      TextSpan(children: _buildInline(lines)),
      maxLines: maxLines,
      overflow: TextOverflow.ellipsis,
      style: const TextStyle(
        fontSize: 14,
        color: NotesColors.labelColor,
        height: 1.5,
        letterSpacing: -0.1,
      ),
    );
  }

  List<InlineSpan> _buildInline(List<NoteLine> lines) {
    final List<InlineSpan> children = [];
    int numberedListCounter = 0;

    for (int i = 0; i < lines.length; i++) {
      final line = lines[i];

      switch (line.type) {
        case "text":
          numberedListCounter = 0;
          children.addAll(_buildStyledSpans(line.spans));
          break;

        case "number":
          numberedListCounter++;
          children.add(
            TextSpan(
              text: "$numberedListCounter. ",
              style: const TextStyle(
                fontWeight: FontWeight.w600,
                color: NotesColors.labelColor,
              ),
            ),
          );
          children.addAll(_buildStyledSpans(line.spans));
          break;

        case "bullet":
          numberedListCounter = 0;
          children.add(
            const TextSpan(
              text: "• ",
              style: TextStyle(
                fontWeight: FontWeight.bold,
                color: NotesColors.labelColor,

                fontSize: 16,
              ),
            ),
          );
          children.addAll(_buildStyledSpans(line.spans));
          break;

        case "code":
          numberedListCounter = 0;
          // Code block handles multiple lines internally
          children.addAll(_buildCodeBlock(line.spans));
          break;

        case "quote":
          numberedListCounter = 0;
          children.add(
            const TextSpan(
              text: "┃ ",
              style: TextStyle(
                fontWeight: FontWeight.bold,
                // color: Colors.grey.shade600,
                color: NotesColors.labelColor,

                fontSize: 16,
              ),
            ),
          );
          children.addAll(_buildQuoteSpans(line.spans));
          break;

        case "h1":
          numberedListCounter = 0;
          children.addAll(_buildHeadingSpans(line.spans, 18, FontWeight.bold));
          break;

        case "h2":
          numberedListCounter = 0;
          children.addAll(_buildHeadingSpans(line.spans, 16, FontWeight.w600));
          break;

        case "checkbox":
          numberedListCounter = 0;
          children.add(
            WidgetSpan(
              alignment: PlaceholderAlignment.middle,
              child: Padding(
                padding: const EdgeInsets.only(right: 6),
                child: IgnorePointer(
                  child: SizedBox(
                    width: 18,
                    height: 18,
                    child: Checkbox(
                      value: line.checked ?? false,
                      onChanged: null,
                      materialTapTargetSize: MaterialTapTargetSize.shrinkWrap,
                      visualDensity: VisualDensity.compact,
                      fillColor: WidgetStateProperty.all(
                        line.checked == true
                            ? NotesColors.labelColor
                            : Colors.transparent,
                      ),
                      side: BorderSide(color: Colors.grey.shade600, width: 1.5),
                    ),
                  ),
                ),
              ),
            ),
          );
          children.addAll(_buildStyledSpans(line.spans));
          break;

        default:
          numberedListCounter = 0;
          children.addAll(_buildStyledSpans(line.spans));
      }

      // Add newline between lines (except last and except for code blocks which handle their own newlines)
      if (i != lines.length - 1 && line.type != "code") {
        children.add(const TextSpan(text: "\n"));
      }
    }

    return children;
  }

  Iterable<InlineSpan> _buildStyledSpans(List<NoteSpan> spans) {
    return spans.map((span) {
      return TextSpan(
        text: span.text,
        style: TextStyle(
          fontWeight: span.bold ? FontWeight.bold : FontWeight.normal,
          fontStyle: span.italic ? FontStyle.italic : FontStyle.normal,
          decoration: TextDecoration.combine([
            if (span.underline) TextDecoration.underline,
            if (span.strike) TextDecoration.lineThrough,
          ]),
          color:
              span.color != null
                  ? Color(int.parse(span.color!))
                  : NotesColors.labelColor,
          backgroundColor:
              span.background != null
                  ? Color(int.parse(span.background!))
                  : null,
        ),
      );
    }).toList();
  }

  Iterable<InlineSpan> _buildCodeBlock(List<NoteSpan> spans) {
    return [
      WidgetSpan(
        child: Container(
          width: double.infinity,
          // margin: const EdgeInsets.symmetric(vertical: 6),
          padding: const EdgeInsets.all(8),
          decoration: BoxDecoration(
            color: NotesColors.floatingMenuColor,
            borderRadius: BorderRadius.circular(8),
          ),
          child: RichText(
            overflow: TextOverflow.ellipsis,
            text: TextSpan(
              children:
                  spans.map((span) {
                    return TextSpan(
                      text: span.text,
                      style: TextStyle(
                        fontFamily: "monospace",
                        fontSize: 13,
                        fontWeight:
                            span.bold ? FontWeight.bold : FontWeight.normal,
                        fontStyle:
                            span.italic ? FontStyle.italic : FontStyle.normal,
                        decoration: TextDecoration.combine([
                          if (span.underline) TextDecoration.underline,
                          if (span.strike) TextDecoration.lineThrough,
                        ]),
                        color:
                            span.color != null
                                ? Color(int.parse(span.color!))
                                : NotesColors.labelColor,
                        // Note: background color can be optionally enabled
                        // backgroundColor:
                        //     span.background != null
                        //         ? Color(int.parse(span.background!))
                        //         : null,
                      ),
                    );
                  }).toList(),
            ),
          ),
        ),
      ),
      // Add newline after code block
      // const TextSpan(text: "\n"),
    ];
  }

  Iterable<InlineSpan> _buildQuoteSpans(List<NoteSpan> spans) {
    return spans.map((span) {
      return TextSpan(
        text: span.text,
        style: TextStyle(
          fontStyle: FontStyle.italic,
          color:
              span.color != null
                  ? Color(int.parse(span.color!))
                  : NotesColors.labelColor,
          fontWeight: span.bold ? FontWeight.bold : FontWeight.normal,
          decoration: TextDecoration.combine([
            if (span.underline) TextDecoration.underline,
            if (span.strike) TextDecoration.lineThrough,
          ]),
          backgroundColor:
              span.background != null
                  ? Color(int.parse(span.background!))
                  : null,
        ),
      );
    }).toList();
  }

  Iterable<InlineSpan> _buildHeadingSpans(
    List<NoteSpan> spans,
    double fontSize,
    FontWeight weight,
  ) {
    return spans.map((span) {
      return TextSpan(
        text: span.text,
        style: TextStyle(
          fontSize: fontSize,
          fontWeight: span.bold ? FontWeight.bold : weight,
          fontStyle: span.italic ? FontStyle.italic : FontStyle.normal,
          decoration: TextDecoration.combine([
            if (span.underline) TextDecoration.underline,
            if (span.strike) TextDecoration.lineThrough,
          ]),
          color:
              span.color != null
                  ? Color(int.parse(span.color!))
                  : NotesColors.labelColor,
          backgroundColor:
              span.background != null
                  ? Color(int.parse(span.background!))
                  : null,
        ),
      );
    }).toList();
  }
}
