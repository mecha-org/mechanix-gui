import 'package:flutter/material.dart';
import 'package:mechanix_music/src/commons/icons.dart';

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
            style: Theme.of(context).textTheme.bodyMedium,
          ),
        ],
      ),
    );
  }
}
