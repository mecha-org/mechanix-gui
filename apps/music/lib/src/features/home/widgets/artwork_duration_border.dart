import 'dart:async';
import 'package:flutter/material.dart';
import 'package:media_kit/media_kit.dart';

class AnimatedCircularProgress extends StatefulWidget {
  final Player player;
  final double size;
  final double strokeWidth;
  final Color progressColor;
  final Color backgroundColor;
  final Widget? child;

  const AnimatedCircularProgress({
    super.key,
    required this.player,
    this.size = 48.0,
    this.strokeWidth = 2.18,
    this.progressColor = const Color(0xFF1DB954), // Spotify green
    this.backgroundColor = Colors.transparent,
    this.child,
  });

  @override
  State<AnimatedCircularProgress> createState() =>
      _AnimatedCircularProgressState();
}

class _AnimatedCircularProgressState extends State<AnimatedCircularProgress> {
  double _progress = 0.0;
  bool _isTransitioning = false;
  Timer? _debounceTimer;
  StreamSubscription? _positionSubscription;
  StreamSubscription? _durationSubscription;

  @override
  void initState() {
    super.initState();
    _listenToPlayer();
  }

  void _listenToPlayer() {
    // Listen to position stream for smooth updates
    _positionSubscription = widget.player.stream.position.listen((position) {
      // Only update if not transitioning
      if (!_isTransitioning) {
        final duration = widget.player.state.duration;

        if (duration != Duration.zero && mounted) {
          setState(() {
            _progress = position.inMilliseconds / duration.inMilliseconds;
          });
        }
      }
    });

    // Debounce track changes
    _durationSubscription = widget.player.stream.duration.listen((duration) {
      if (mounted) {
        // Cancel previous timer
        _debounceTimer?.cancel();

        // Set transitioning state
        setState(() {
          _isTransitioning = true;
          _progress = 0.0;
        });

        // Wait for track to stabilize (300-500ms is usually good)
        _debounceTimer = Timer(const Duration(milliseconds: 400), () {
          if (mounted) {
            setState(() {
              _isTransitioning = false;
            });
          }
        });
      }
    });
  }

  @override
  void dispose() {
    _debounceTimer?.cancel();
    _positionSubscription?.cancel();
    _durationSubscription?.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return RepaintBoundary(
      child: SizedBox(
        width: widget.size,
        height: widget.size,
        child: Stack(
          alignment: Alignment.center,
          children: [
            // Animated progress indicator
            AnimatedOpacity(
              opacity: _isTransitioning ? 0.3 : 1.0, // Fade during transition
              duration: const Duration(milliseconds: 200),
              child: SizedBox(
                width: widget.size,
                height: widget.size,
                child: Transform.scale(
                  scaleX: -1, // Flip horizontally for anti-clockwise
                  child: CircularProgressIndicator(
                    padding: EdgeInsets.all(5),
                    trackGap: 10,
                    value: _progress,
                    strokeWidth: widget.strokeWidth,
                    backgroundColor: widget.backgroundColor,
                    valueColor: AlwaysStoppedAnimation<Color>(
                      widget.progressColor,
                    ),
                  ),
                ),
              ),
            ),
            // Center child (artwork)
            if (widget.child != null) widget.child!,
          ],
        ),
      ),
    );
  }
}