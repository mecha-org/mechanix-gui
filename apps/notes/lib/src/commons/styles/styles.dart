import 'package:flutter/material.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';

const borderSideStyle = BorderSide(color: Color(0xFF464646), width: 1);

final titleStyle = const TextStyle(
  color: NotesColors.titleTextColor,
  fontSize: 18,
  overflow: TextOverflow.ellipsis,
  fontWeight: FontWeight.w500,
);

final normalStyle = const TextStyle(
  fontSize: 18,
  color: NotesColors.labelColor,
  overflow: TextOverflow.ellipsis,
  fontWeight: FontWeight.w400,
);
