import 'package:flutter/material.dart';

class ColorIcon extends StatelessWidget {
  final double height;
  final double width;
  final String color;
  const ColorIcon({
    super.key,
    this.height = 21,
    this.width = 21,
    required this.color,
  });

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: height,
      width: width,
      child: Container(color: color != "none" ? Color(int.parse(color)) : null),
    );
  }
}
