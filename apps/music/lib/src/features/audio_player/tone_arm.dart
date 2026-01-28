import 'dart:math' as math;

import 'package:flutter/material.dart';
import 'package:widgets/extensions/color.dart';

class ToneArm extends StatefulWidget {
  final bool isPlaying;

  const ToneArm({super.key, required this.isPlaying});

  @override
  State<ToneArm> createState() => _ToneArmState();
}

class _ToneArmState extends State<ToneArm> {
  Offset position = const Offset(-25, -10);

  @override
  Widget build(BuildContext context) {
    return Positioned(
      right: -80,
      top: -10,
      child: GestureDetector(
        child: SizedBox(
          width: 150,
          height: 150,
          child: Stack(
            clipBehavior: Clip.none,
            children: [
              // Tone arm - rotates around the circle (hinge)
              Positioned(
                right: 28.5, // Center of the 57px circle
                top: 28.5, // Center of the 57px circle
                child: AnimatedRotation(
                  duration: const Duration(milliseconds: 400),
                  curve: Curves.easeOutCubic,
                  alignment: Alignment.topCenter, // Pivot at the hinge
                  turns: widget.isPlaying ? 0.05 : -0.08,
                  child: CustomPaint(
                    size: const Size(90, 100),
                    painter: ToneArmPainter(color: context.onSurface),
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class ToneArmPainter extends CustomPainter {
  final Color color;
  const ToneArmPainter({required this.color});
  @override
  void paint(Canvas canvas, Size size) {
    final paint =
        Paint()
          ..color = color
          ..style = PaintingStyle.stroke
          ..strokeWidth = 6
          ..strokeCap = StrokeCap.round
          ..strokeJoin = StrokeJoin.round;

    final fillPaint =
        Paint()
          ..color = color
          ..style = PaintingStyle.fill;

    final centerX = size.width / 2;

    final armPath = Path();

    armPath.moveTo(centerX, 0);

    armPath.lineTo(centerX, size.height * 0.8);

    armPath.lineTo(centerX - 20, size.height - 8);

    canvas.drawPath(armPath, paint);
    final ringPaint =
        Paint()
          ..color = color
          ..style = PaintingStyle.stroke
          ..strokeWidth = 5;

    canvas.drawCircle(Offset(centerX, 0), 21, ringPaint);

    canvas.drawCircle(Offset(centerX, 0), 8, fillPaint);
    const rectWidth = 18.0;
    const rectHeight = 8.0;

    final attachX = centerX - 20;
    final attachY = size.height - 8;

    final pivot = Offset(attachX, attachY);

    canvas.save();

    canvas.translate(pivot.dx, pivot.dy);

    canvas.rotate(150 * math.pi / 180);

    final rrect = RRect.fromRectAndRadius(
      Rect.fromLTWH(0, -rectHeight / 2, rectWidth, rectHeight),
      const Radius.circular(2),
    );

    canvas.drawRRect(rrect, fillPaint);

    canvas.restore();
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) => false;
}
