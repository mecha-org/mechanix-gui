import 'dart:math' as math;

import 'package:flutter/material.dart';

class CircularFramePainter extends CustomPainter {
  final ColorScheme colorScheme;
  const CircularFramePainter({required this.colorScheme});

  @override
  void paint(Canvas canvas, Size size) {
    final paint =
        Paint()
          ..color = colorScheme.onSurface
          ..style = PaintingStyle.stroke
          ..strokeWidth = 0.5;

    final center = Offset(size.width / 2, size.height / 2);
    final radius = size.width / 2;

    canvas.drawCircle(center, radius, paint);

    final indicatorPaint =
        Paint()
          ..color = colorScheme.onSurface
          ..style = PaintingStyle.stroke
          ..strokeWidth = 5.0
          ..strokeCap = StrokeCap.round;

    final arcLength = 63.0;
    final indicatorAngle = arcLength / (2 * radius);

    canvas.drawArc(
      Rect.fromCircle(center: center, radius: radius),
      -math.pi / 2 - indicatorAngle,
      indicatorAngle * 2,
      false,
      indicatorPaint,
    );
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) {
    return false;
  }
}
