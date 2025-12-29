import 'package:flutter/material.dart';
import 'dart:math' as math;

import 'package:mechanix_music/src/commons/colors.dart';

class SemiCircularAudioProgress extends StatefulWidget {
  final Duration currentDuration;
  final Duration totalDuration;
  final ValueChanged<Duration> onPositionChange;
  final VoidCallback onDragStart;
  final ValueChanged<double> onDragUpdate;
  final VoidCallback onDragEnd;

  const SemiCircularAudioProgress({
    super.key,
    required this.currentDuration,
    required this.totalDuration,
    required this.onPositionChange,
    required this.onDragStart,
    required this.onDragUpdate,
    required this.onDragEnd,
  });

  @override
  State<SemiCircularAudioProgress> createState() =>
      _SemiCircularAudioProgressState();
}

class _SemiCircularAudioProgressState extends State<SemiCircularAudioProgress> {
  bool _isDragging = false;
  double _previousAngle = 0.0;
  DateTime? _lastDragTime;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 360,
      height: 180,
      child: LayoutBuilder(
        builder: (context, constraints) {
          final size = Size(constraints.maxWidth, constraints.maxHeight);

          return MouseRegion(
            cursor: SystemMouseCursors.click,
            hitTestBehavior: HitTestBehavior.translucent,
            child: GestureDetector(
              behavior: HitTestBehavior.translucent,

              // 👉 Tap to seek
              onTapDown: (d) {
                final angle = _calculateAngle(d.localPosition, size);
                if (angle != null) {
                  _updateDragAngle(d.localPosition, size, isTap: true);
                  // Trigger the callbacks for tap
                  widget.onDragStart();
                  widget.onDragEnd();
                }
              },

              // 👉 Drag to seek with velocity tracking
              onPanStart: (d) {
                final angle = _calculateAngle(d.localPosition, size);
                if (angle != null) {
                  setState(() => _isDragging = true);
                  _lastDragTime = DateTime.now();
                  widget.onDragStart();
                  _previousAngle = angle;
                }
              },
              onPanUpdate: (d) {
                if (_isDragging) {
                  _updateDragAngle(d.localPosition, size, isTap: false);
                }
              },
              onPanEnd: (_) {
                if (_isDragging) {
                  setState(() => _isDragging = false);
                  _lastDragTime = null;
                  widget.onDragEnd();
                }
              },

              child: Stack(
                children: [
                  // Invisible hit area overlay
                  CustomPaint(
                    size: size,
                    painter: HitAreaPainter(),
                  ),
                  // Actual progress visualization
                  CustomPaint(
                    size: size,
                    painter: SemiCircularProgressPainter(
                      currentDuration: widget.currentDuration,
                      totalDuration: widget.totalDuration,
                      isDragging: _isDragging,
                      showDebugHitArea: false, // Enable debug visualization
                    ),
                  ),
                ],
              ),
            ),
          );
        },
      ),
    );
  }

  double? _calculateAngle(Offset localPosition, Size size) {
    final center = Offset(size.width / 2, 0);
    final radius = size.width / 2;
    const strokeWidth = 6.0;

    final dx = localPosition.dx - center.dx;
    final dy = localPosition.dy - center.dy;

    final distance = math.sqrt(dx * dx + dy * dy);
    final trackRadius = radius - strokeWidth / 2;

    // Calculate angle first
    double angle = math.atan2(dy, dx);

    // Normalize to 0 → 2π
    if (angle < 0) angle += 2 * math.pi;

    // 🚫 Reject bottom half — ONLY top semi-circle allowed
    if (angle > math.pi) return null;

    // 🎯 EXPANDED hit test → Larger drag area for easier interaction
    // Allow a 60px band (increased from 50px for better edge detection)
    const hitAreaWidth = 60.0;
    
    // Check if within the hit area band
    if (distance < trackRadius - hitAreaWidth || 
        distance > trackRadius + hitAreaWidth) {
      return null;
    }

    return angle;
  }

  void _updateDragAngle(
    Offset localPosition,
    Size size, {
    required bool isTap,
  }) {
    final angle = _calculateAngle(localPosition, size);
    if (angle == null) return;

    // Calculate angular velocity for disc rotation
    if (!isTap && _isDragging) {
      final now = DateTime.now();
      final timeDelta =
          _lastDragTime != null
              ? now.difference(_lastDragTime!).inMilliseconds
              : 16;

      if (timeDelta > 0) {
        // Calculate angular difference
        double angleDelta = angle - _previousAngle;

        // Handle wrap-around at π/0 boundary
        if (angleDelta > math.pi) {
          angleDelta -= 2 * math.pi;
        } else if (angleDelta < -math.pi) {
          angleDelta += 2 * math.pi;
        }

        // Calculate angular velocity (radians per millisecond, scaled up)
        final angularVelocity = (angleDelta / timeDelta) * 1000;

        // Send velocity to parent for disc rotation
        widget.onDragUpdate(angularVelocity);

        _previousAngle = angle;
        _lastDragTime = now;
      }
    } else if (isTap) {
      _previousAngle = angle;
    }

    // 🎧 Map angle to progress
    // π → 0.0
    // 0 → 1.0
    final progress = (1 - angle / math.pi).clamp(0.0, 1.0);

    widget.onPositionChange(widget.totalDuration * progress);
  }
}

class SemiCircularProgressPainter extends CustomPainter {
  final Duration currentDuration;
  final Duration totalDuration;
  final bool isDragging;
  final bool showDebugHitArea;

