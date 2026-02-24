import 'dart:math' as math;

import 'package:flutter/material.dart';

class ZoomStrips extends CustomPainter {
  final double zoomLevel;
  final Color stripsColor;
  final Color activeColor;

  const ZoomStrips({
    required this.zoomLevel,
    required this.stripsColor,
    required this.activeColor,
  });

  @override
  void paint(Canvas canvas, Size size) {
    final center = Offset(size.width / 2, size.height / 2);
    final radius = size.width / 2;

    final baseStripRadius = radius + 10.0;

    const totalArcLength = 152.0;
    final totalArcAngle = totalArcLength / baseStripRadius;

    const stripCount = 11;
    const spaceBetween = 6.0;

    const totalSpaceUsed = (stripCount - 1) * spaceBetween;
    const stripWidth = (totalArcLength - totalSpaceUsed) / stripCount;

    const bottomAngle = math.pi / 2;

    final zoomNormalized = (zoomLevel - 1.0) / (2.0 - 1.0);
    final currentZoomOffsetAngle =
        -(totalArcAngle / 2) + (zoomNormalized * totalArcAngle);

    final startAngle = bottomAngle + currentZoomOffsetAngle - totalArcAngle / 2;

    for (int i = 0; i < stripCount; i++) {
      final zoomValue = 2.0 - (i * 0.1);

      final stripPosition = i * (stripWidth + spaceBetween);
      final stripAngleStart = startAngle + (stripPosition / baseStripRadius);
      final stripAngleEnd = stripAngleStart + (1.5 / baseStripRadius);

      final isActive = (zoomLevel - zoomValue).abs() < 0.05;

      final isMajorMarker =
          (zoomValue == 1.0 || zoomValue == 1.5 || zoomValue == 2.0);

      final strokeWidth = isMajorMarker ? 27.0 : 18.0;

      final stripRadius =
          isMajorMarker ? baseStripRadius + 4.5 : baseStripRadius;

      final stripPaint =
          Paint()
            ..color = isActive ? activeColor : stripsColor
            ..style = PaintingStyle.stroke
            ..strokeWidth = strokeWidth
            ..strokeCap = StrokeCap.butt;

      canvas.drawArc(
        Rect.fromCircle(center: center, radius: stripRadius),
        stripAngleStart,
        stripAngleEnd - stripAngleStart,
        false,
        stripPaint,
      );
    }

    final zoomMarkers = [
      {'zoom': 2.0, 'label': '2x'},
      {'zoom': 1.5, 'label': '1.5x'},
      {'zoom': 1.0, 'label': '1x'},
    ];

    for (var marker in zoomMarkers) {
      final zoom = marker['zoom'] as double;
      final label = marker['label'] as String;

      final zoomNorm = (2.0 - zoom) / (2.0 - 1.0);
      final markerPosition = zoomNorm * totalArcLength;
      final markerAngle = startAngle + (markerPosition / baseStripRadius);

      final isNearActive = (zoomLevel - zoom).abs() < 0.05;

      final textPainter = TextPainter(
        text: TextSpan(
          text: label,
          style: TextStyle(
            color: isNearActive ? activeColor : stripsColor,
            fontSize: isNearActive ? 16 : 14,
            fontWeight: isNearActive ? FontWeight.bold : FontWeight.normal,
          ),
        ),
        textDirection: TextDirection.ltr,
      );
      textPainter.layout();

      final textDistance = baseStripRadius + 31.5;
      final textX = center.dx + textDistance * math.cos(markerAngle);
      final textY = center.dy + textDistance * math.sin(markerAngle);

      textPainter.paint(
        canvas,
        Offset(textX - textPainter.width / 2, textY - textPainter.height / 2),
      );
    }
  }

  @override
  bool shouldRepaint(ZoomStrips oldDelegate) {
    return oldDelegate.zoomLevel != zoomLevel;
  }
}
