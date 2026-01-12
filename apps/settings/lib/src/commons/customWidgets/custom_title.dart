import 'package:flutter/material.dart';
import 'package:widgets/extension.dart';

class CustomTitle extends StatelessWidget {
  const CustomTitle({
    super.key,
    required this.title,
    this.fontSize = 24.0,
    this.textStyle,
  });

  final String title;
  final double fontSize;
  final TextStyle? textStyle;

  @override
  Widget build(BuildContext context) {
    return Align(
      alignment: Alignment.centerLeft,
      child: Text(
        title,
        style: textStyle ??
            TextStyle(
              color: context.primary,
              fontWeight: FontWeight.w600,
              fontSize: fontSize,
            ),
      ).padOnly(top: 6, right: 16, bottom: 12),
    );
  }
}
