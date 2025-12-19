import 'package:flutter/material.dart';
import 'package:mechanix_notes/src/features/home/presentation/grid_empty_layout.dart';
class LoadingNotes extends StatefulWidget {
  const LoadingNotes({super.key});

  @override
  State<LoadingNotes> createState() => _LoadingNotesState();
}

class _LoadingNotesState extends State<LoadingNotes>
    with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _animation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      duration: const Duration(milliseconds: 1500),
      vsync: this,
    )..repeat();

    _animation = Tween<double>(begin: -1.0, end: 2.0).animate(
      CurvedAnimation(parent: _controller, curve: Curves.easeInOut),
    );
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return NotesGridLayout(
      leftColumnHeights: const [170, 80, 150],
      rightColumnHeights: const [80, 170, 100],
      isLoading: true,
      shimmerAnimation: _animation,
    );
  }
}