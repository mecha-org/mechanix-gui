import 'package:flutter/material.dart';

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
