import 'package:flutter/material.dart';

class WidgetsScreenPreview extends StatelessWidget {
  const WidgetsScreenPreview({super.key, required this.iconPath});

  final String iconPath;
  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 159,
      height: 182,
      child: Image.asset(''),
    );
  }
}
