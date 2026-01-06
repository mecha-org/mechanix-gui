import 'dart:math';
import 'dart:ui';
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
  State<SegmentedBarEqualizer> createState() => _SegmentmentedBarEqualizerState();
}

class _SegmentmentedBarEqualizerState extends State<SegmentedBarEqualizer>
    with SingleTickerProviderStateMixin {
  static const int numberOfBars = 3;
  static const int segmentsPerBar = 4;

  late final AnimationController _controller;
  late final Random _rand;

  late List<double> _currentLevels;
  late List<double> _targetLevels;

  @override
  void initState() {
    super.initState();

    _rand = Random();

    _currentLevels = List.generate(numberOfBars, (_) => _rand.nextDouble());
    _targetLevels = List.generate(numberOfBars, (_) => _rand.nextDouble());

    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 280),
    )..addListener(() {
        setState(() {});
      });

    _controller.addStatusListener((status) {
      if (status == AnimationStatus.completed && widget.isPlaying) {
        _generateNewTargets();
        _controller.forward(from: 0);
      }
    });

    if (widget.isPlaying) {
      _controller.forward();
    }
  }

  void _generateNewTargets() {
    _currentLevels = List.from(_targetLevels);
    _targetLevels =
        List.generate(numberOfBars, (_) => _rand.nextDouble());
  }

  @override
  void didUpdateWidget(covariant SegmentedBarEqualizer oldWidget) {
    super.didUpdateWidget(oldWidget);

    if (oldWidget.isPlaying != widget.isPlaying) {
      if (widget.isPlaying) {
        _controller.forward();
      } else {
        _controller.stop(canceled: false);
      }
    }
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  double _interpolatedLevel(int index) {
    return lerpDouble(
          _currentLevels[index],
          _targetLevels[index],
          _controller.value,
        )!
        .clamp(0.0, 1.0);
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 24,
      height: 24,
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        crossAxisAlignment: CrossAxisAlignment.end,
        children: List.generate(
          numberOfBars,
          (i) => _buildBar(i),
        ),
      ),
    );
  }

  Widget _buildBar(int index) {
    final level = _interpolatedLevel(index);
    final activeSegments =
        (level * segmentsPerBar).round().clamp(0, segmentsPerBar);

    return Column(
      mainAxisAlignment: MainAxisAlignment.end,
      children: List.generate(segmentsPerBar, (i) {
        final isActive = i < activeSegments;
        return Container(
          margin: EdgeInsets.only(bottom: i < segmentsPerBar - 1 ? 2 : 0),
          width: 5,
          height: 2,
          decoration: BoxDecoration(
            color: widget.color.withValues(
              alpha: isActive ? 1.0 : 0.15,
            ),
            borderRadius: BorderRadius.circular(1),
          ),
        );
      }).reversed.toList(),
    );
  }
}
