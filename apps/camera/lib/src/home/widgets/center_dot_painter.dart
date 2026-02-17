import 'package:flutter/material.dart';

class CenterDotPainter extends CustomPainter {
  final ColorScheme colorScheme;

  CenterDotPainter({required this.colorScheme});

  @override
  void paint(Canvas canvas, Size size) {
    final center = Offset(size.width / 2, size.height / 2);

    final dotPaint =
        Paint()
          ..color = colorScheme.onSecondaryFixedVariant
          ..style = PaintingStyle.fill;
    final smallDotPaint =
        Paint()
          ..color = colorScheme.onSurface
          ..style = PaintingStyle.fill;

    canvas.drawCircle(center, 2.0, dotPaint);
    canvas.drawCircle(center, 1.0, smallDotPaint);
  }

  @override
  bool shouldRepaint(CenterDotPainter oldDelegate) {
    return oldDelegate.colorScheme != colorScheme;
  }
}
