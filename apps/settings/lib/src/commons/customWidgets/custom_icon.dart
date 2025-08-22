import 'package:flutter/material.dart';

class CustomIcon extends StatelessWidget {
  final double width;
  final double height;
  final Widget icon;

  const CustomIcon(
      {super.key, this.width = 24, this.height = 24, required this.icon});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: height,
      width: width,
      child: icon,
    );
  }
}
