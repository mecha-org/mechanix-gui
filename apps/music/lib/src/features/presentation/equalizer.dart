import 'dart:math';
import 'package:flutter/material.dart';

class SegmentedBarEqualizer extends StatefulWidget {
  final Color color;
  final bool isPlaying;

  const SegmentedBarEqualizer({
    super.key,
    required this.color,
    required this.isPlaying,
  });

  @override
  State<SegmentedBarEqualizer> createState() => _SegmentedBarEqualizerState();
}

class _SegmentedBarEqualizerState extends State<SegmentedBarEqualizer>
    with SingleTickerProviderStateMixin {
  late final AnimationController _controller;
  late final List<double> _barPhases;

  static const int numberOfBars = 3;
  static const int segmentsPerBar = 4;

  @override
  void initState() {
    super.initState();

    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 900),
    );

    final rand = Random();
    _barPhases = List.generate(numberOfBars, (_) => rand.nextDouble());

    if (widget.isPlaying) {
      _controller.repeat(reverse: true);
    }
  }

  @override
  void didUpdateWidget(covariant SegmentedBarEqualizer oldWidget) {
    super.didUpdateWidget(oldWidget);

    if (oldWidget.isPlaying != widget.isPlaying) {
      if (widget.isPlaying) {
        // Resume from current value
        _controller.repeat(reverse: true);
      } else {
        // Freeze at current frame
        _controller.stop(canceled: false);
      }
    }
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 24,
      height: 24,
      child: AnimatedBuilder(
        animation: _controller,
        builder: (_, __) {
          return Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            crossAxisAlignment: CrossAxisAlignment.end,
            children: List.generate(
              numberOfBars,
              (barIndex) => _buildBar(barIndex),
            ),
          );
        },
      ),
    );
  }

  Widget _buildBar(int barIndex) {
    final t = (_controller.value + _barPhases[barIndex]) % 1.0;
    final normalized = (sin(t * pi) + 1) / 2;

    final activeSegments = (normalized * segmentsPerBar).round().clamp(
      0,
      segmentsPerBar,
    );

    return Column(
      mainAxisAlignment: MainAxisAlignment.end,
      children:
          List.generate(segmentsPerBar, (i) {
            final isActive = i < activeSegments;
            return Container(
              margin: EdgeInsets.only(bottom: i < segmentsPerBar - 1 ? 2 : 0),
              width: 5,
              height: 2,
              decoration: BoxDecoration(
                color: widget.color.withValues(alpha: isActive ? 1.0 : 0.15),
                borderRadius: BorderRadius.circular(1),
              ),
            );
          }).reversed.toList(),
    );
  }
}
