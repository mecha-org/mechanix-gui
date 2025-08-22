import 'package:flutter/material.dart';

class CustomCursorWidget extends StatelessWidget {
  final Widget child;
  const CustomCursorWidget({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      cursor: SystemMouseCursors.click,
      child: child,
    );
  }
}
