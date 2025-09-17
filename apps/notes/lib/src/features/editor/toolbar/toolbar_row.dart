import 'package:flutter/material.dart';
import 'package:mechanix_notes/src/commons/styles/styles.dart';

class ToolbarRow extends StatelessWidget {
  final BoxDecoration? boxDecoration;
  final List<Widget> child;
  final bool isBorder;
  const ToolbarRow({
    super.key,
    this.boxDecoration,
    required this.child,
    this.isBorder = true,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 350,
      height: 52.5,
      decoration:
          isBorder
              ? BoxDecoration(border: Border(bottom: borderSideStyle))
              : null,
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: child,
      ),
    );
  }
}
