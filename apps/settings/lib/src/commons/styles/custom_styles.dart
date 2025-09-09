import 'package:flutter/material.dart';

const TextStyle baseHeaderStyle = TextStyle(
  color: Color(0xFFFAFBFC),
  fontWeight: FontWeight.w500,
  fontSize: 16,
);

BoxDecoration rowBoxDecoration = BoxDecoration(
  color: Color(0xFF2B2B2B),
  borderRadius: BorderRadius.circular(8),
);

const TextStyle secondaryHeaderStyle = TextStyle(
  color: Color(0xFF898A8D),
  fontWeight: FontWeight.w500,
  fontSize: 16,
);

ButtonStyle buttonStyle = ButtonStyle(
  backgroundColor: WidgetStateProperty.resolveWith<Color?>(
    (states) {
      if (states.contains(WidgetState.pressed)) {
        return Color(0xFF3A3A3A);
      }
      return Color(0xFF3A3A3A);
    },
  ),
  padding: WidgetStateProperty.all(
    EdgeInsets.symmetric(horizontal: 12, vertical: 8),
  ),
  foregroundColor: WidgetStateProperty.all(Color(0xFF2D8AFF)),
  shape: WidgetStateProperty.all(
    RoundedRectangleBorder(
      borderRadius: BorderRadius.circular(8),
    ),
  ),
);
