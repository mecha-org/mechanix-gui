import 'package:flutter/material.dart';

PopupMenuItem<String> buildStyledMenuItem(
  String value,
  String label,
  String icon,
  Color labelColor,
  Color iconColor,
) {
  return PopupMenuItem<String>(
    height: 40,
    value: value,
    padding: EdgeInsets.zero, // Removes default padding
    child: Container(
      width: 220, // Consistent width
      padding: const EdgeInsets.symmetric(
        horizontal: 16,
      ),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Text(
            label,
            style: TextStyle(color: labelColor, fontSize: 16),
          ),
          Image.asset(
            icon,
            height: 20,
            width: 20,
            color: iconColor,
          )
        ],
      ),
    ),
  );
}
