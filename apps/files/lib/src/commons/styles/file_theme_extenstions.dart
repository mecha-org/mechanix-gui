import 'package:flutter/material.dart';

class FilesTheme extends ThemeExtension<FilesTheme> {
  final Color primaryColor;

  const FilesTheme({this.primaryColor = const Color(0xFFAA6400)});

  @override
  FilesTheme copyWith({Color? accent}) {
    return FilesTheme(primaryColor: accent ?? primaryColor);
  }

  @override
  FilesTheme lerp(ThemeExtension<dynamic>? other, double t) {
    if (other is! FilesTheme) return this;
    return FilesTheme(
      primaryColor: Color.lerp(primaryColor, other.primaryColor, t)!,
    );
  }
}
