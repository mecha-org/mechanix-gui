import 'package:flutter/material.dart';
import 'package:widgets/extensions/color.dart';

class BulletListBuilder extends StatelessWidget {
  const BulletListBuilder({super.key});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(right: 0, top: 15),
      child: Container(
        width: 4,
        height: 4,
        decoration: BoxDecoration(
          color: context.onSurface,
          shape: BoxShape.circle,
        ),
      ),
    );
  }
}
