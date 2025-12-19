import 'package:flutter/material.dart';
import 'package:mechanix_music/src/commons/colors.dart';

class TitleWidget extends StatelessWidget {
  final String title;
  const TitleWidget({super.key, required this.title});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: EdgeInsets.only(top: 0, bottom: 12),
      child: Text(
        title,
        style: TextStyle(
          color: MusicColors.titleColor,
          fontSize: 24,
          height: 1.25,
          letterSpacing: -1.1,
          fontWeight: FontWeight.w600,
        ),
      ),
    );
  }
}
