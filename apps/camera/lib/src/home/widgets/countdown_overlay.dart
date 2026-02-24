import 'dart:async';
import 'package:flutter/material.dart';
import 'package:widgets/extensions/color.dart';

class CountdownOverlay extends StatefulWidget {
  final int seconds;
  final VoidCallback onComplete;

  const CountdownOverlay({
    super.key,
    required this.seconds,
    required this.onComplete,
  });

  @override
  State<CountdownOverlay> createState() => _CountdownOverlayState();
}

class _CountdownOverlayState extends State<CountdownOverlay> {
  late int _remaining;
  Timer? _timer;

  static const _numberStyle = TextStyle(
    fontSize: 64,
    fontWeight: FontWeight.w500,
  );

  @override
  void initState() {
    super.initState();
    _remaining = widget.seconds;
    _startTick();
  }

  void _startTick() {
    _timer = Timer.periodic(const Duration(seconds: 1), (timer) {
      if (_remaining <= 1) {
        timer.cancel();
        widget.onComplete();
        return;
      }

      setState(() {
        _remaining--;
      });
    });
  }

  @override
  void dispose() {
    _timer?.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final numberColor = context.onSecondaryFixedVariant;

    return Center(
      child: AnimatedSwitcher(
        duration: const Duration(milliseconds: 400),
        transitionBuilder: (child, animation) {
          return FadeTransition(opacity: animation, child: child);
        },
        child: Text(
          '$_remaining',
          key: ValueKey(_remaining),
          style: _numberStyle.copyWith(color: numberColor),
        ),
      ),
    );
  }
}
