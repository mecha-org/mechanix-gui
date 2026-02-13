import 'package:flutter/material.dart';

class CircleOverlayPainter extends CustomPainter {
  final ColorScheme colorScheme;
  final double circleSize;
  final Size screenSize;

  const CircleOverlayPainter({
    required this.circleSize,
    required this.screenSize,
    required this.colorScheme,
  });

  @override
  void paint(Canvas canvas, Size size) {
    final paint =
        Paint()
          ..color = colorScheme.surface.withValues(alpha: 0.5)
          ..style = PaintingStyle.fill;

    final screenPath =
        Path()
          ..addRect(Rect.fromLTWH(0, 0, screenSize.width, screenSize.height));

    final center = Offset(screenSize.width / 2, screenSize.height / 2);
    final radius = circleSize / 2;
    final circlePath =
        Path()..addOval(Rect.fromCircle(center: center, radius: radius));

    final overlayPath = Path.combine(
      PathOperation.difference,
      screenPath,
      circlePath,
    );

    canvas.drawPath(overlayPath, paint);
  }

  @override
  bool shouldRepaint(CircleOverlayPainter oldDelegate) {
    return oldDelegate.circleSize != circleSize ||
        oldDelegate.screenSize != screenSize;
  }
}
