import 'package:flutter/material.dart';
import 'package:widgets/mechanix.dart';

class CustomTrailingText extends StatelessWidget {
  const CustomTrailingText({super.key, required this.title, this.titleStyle});

  final String title;
  final TextStyle? titleStyle;

  @override
  Widget build(BuildContext context) {
    final baseStyle = context.textTheme.labelLarge ?? const TextStyle();

    return Text(
      title,
      style: baseStyle.merge(titleStyle),
    );
  }
}
