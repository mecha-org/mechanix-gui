import 'package:flutter/material.dart';
import 'package:widgets/mechanix.dart';

class TitleWidget extends StatelessWidget {
  final String title;
  final TextStyle? textStyle;
  const TitleWidget({super.key, required this.title, this.textStyle});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: EdgeInsets.only(top: 0, bottom: 12),
      child: Text(
        title,
        style: TextStyle(
          color: context.primary,
          fontSize: 24,
          height: 1.25,
          letterSpacing: -1.1,
          fontWeight: FontWeight.w600,
        ).merge(textStyle),
      ),
    );
  }
}
