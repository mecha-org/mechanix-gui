import 'package:flutter/material.dart';
import 'dart:math' as math;

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
  int _dragUpdateCount = 0;
  DateTime? _lastDragTime;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 360,
      height: 180,
      child: MouseRegion(
        cursor: SystemMouseCursors.click,
        child: LayoutBuilder(
          builder: (context, constraints) {
            final size = Size(constraints.maxWidth, constraints.maxHeight);

            return GestureDetector(
              behavior: HitTestBehavior.translucent,

              // 👉 Tap to seek
              onTapDown:
                  (d) => _updateDragAngle(d.localPosition, size, isTap: true),

              // 👉 Drag to seek with velocity tracking
              onPanStart: (d) {
                setState(() => _isDragging = true);
                _dragUpdateCount = 0;
                _lastDragTime = DateTime.now();
                widget.onDragStart();

                final angle = _calculateAngle(d.localPosition, size);
                if (angle != null) {
                  _previousAngle = angle;
                }
              },
              onPanUpdate: (d) {
                _updateDragAngle(d.localPosition, size, isTap: false);
              },
              onPanEnd: (_) {
                setState(() => _isDragging = false);
                _dragUpdateCount = 0;
                _lastDragTime = null;
                widget.onDragEnd();
              },

              child: CustomPaint(
                size: size,
                painter: SemiCircularProgressPainter(
                  currentDuration: widget.currentDuration,
                  totalDuration: widget.totalDuration,
                  isDragging: _isDragging,
                ),
              ),
            );
          },
        ),
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

    // 🎯 EXPANDED hit test → Larger drag area for easier interaction
    if ((distance - trackRadius).abs() > 50) return null;

    double angle = math.atan2(dy, dx);

    // Normalize to 0 → 2π
    if (angle < 0) angle += 2 * math.pi;

    // 🚫 Reject bottom half — ONLY top semi-circle allowed
    if (angle > math.pi) return null;

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

  SemiCircularProgressPainter({
    required this.currentDuration,
    required this.totalDuration,
    required this.isDragging,
  });

  @override
  void paint(Canvas canvas, Size size) {
    final center = Offset(size.width / 2, 0);
    final radius = size.width / 2;
    final strokeWidth = 6.0;

    // Background track (inactive part)
    final backgroundPaint =
        Paint()
          ..color = Colors.grey.withOpacity(0.3)
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
          ..color = Colors.white
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
          ..color = Colors.white
          ..style = PaintingStyle.fill;

    // Draw larger thumb when dragging
    final thumbRadius = isDragging ? 10.0 : 8.0;
    canvas.drawCircle(Offset(thumbX, thumbY), thumbRadius, thumbPaint);

    // Draw time labels
    final textStyle = TextStyle(
      color: Colors.white.withOpacity(0.7),
      fontSize: 12,
      fontWeight: FontWeight.w500,
    );

    // Start time (left)
    final startTime = _formatDuration(currentDuration);
    final startTextPainter = TextPainter(
      text: TextSpan(text: startTime, style: textStyle),
      textDirection: TextDirection.ltr,
    );
    startTextPainter.layout();
    startTextPainter.paint(canvas, Offset(10, 10));

    // End time (right)
    final endTime = _formatDuration(totalDuration);
    final endTextPainter = TextPainter(
      text: TextSpan(text: endTime, style: textStyle),
      textDirection: TextDirection.ltr,
    );
    endTextPainter.layout();
    endTextPainter.paint(
      canvas,
      Offset(size.width - endTextPainter.width - 10, 10),
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
