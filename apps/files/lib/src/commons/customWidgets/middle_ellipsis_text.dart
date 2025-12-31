import 'package:flutter/material.dart';

class MiddleEllipsisText extends StatelessWidget {
  final String text;
  final TextStyle? style;
  final int maxLines;
  final TextAlign textAlign;

  const MiddleEllipsisText(
    this.text, {
    super.key,
    this.style,
    this.maxLines = 1,
    this.textAlign = TextAlign.start,
  });

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        final effectiveStyle = style ?? DefaultTextStyle.of(context).style;

        final availableWidth = constraints.maxWidth;

        final painter = TextPainter(
          textDirection: TextDirection.ltr,
          maxLines: maxLines,
        );

        // Fast path: fits as-is
        painter.text = TextSpan(text: text, style: effectiveStyle);
        painter.layout(maxWidth: availableWidth);

        if (!painter.didExceedMaxLines) {
          return Text(
            text,
            style: effectiveStyle,
            maxLines: maxLines,
            overflow: TextOverflow.clip,
            softWrap: false,
            textAlign: textAlign,
          );
        }

        // Split extension
        final dot = text.lastIndexOf('.');
        final hasExt = dot > 0 && dot < text.length - 1;

        final ext = hasExt ? text.substring(dot) : '';
        final base = hasExt ? text.substring(0, dot) : text;

        String left = '';
        String right = '';
        const ellipsis = '…';

        // Binary shrink for performance
        int low = 0;
        int high = base.length;

        while (low < high) {
          final mid = (low + high) ~/ 2;
          final leftCount = (mid * 0.6).floor();
          final rightCount = mid - leftCount;

          if (leftCount + rightCount >= base.length) break;

          final candidate = base.substring(0, leftCount) +
              ellipsis +
              base.substring(base.length - rightCount) +
              ext;

          painter.text = TextSpan(text: candidate, style: effectiveStyle);
          painter.layout(maxWidth: availableWidth);

          if (painter.didExceedMaxLines) {
            high = mid - 1;
          } else {
            left = base.substring(0, leftCount);
            right = base.substring(base.length - rightCount);
            low = mid + 1;
          }
        }

        final result =
            left.isEmpty && right.isEmpty ? text : '$left$ellipsis$right$ext';

        return Text(
          result,
          style: effectiveStyle,
          maxLines: maxLines,
          overflow: TextOverflow.clip,
          softWrap: false,
          textAlign: textAlign,
        );
      },
    );
  }
}

String middleEllipsisString(
  String text,
  double maxWidth,
  TextStyle style,
) {
  final painter = TextPainter(
    textDirection: TextDirection.ltr,
    maxLines: 1,
  );

  painter.text = TextSpan(text: text, style: style);
  painter.layout(maxWidth: maxWidth);

  if (!painter.didExceedMaxLines) return text;

  final dot = text.lastIndexOf('.');
  final hasExt = dot > 0 && dot < text.length - 1;

  final ext = hasExt ? text.substring(dot) : '';
  final base = hasExt ? text.substring(0, dot) : text;

  const ellipsis = '…';

  int low = 0;
  int high = base.length;
  String finalText = text;

  while (low <= high) {
    final mid = (low + high) ~/ 2;
    final left = base.substring(0, (mid * 0.6).floor());
    final right = base.substring(base.length - (mid - left.length));

    final candidate = '$left$ellipsis$right$ext';

    painter.text = TextSpan(text: candidate, style: style);
    painter.layout(maxWidth: maxWidth);

    if (painter.didExceedMaxLines) {
      high = mid - 1;
    } else {
      finalText = candidate;
      low = mid + 1;
    }
  }

  return finalText;
}
