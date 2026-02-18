import 'package:flutter/material.dart';
import 'package:widgets/extensions/color.dart';

TextStyle settingsTextStyle(BuildContext context, {bool isActive = false}) {
  return TextStyle(
    color: isActive ? context.primaryContainer : context.onInverseSurface,
    fontSize: 20,
    fontWeight: FontWeight.w500,
  );
}
