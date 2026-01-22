import 'dart:ui' as ui;

import 'package:flutter/material.dart';

class BatteryPainter extends CustomPainter {
  final double batteryPercentage;
  final bool isCharging;
  final List<Color> colors;
  final Color borderColor;
  final Color backgroundColor;
  final double height;
  final double tipHeight;
  final double tipWidth;
  final double padding;
  final double borderRadius;
  final ui.Image? chargingIcon;

  BatteryPainter({
    required this.batteryPercentage,
    required this.isCharging,
    required this.colors,
    required this.borderColor,
    required this.backgroundColor,
    this.height = 68,
    this.tipHeight = 32.0,
    this.tipWidth = 13.0,
    this.padding = 8,
    this.borderRadius = 8,
    this.chargingIcon,
  });

  @override
  void paint(Canvas canvas, Size size) {
    final paint = Paint();
    final mainRectWidth = size.width - tipWidth;
    final tipTop = (size.height - tipHeight) / 2;
    final tipBottom = tipTop + tipHeight;

    // Draw main battery body background WITH rounded corners on both sides
    final mainRect = RRect.fromRectAndCorners(
      Rect.fromLTWH(0, 0, mainRectWidth, size.height),
      topLeft: Radius.circular(borderRadius),
      topRight: Radius.circular(borderRadius), // Radius on right side too
      bottomLeft: Radius.circular(borderRadius),
      bottomRight: Radius.circular(borderRadius), // Radius on right side too
    );

    paint.color = backgroundColor;
    canvas.drawRRect(mainRect, paint);

    // Draw battery fill with gradient
    if (batteryPercentage > 0) {
      final fillWidth =
          (mainRectWidth - (padding * 2)) * (batteryPercentage / 100);

      // Create inner fill rect with rounded corners on both sides
      final fillRect = RRect.fromRectAndCorners(
        Rect.fromLTRB(
            padding, padding, padding + fillWidth, size.height - padding),
        topLeft: Radius.circular(borderRadius - padding / 2),
        bottomLeft: Radius.circular(borderRadius - padding / 2),
        topRight: Radius.circular(borderRadius - padding / 2),
        bottomRight: Radius.circular(borderRadius - padding / 2),
      );

      // Create gradient
      final gradient = LinearGradient(
        colors: colors,
      ).createShader(Rect.fromLTWH(0, 0, fillWidth, size.height));

      paint.shader = gradient;
      canvas.drawRRect(fillRect, paint);
      paint.shader = null;
    }

    // Draw battery tip background (NO rounded corners on left side of tip)
    final tipRect = RRect.fromRectAndCorners(
      Rect.fromLTWH(mainRectWidth, tipTop, tipWidth, tipHeight),
      topLeft: Radius.circular(0), // No radius on left side
      bottomLeft: Radius.circular(0), // No radius on left side
      topRight: const Radius.circular(2),
      bottomRight: const Radius.circular(2),
    );

    paint.color = backgroundColor;
    canvas.drawRRect(tipRect, paint);

    // Draw borders with custom paths to skip connecting part
    paint.color = borderColor;
    paint.style = PaintingStyle.stroke;
    paint.strokeWidth = 1;

    // Draw main body border - custom to skip connecting part
    _drawMainBodyBorder(canvas, size, mainRectWidth, tipTop, tipBottom);

    // Draw tip border - custom to skip connecting part
    _drawTipBorder(canvas, size, mainRectWidth, tipTop, tipBottom);

    // Draw charging icon if needed
    if (isCharging && chargingIcon != null) {
      _drawChargingIcon(canvas, size, mainRectWidth);
    }
  }

