import 'package:flutter/material.dart';

class CustomTextButton extends StatelessWidget {
  final String label;
  final VoidCallback onPressed;
  final Color textColor;
  final double fontSize;
  final EdgeInsetsGeometry padding;

  const CustomTextButton({
    super.key,
    required this.label,
    required this.onPressed,
    this.textColor = Colors.white,
    this.fontSize = 20,
    this.padding = const EdgeInsets.symmetric(vertical: 6.0, horizontal: 4.0),
  });

  @override
  Widget build(BuildContext context) {
    return TextButton(
      onPressed: onPressed,
      child: Container(
        padding: padding,
        child: Text(
          label,
          textAlign: TextAlign.center,
          style: TextStyle(
            fontSize: fontSize,
            color: textColor,
          ),
        ),
      ),
    );
  }
}
