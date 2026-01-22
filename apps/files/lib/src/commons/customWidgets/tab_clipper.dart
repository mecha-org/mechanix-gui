import 'package:flutter/material.dart';

class TabClipper extends CustomClipper<Path> {
  final double shift; // Shift amount for the tab

  // Constructor
  TabClipper({required this.shift});

  @override
  Path getClip(Size size) {
    const double radius = 8; // Outer rounded corners
    const double tabH = 12; // Tab height
    final double tabW = 42; // Tab width
    const double tabSlope = 12; // Small sloped curve
    // final double shift =
    //     topTabRightSideShiftLength ?? 450; // Shift tab right by this amount

    final Path p = Path();

    // Bottom-left corner
    p.moveTo(radius, size.height);
    p.quadraticBezierTo(0, size.height, 0, size.height - radius);

    // Left side up to tab start
    p.lineTo(0, tabH + radius);

    // Slope into the tab
    p.quadraticBezierTo(
      0,
      tabH,
      tabSlope,
      tabH,
    );

    // Top of tab
    p.lineTo(shift + tabW, tabH);

    // Tab end slope upward
    p.quadraticBezierTo(
      shift + tabW + tabSlope,
      tabH,
      shift + tabW + tabSlope,
      tabH - tabSlope,
    );

    // Connect into top bar
    p.lineTo(shift + tabW + tabSlope, radius);
    p.quadraticBezierTo(
      shift + tabW + tabSlope,
      0,
      shift + tabW + tabSlope + radius,
      0,
    );

    // Top-right corner
    p.lineTo(size.width - radius, 0);
    p.quadraticBezierTo(size.width, 0, size.width, radius);

    // Right side down
    p.lineTo(size.width, size.height - radius);
    p.quadraticBezierTo(
      size.width,
      size.height,
      size.width - radius,
      size.height,
    );

    //  Bottom side
    p.lineTo(radius, size.height);

    p.close();
    return p;
  }

  @override
  bool shouldReclip(CustomClipper<Path> oldClipper) => false;
}
