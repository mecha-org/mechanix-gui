import 'dart:math' as math;

import 'package:flutter/material.dart';
import 'dart:math' as math;
import 'package:flutter/material.dart';

class SemiCircleSlider extends StatefulWidget {
  final double progress; // 0.0 - 1.0
  final ValueChanged<double> onChanged;

  const SemiCircleSlider({
    super.key,
    required this.progress,
    required this.onChanged,
  });

  @override
  _SemiCircleSliderState createState() => _SemiCircleSliderState();
}

class _SemiCircleSliderState extends State<SemiCircleSlider> {
  late double _progress;

  @override
  void initState() {
    super.initState();
    _progress = widget.progress;
  }

  @override
  void didUpdateWidget(covariant SemiCircleSlider oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.progress != widget.progress) {
      _progress = widget.progress;
    }
  }

  void _updateProgress(Offset localPos, Size size) {
    final center = Offset(size.width / 2, size.height);
    final dx = localPos.dx - center.dx;
    final dy = localPos.dy - center.dy;

    // atan2 returns angle relative to center (radians)
    double angle = math.atan2(dy, dx);

    // Limit to bottom semi circle (0 to π)
    if (angle < 0) angle = 0;
    if (angle > math.pi) angle = math.pi;

    final newProgress = angle / math.pi;
    setState(() => _progress = newProgress);
    widget.onChanged(newProgress);
  }

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onPanUpdate: (details) {
        final box = context.findRenderObject() as RenderBox;
        _updateProgress(box.globalToLocal(details.globalPosition), box.size);
      },
      onPanStart: (details) {
        final box = context.findRenderObject() as RenderBox;
        _updateProgress(box.globalToLocal(details.globalPosition), box.size);
      },
      child: CustomPaint(
        size: const Size(350, 175),
        painter: _SemiCirclePainter(progress: _progress),
      ),
    );
  }
}

class _SemiCirclePainter extends CustomPainter {
  final double progress;

  _SemiCirclePainter({required this.progress});

  @override
  void paint(Canvas canvas, Size size) {
    final Paint inactivePaint =
        Paint()
          ..color = Colors.grey.shade400
          ..strokeWidth = 6
          ..style = PaintingStyle.stroke;

    final Paint activePaint =
        Paint()
          ..color = Colors.white
          ..strokeWidth = 6
          ..style = PaintingStyle.stroke;

    final Rect rect = Rect.fromLTWH(
      0,
      -size.height,
      size.width,
      size.height * 2,
    );

    // Draw inactive arc
    canvas.drawArc(rect, 0, math.pi, false, inactivePaint);

    // Draw active arc
    canvas.drawArc(rect, 0, math.pi * progress, false, activePaint);

    // Draw thumb (20x20 circle)
    final angle = progress * math.pi;
    final center = Offset(size.width / 2, size.height);
    final radius = size.width / 2;

    final thumbX = center.dx + radius * math.cos(angle);
    final thumbY = center.dy + radius * math.sin(angle);

    final Paint thumbPaint = Paint()..color = Colors.white;
    canvas.drawCircle(Offset(thumbX, thumbY), 10, thumbPaint);
  }

  @override
  bool shouldRepaint(_SemiCirclePainter oldDelegate) =>
      oldDelegate.progress != progress;
}
