import 'package:flutter/material.dart';

class FocusPreserveButton extends StatelessWidget {
  final Widget child;

  const FocusPreserveButton({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    return Focus(
      canRequestFocus: false,
      descendantsAreFocusable: false,
      child: child,
    );
  }
}