  SemiCircularProgressPainter({
    required this.currentDuration,
    required this.totalDuration,
    required this.isDragging,
    this.showDebugHitArea = false,
  });

  @override
  void paint(Canvas canvas, Size size) {
    final center = Offset(size.width / 2, 0);
    final radius = size.width / 2;
    final strokeWidth = 6.0;

    // 🔴 DEBUG: Draw hit area in red (60px band around the track)
    if (showDebugHitArea) {
      final debugPaint = Paint()
        ..color = Colors.red.withOpacity(0.3)
        ..style = PaintingStyle.stroke
        ..strokeWidth = 120; // 60px on each side = 120px total width

      canvas.drawArc(
        Rect.fromCircle(center: center, radius: radius - strokeWidth / 2),
        math.pi,
        -math.pi,
        false,
        debugPaint,
      );
    }

    // Background track (inactive part)
    final backgroundPaint =
        Paint()
          ..color = MusicColors.dividerColor
          ..style = PaintingStyle.stroke
          ..strokeWidth = strokeWidth
          ..strokeCap = StrokeCap.round;

    canvas.drawArc(
      Rect.fromCircle(center: center, radius: radius - strokeWidth / 2),
      math.pi,
      -math.pi,
      false,
      backgroundPaint,
    );

    // Calculate progress
    final progress =
        totalDuration.inMilliseconds > 0
            ? currentDuration.inMilliseconds / totalDuration.inMilliseconds
            : 0.0;
    final sweepAngle = -math.pi * progress.clamp(0.0, 1.0);

    // Active track (progress part)
    final progressPaint =
        Paint()
          ..color = MusicColors.primaryTextColor
          ..style = PaintingStyle.stroke
          ..strokeWidth = strokeWidth
          ..strokeCap = StrokeCap.round;

    canvas.drawArc(
      Rect.fromCircle(center: center, radius: radius - strokeWidth / 2),
      math.pi,
      sweepAngle,
      false,
      progressPaint,
    );

    // Draw thumb indicator at current position
    final thumbAngle = math.pi + sweepAngle;
    final thumbX =
        center.dx + (radius - strokeWidth / 2) * math.cos(thumbAngle);
    final thumbY =
        center.dy + (radius - strokeWidth / 2) * math.sin(thumbAngle);

    final thumbPaint =
        Paint()
          ..color = MusicColors.primaryTextColor
          ..style = PaintingStyle.fill;

    // Draw larger thumb when dragging
    final thumbRadius = isDragging ? 10.0 : 8.0;
    canvas.drawCircle(Offset(thumbX, thumbY), thumbRadius, thumbPaint);

    // Draw time labels
    final textStyle = TextStyle(
      color: MusicColors.primaryTextColor,
      fontSize: 12,
      fontWeight: FontWeight.w500,
    );

    // Calculate positions at the ends of the arc (left and right)
    final leftAngle = math.pi; // Left side of semi-circle
    final rightAngle = 0.0; // Right side of semi-circle

    final leftX = center.dx + (radius - strokeWidth / 2) * math.cos(leftAngle);
    final leftY = center.dy + (radius - strokeWidth / 2) * math.sin(leftAngle);

    final rightX =
        center.dx + (radius - strokeWidth / 2) * math.cos(rightAngle);
    final rightY =
        center.dy + (radius - strokeWidth / 2) * math.sin(rightAngle);

    // Start time (current time - positioned at left end, above the arc)
    final startTextPainter = TextPainter(
      text: TextSpan(text: "00:00", style: textStyle),
      textDirection: TextDirection.ltr,
    );
    startTextPainter.layout();
    startTextPainter.paint(
      canvas,
      Offset(leftX - startTextPainter.width / 2, leftY - 20),
    );

    // End time (total duration - positioned at right end, above the arc)
    final endTime = _formatDuration(totalDuration);
    final endTextPainter = TextPainter(
      text: TextSpan(text: endTime, style: textStyle),
      textDirection: TextDirection.ltr,
    );
    endTextPainter.layout();
    endTextPainter.paint(
      canvas,
      Offset(rightX - endTextPainter.width / 2, rightY - 20),
    );
  }

  String _formatDuration(Duration duration) {
    String twoDigits(int n) => n.toString().padLeft(2, '0');
    final minutes = twoDigits(duration.inMinutes.remainder(60));
    final seconds = twoDigits(duration.inSeconds.remainder(60));
    return '$minutes:$seconds';
  }

  @override
  bool shouldRepaint(SemiCircularProgressPainter oldDelegate) {
    return oldDelegate.currentDuration != currentDuration ||
        oldDelegate.totalDuration != totalDuration ||
        oldDelegate.isDragging != isDragging;
  }
}

// Painter for the invisible hit area
class HitAreaPainter extends CustomPainter {
  @override
  void paint(Canvas canvas, Size size) {
    final center = Offset(size.width / 2, 0);
    final radius = size.width / 2;
    final strokeWidth = 6.0;

    // Draw a wide transparent stroke that creates the hit area
    final hitAreaPaint = Paint()
      ..color = Colors.transparent
      ..style = PaintingStyle.stroke
      ..strokeWidth = 120; // 60px on each side (increased from 100)

    canvas.drawArc(
      Rect.fromCircle(center: center, radius: radius - strokeWidth / 2),
      math.pi,
      -math.pi,
      false,
      hitAreaPaint,
    );
  }

  @override
  bool shouldRepaint(HitAreaPainter oldDelegate) => false;

  @override
  bool hitTest(Offset position) => true; // Always return true for hit testing
}