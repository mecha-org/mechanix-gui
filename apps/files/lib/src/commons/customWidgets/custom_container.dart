import 'package:flutter/material.dart';

class ContainerWidget extends StatelessWidget {
  final Widget child;
  const ContainerWidget({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.all(4),
      child: child,
    );
  }
}
