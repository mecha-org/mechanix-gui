import 'package:flutter/material.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:widgets/mechanix.dart';

class EmptyHomeScreen extends StatelessWidget {
  const EmptyHomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        children: [
          Image.asset(MusicIcons.emptyHomeScreenIcon, width: 355, height: 355),
          Text(
            "No music tracks available",
            style: TextStyle(
              fontSize: 18,
              height: 1.35,
              color: context.onSurfaceVariant,
            ),
          ),
        ],
      ),
    );
  }
}
