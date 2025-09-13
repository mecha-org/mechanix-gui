import 'package:flutter/material.dart';

class ToolbarContainer extends StatelessWidget {
  final List<Widget> child;

  final double? width;
  final double? height;
  const ToolbarContainer({
    super.key,
    required this.child,
    this.width = 350,
    this.height = 105,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(16),
        color: Color(0xFF2B2B2B),
      ),
      width: width,
      height: height,
      child: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        children: child,
      ),
    );
  }
}
