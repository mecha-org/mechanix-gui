import 'package:flutter/material.dart';
import 'package:widgets/extensions/color.dart';

const borderSideStyle = BorderSide(color: Color(0xFF464646), width: 1);

titleStyle(BuildContext context) => TextStyle(
  color: context.onSurface,
  fontSize: 20,
  overflow: TextOverflow.ellipsis,
  fontWeight: FontWeight.w500,
  fontFamily: "Overused Grotesk",
);

normalStyle(BuildContext context) => TextStyle(
  fontSize: 20,
  color: context.onSecondaryFixed,
  overflow: TextOverflow.ellipsis,
  fontWeight: FontWeight.w400,
  fontFamily: "Overused Grotesk",
);
