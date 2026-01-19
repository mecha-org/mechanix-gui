import 'package:flutter/material.dart';
import 'package:mechanix_notes/src/features/home/models/notes_model.dart';
import 'package:widgets/extensions/color.dart';

class NotePreview extends StatelessWidget {
  final List<NoteLine> lines;
  final int maxLines;

  const NotePreview({super.key, required this.lines, this.maxLines = 4});

  @override
  Widget build(BuildContext context) {
    return _buildPreview(lines, context);
  }

  Widget _buildPreview(List<NoteLine> lines, BuildContext context) {
    return Text.rich(
      TextSpan(children: _buildInline(lines, context)),
      maxLines: maxLines,
      overflow: TextOverflow.ellipsis,
      style: TextStyle(fontSize: 16, color: context.onSecondaryFixed),
    );
  }

  List<InlineSpan> _buildInline(List<NoteLine> lines, BuildContext context) {
    final List<InlineSpan> children = [];
    int numberedListCounter = 0;

    for (int i = 0; i < lines.length; i++) {
      final line = lines[i];

      switch (line.type) {
        case "text":
          numberedListCounter = 0;
          children.addAll(_buildStyledSpans(line.spans, context));
          break;

        case "number":
          numberedListCounter++;
          children.add(
            TextSpan(
              text: "$numberedListCounter. ",
              style: TextStyle(
                fontWeight: FontWeight.w600,
                fontSize: 16,
                color: context.onSecondaryFixed,
              ),
            ),
          );
          children.addAll(_buildStyledSpans(line.spans, context));
          break;

        case "bullet":
          numberedListCounter = 0;
          children.add(
            TextSpan(
              text: "• ",
              style: TextStyle(
                fontWeight: FontWeight.bold,
                color: context.onSecondaryFixed,
                fontSize: 16,
              ),
            ),
          );
          children.addAll(_buildStyledSpans(line.spans, context));
          break;

        case "code":
          numberedListCounter = 0;
          // Code block handles multiple lines internally
          children.addAll(_buildCodeBlock(line.spans, context));
          break;

        case "quote":
          numberedListCounter = 0;
          children.add(
            TextSpan(
              text: "┃ ",
              style: TextStyle(
                fontWeight: FontWeight.bold,
                color: context.onSecondaryFixed,

                fontSize: 16,
              ),
            ),
          );
          children.addAll(_buildQuoteSpans(line.spans, context));
          break;

        case "h1":
          numberedListCounter = 0;
          children.addAll(
            _buildHeadingSpans(line.spans, 18, FontWeight.bold, context),
          );
          break;

        case "h2":
          numberedListCounter = 0;
          children.addAll(
            _buildHeadingSpans(line.spans, 16, FontWeight.w600, context),
          );
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
                            ? context.onSecondaryFixed
                            : Colors.transparent,
                      ),
                      side: BorderSide(color: context.onSurface, width: 1.5),
                    ),
                  ),
                ),
              ),
            ),
          );
          children.addAll(_buildStyledSpans(line.spans, context));
          break;

        default:
          numberedListCounter = 0;
          children.addAll(_buildStyledSpans(line.spans, context));
      }

      // Add newline between lines (except last and except for code blocks which handle their own newlines)
      if (i != lines.length - 1 && line.type != "code") {
        children.add(const TextSpan(text: "\n"));
      }
    }

    return children;
  }

  Iterable<InlineSpan> _buildStyledSpans(
    List<NoteSpan> spans,
    BuildContext context,
  ) {
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
                  : context.onSecondaryFixed,
          backgroundColor:
              span.background != null
                  ? context.primaryContainer.withValues(alpha: 0.8)
                  : null,
        ),
      );
    }).toList();
  }

  Iterable<InlineSpan> _buildCodeBlock(
    List<NoteSpan> spans,
    BuildContext context,
  ) {
    return [
      WidgetSpan(
        child: Container(
          width: double.infinity,
          // margin: const EdgeInsets.symmetric(vertical: 6),
          padding: const EdgeInsets.all(8),
          decoration: BoxDecoration(
            color: context.secondaryContainer,
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
                                : context.onSecondaryFixed,
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

  Iterable<InlineSpan> _buildQuoteSpans(
    List<NoteSpan> spans,
    BuildContext context,
  ) {
    return spans.map((span) {
      return TextSpan(
        text: span.text,
        style: TextStyle(
          fontStyle: FontStyle.italic,
          color:
              span.color != null
                  ? Color(int.parse(span.color!))
                  : context.onSecondaryFixed,
          fontWeight: span.bold ? FontWeight.bold : FontWeight.normal,
          decoration: TextDecoration.combine([
            if (span.underline) TextDecoration.underline,
            if (span.strike) TextDecoration.lineThrough,
          ]),
          backgroundColor:
              span.background != null
                  ? context.primaryContainer.withValues(alpha: 0.8)
                  : null,
        ),
      );
    }).toList();
  }

  Iterable<InlineSpan> _buildHeadingSpans(
    List<NoteSpan> spans,
    double fontSize,
    FontWeight weight,
    BuildContext context,
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
                  : context.onSecondaryFixed,
          backgroundColor:
              span.background != null
                  ? context.primaryContainer.withValues(alpha: 0.8)
                  : null,
        ),
      );
    }).toList();
  }
}