  void _drawMainBodyBorder(Canvas canvas, Size size, double mainRectWidth,
      double tipTop, double tipBottom) {
    final paint = Paint()
      ..color = borderColor
      ..style = PaintingStyle.stroke
      ..strokeWidth = 1;

    final path = Path();

    // Start at top-right corner (where the border will start after the tip connection)
    // We need to start drawing the border from where the tip ends
    if (tipTop > 0) {
      // Draw border from top-left to top-right (but stop before connecting with tip)
      // Start at top-left
      path.moveTo(0, borderRadius);

      // Top-left rounded corner
      path.quadraticBezierTo(0, 0, borderRadius, 0);

      // Top edge - stop before the tip connection
      path.lineTo(mainRectWidth - borderRadius, 0);

      // Top-right rounded corner
      path.quadraticBezierTo(mainRectWidth, 0, mainRectWidth, borderRadius);
    } else {
      // If no space above tip, draw full top border
      path.moveTo(0, borderRadius);
      path.quadraticBezierTo(0, 0, borderRadius, 0);
      path.lineTo(mainRectWidth - borderRadius, 0);
      path.quadraticBezierTo(mainRectWidth, 0, mainRectWidth, borderRadius);
    }

    // Draw vertical line on right side ABOVE the tip (if there's space)
    if (tipTop > borderRadius) {
      path.lineTo(mainRectWidth, tipTop);
    }

    // Draw vertical line on right side BELOW the tip (if there's space)
    if (tipBottom < size.height - borderRadius) {
      path.moveTo(mainRectWidth, tipBottom);
      path.lineTo(mainRectWidth, size.height - borderRadius);
    } else if (tipBottom < size.height) {
      // If tip extends into the rounded corner area
      path.moveTo(mainRectWidth, tipBottom);
      path.lineTo(mainRectWidth, size.height - borderRadius);
    }

    // Draw bottom-right rounded corner
    path.quadraticBezierTo(
        mainRectWidth, size.height, mainRectWidth - borderRadius, size.height);

    // Bottom edge
    path.lineTo(borderRadius, size.height);

    // Bottom-left rounded corner
    path.quadraticBezierTo(0, size.height, 0, size.height - borderRadius);

    // Left edge
    path.lineTo(0, borderRadius);

    canvas.drawPath(path, paint);
  }

  void _drawTipBorder(Canvas canvas, Size size, double mainRectWidth,
      double tipTop, double tipBottom) {
    final paint = Paint()
      ..color = borderColor
      ..style = PaintingStyle.stroke
      ..strokeWidth = 1;

    final path = Path();

    // Start at top-right of tip (skip the connecting part on left)
    path.moveTo(mainRectWidth + tipWidth, tipTop);

    // Top-right rounded corner
    path.quadraticBezierTo(
        mainRectWidth + tipWidth, tipTop, mainRectWidth + tipWidth, tipTop + 2);

    // Right vertical line
    path.lineTo(mainRectWidth + tipWidth, tipBottom - 2);

    // Bottom-right rounded corner
    path.quadraticBezierTo(mainRectWidth + tipWidth, tipBottom,
        mainRectWidth + tipWidth - 2, tipBottom);

    // Bottom edge of tip (skip the connecting part on left)
    // We stop before reaching mainRectWidth to leave a gap
    path.lineTo(mainRectWidth + 2, tipBottom);

    // Draw top edge of tip (skip the connecting part on left)
    final topPath = Path();
    topPath.moveTo(mainRectWidth + 2, tipTop);
    topPath.lineTo(mainRectWidth + tipWidth, tipTop);

    canvas.drawPath(path, paint);
    canvas.drawPath(topPath, paint);
  }

  void _drawChargingIcon(Canvas canvas, Size size, double mainRectWidth) {
    if (chargingIcon == null) return;

    final centerX = mainRectWidth / 2;
    final centerY = size.height / 2;
    final iconWidth = 24.0;
    final iconHeight = 28.0;

    canvas.drawImageRect(
      chargingIcon!,
      Rect.fromLTWH(
        0,
        0,
        chargingIcon!.width.toDouble(),
        chargingIcon!.height.toDouble(),
      ),
      Rect.fromCenter(
        center: Offset(centerX, centerY),
        width: iconWidth,
        height: iconHeight,
      ),
      Paint(),
    );
  }

  @override
  bool shouldRepaint(covariant BatteryPainter oldDelegate) {
    return oldDelegate.batteryPercentage != batteryPercentage ||
        oldDelegate.isCharging != isCharging ||
        oldDelegate.colors != colors ||
        oldDelegate.chargingIcon != chargingIcon;
  }
}
