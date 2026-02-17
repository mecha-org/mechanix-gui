import 'package:flutter/material.dart';
import 'package:mechanix_camera/src/home/widgets/camera_frame_scope.dart';
import 'package:widgets/extensions/color.dart';

class ReframeSize extends StatelessWidget {
  const ReframeSize({super.key});

  @override
  Widget build(BuildContext context) {
    final frameRect = CameraFrameScope.of(context);

    if (frameRect.left == 0 && frameRect.top == 0) {
      return const SizedBox.shrink();
    }

    return CustomPaint(
      painter: ReframePainter(
        frameRect: frameRect,
        overlayColor: context.surface.withValues(alpha: 0.6),
      ),
      child: const SizedBox.expand(),
    );
  }
}

class ReframePainter extends CustomPainter {
  final Rect frameRect;
  final Color overlayColor;

  ReframePainter({required this.frameRect, required this.overlayColor});

  @override
  void paint(Canvas canvas, Size size) {
    final overlayPaint =
        Paint()
          ..color = overlayColor
          ..style = PaintingStyle.fill;

    final path =
        Path()
          ..addRect(
            Rect.fromLTWH(0, 0, size.width, size.height),
          ) // ← use size here
          ..addRect(frameRect)
          ..fillType = PathFillType.evenOdd;

    canvas.drawPath(path, overlayPaint);
  }

  @override
  bool shouldRepaint(ReframePainter oldDelegate) {
    return oldDelegate.frameRect != frameRect ||
        oldDelegate.overlayColor != overlayColor;
  }
}
