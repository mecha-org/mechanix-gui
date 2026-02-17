import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_camera/models/camera_models.dart';
import 'package:mechanix_camera/src/bloc/camera_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_state.dart';
import 'package:widgets/extensions/color.dart';

class GridLayout extends StatelessWidget {
  const GridLayout({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocSelector<CameraBloc, CameraState, CameraGrid>(
      selector: (state) => state.grid,
      builder: (context, grid) {
        // Don't show grid if divisions is 0 or grid is disabled
        if (grid.divisions == 0) {
          return const SizedBox.shrink();
        }

        return CustomPaint(
          painter: GridPainter(
            divisions: grid.divisions,
            gridColor: context.onSurface,
          ),
          child: Container(),
        );
      },
    );
  }
}

class GridPainter extends CustomPainter {
  final int divisions;
  final Color gridColor;
  final double strokeWidth;

  GridPainter({
    required this.divisions,
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

    // Calculate spacing between grid lines
    final horizontalSpacing = size.width / divisions;
    final verticalSpacing = size.height / divisions;

    // Draw vertical lines
    for (int i = 1; i < divisions; i++) {
      final x = horizontalSpacing * i;
      canvas.drawLine(Offset(x, 0), Offset(x, size.height), paint);
    }

    // Draw horizontal lines
    for (int i = 1; i < divisions; i++) {
      final y = verticalSpacing * i;
      canvas.drawLine(Offset(0, y), Offset(size.width, y), paint);
    }
  }

  @override
  bool shouldRepaint(GridPainter oldDelegate) {
    return oldDelegate.divisions != divisions ||
        oldDelegate.gridColor != gridColor ||
        oldDelegate.strokeWidth != strokeWidth;
  }
}
