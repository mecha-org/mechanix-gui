import 'package:flutter/material.dart';
import 'package:mechanix_notes/src/features/home/presentation/grid_empty_layout.dart';

class EmptyNotes extends StatelessWidget {
  const EmptyNotes({super.key});

  @override
  Widget build(BuildContext context) {
    return const NotesGridLayout(
      leftColumnHeights: [320, 176],
      rightColumnHeights: [176, 320],
      interactiveCardIndex: 0,
    );
  }
}
