import 'package:flutter/material.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:widgets/mechanix.dart';

class EmptyHomeScreen extends StatelessWidget {
  final bool isLoading;
  const EmptyHomeScreen({super.key, this.isLoading = false});

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        children: [
          Image.asset(MusicIcons.emptyHomeScreenIcon, width: 355, height: 355),
          Text(
            isLoading ? "Loading ..." : "No music tracks available",
            style: TextStyle(
              fontSize: 20,
              height: 1.35,
              color: context.onSurfaceVariant,
            ),
          ),
        ],
      ),
    );
  }
}
