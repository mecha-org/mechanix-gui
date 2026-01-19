import 'package:flutter/material.dart';
import 'package:widgets/extensions/color.dart';

class AppTitle extends StatelessWidget {
  const AppTitle({super.key});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: EdgeInsets.fromLTRB(16, 8, 16, 0),
      child: Align(
        alignment: Alignment.centerLeft,
        child: Text(
          'Notes',
          style: TextStyle(
            fontSize: 32,
            height: 1.3,
            letterSpacing: -1.1,
            fontWeight: FontWeight.w600,
            color: context.primary,
          ),
        ),
      ),
    );
  }
}
