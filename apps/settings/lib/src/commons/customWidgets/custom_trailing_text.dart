import 'package:flutter/material.dart';
import 'package:widgets/mechanix.dart';

class CustomTrailingText extends StatelessWidget {
  const CustomTrailingText({
    super.key,
    required this.title,
    this.titleStyle,
    this.textAlign = TextAlign.right,
  });

  final String title;
  final TextStyle? titleStyle;
  final TextAlign textAlign;

  @override
  Widget build(BuildContext context) {
    final baseStyle =
        context.textTheme.bodyMedium?.copyWith(color: context.outlineVariant) ??
            const TextStyle();

    return Text(
      title,
      textAlign: textAlign,
      style: baseStyle.merge(titleStyle),
    );
  }
}
