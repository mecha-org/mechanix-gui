import 'package:flutter/material.dart';
import 'package:mechanix_notes/src/features/home/presentation/grid_empty_layout.dart';

class EmptyNotes extends StatelessWidget {
  const EmptyNotes({super.key});

  @override
  Widget build(BuildContext context) {
    return const NotesGridLayout(
      leftColumnHeights: [174, 92],
      rightColumnHeights: [88, 174],
      interactiveCardIndex: 0,
    );
  }
}
