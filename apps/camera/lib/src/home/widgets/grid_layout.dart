import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_bloc.dart';
import 'package:mechanix_camera/src/home/widgets/camera_frame_scope.dart';
import 'package:widgets/extensions/color.dart';

class GridLayout extends StatelessWidget {
  const GridLayout({super.key});

  @override
  Widget build(BuildContext context) {
    final grid = context.select((CameraBloc b) => b.state.grid);
    if (grid.divisions == 0) return const SizedBox.shrink();

    final frameRect = CameraFrameScope.of(context);

    return CustomPaint(
      painter: GridPainter(
        divisions: grid.divisions,
        gridColor: context.onSurface,
        frameRect: frameRect,
      ),
      child: const SizedBox.expand(),
    );
  }
}

class GridPainter extends CustomPainter {
  final int divisions;
  final Rect frameRect;
  final Color gridColor;
  final double strokeWidth;

  GridPainter({
    required this.divisions,
    required this.frameRect,
    required this.gridColor,
    this.strokeWidth = 0.5,
  });

  @override
  void paint(Canvas canvas, Size size) {
    final paint =
        Paint()
          ..color = gridColor
          ..strokeWidth = strokeWidth
          ..style = PaintingStyle.stroke;

    // Calculate spacing between grid lines within the frame
    final horizontalSpacing = frameRect.width / divisions;
    final verticalSpacing = frameRect.height / divisions;

    // Draw vertical lines
    for (int i = 1; i < divisions; i++) {
      final x = frameRect.left + (horizontalSpacing * i);
      canvas.drawLine(
        Offset(x, frameRect.top),
        Offset(x, frameRect.bottom),
        paint,
      );
    }

    // Draw horizontal lines
    for (int i = 1; i < divisions; i++) {
      final y = frameRect.top + (verticalSpacing * i);
      canvas.drawLine(
        Offset(frameRect.left, y),
        Offset(frameRect.right, y),
        paint,
      );
    }
  }

  @override
  bool shouldRepaint(GridPainter oldDelegate) {
    return oldDelegate.divisions != divisions ||
        oldDelegate.frameRect != frameRect ||
        oldDelegate.gridColor != gridColor ||
        oldDelegate.strokeWidth != strokeWidth;
  }
}
