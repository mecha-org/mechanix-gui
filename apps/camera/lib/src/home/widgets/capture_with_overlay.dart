import 'package:flutter/material.dart';
import 'package:widgets/extension.dart';

class CaptureFlashOverlay extends StatefulWidget {
  const CaptureFlashOverlay({super.key});

  @override
  State<CaptureFlashOverlay> createState() => CaptureFlashOverlayState();
}

class CaptureFlashOverlayState extends State<CaptureFlashOverlay>
    with SingleTickerProviderStateMixin {
  late final AnimationController _controller;
  late final Animation<double> _opacity;

  @override
  void initState() {
    super.initState();

    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 350),
    );

    _opacity = TweenSequence<double>([
      TweenSequenceItem(
        tween: Tween(
          begin: 0.0,
          end: 1.0,
        ).chain(CurveTween(curve: Curves.easeIn)),
        weight: 15,
      ),
      TweenSequenceItem(
        tween: Tween(
          begin: 1.0,
          end: 0.0,
        ).chain(CurveTween(curve: Curves.easeOut)),
        weight: 85,
      ),
    ]).animate(_controller);
  }

  void flash() {
    _controller.forward(from: 0.0);
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AnimatedBuilder(
      animation: _opacity,
      builder: (context, _) {
        if (_opacity.value == 0.0) return const SizedBox.shrink();
        return Positioned.fill(
          child: IgnorePointer(
            child: DecoratedBox(
              decoration: BoxDecoration(
                color: context.secondary.withValues(
                  alpha: _opacity.value,
                ),
              ),
            ),
          ),
        );
      },
    );
  }
}
