import 'package:flutter/material.dart';

class SlideLeftTransitionsBuilder extends PageTransitionsBuilder {
  const SlideLeftTransitionsBuilder();

  @override
  Widget buildTransitions<T>(
    PageRoute<T> route,
    BuildContext context,
    Animation<double> animation,
    Animation<double> secondaryAnimation,
    Widget child,
  ) {
    const curve = Curves.easeOut;

    // Animate both push (enter from right) and pop (exit to left)
    var tween = Tween<Offset>(
      begin: const Offset(1.0, 0.0), // start off-screen right
      end: Offset.zero, // slide to normal position
    ).chain(CurveTween(curve: curve));

    return SlideTransition(
      position: animation.drive(tween),
      child: child,
    );
  }
}
