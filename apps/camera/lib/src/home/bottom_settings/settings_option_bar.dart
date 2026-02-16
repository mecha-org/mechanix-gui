import 'package:flutter/material.dart';
import 'package:widgets/extensions/color.dart';

class SettingsOptionsBar extends StatelessWidget {
  final List<Widget> children;

  const SettingsOptionsBar({super.key, required this.children});

  @override
  Widget build(BuildContext context) {
    return Container(
      width: double.infinity,
      height: 44,
      padding: const EdgeInsets.all(5),
      decoration: BoxDecoration(
        color: context.secondary,
        borderRadius: const BorderRadius.only(
          topLeft: Radius.circular(8),
          topRight: Radius.circular(8),
        ),
      ),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        mainAxisSize: MainAxisSize.max,
        spacing: 24,
        children: children,
      ),
    );
  }
}
