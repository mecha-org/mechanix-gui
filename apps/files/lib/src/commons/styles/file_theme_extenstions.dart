import 'package:flutter/material.dart';

class FilesTheme extends ThemeExtension<FilesTheme> {
  final Color primaryColor;
  final Color disableColor;

  const FilesTheme({
    this.primaryColor = const Color(0xFFAA6400),
    this.disableColor = const Color(0xFF585858),
  });

  @override
  FilesTheme copyWith({
    Color? primaryColor,
    Color? disableColor,
  }) {
    return FilesTheme(
      primaryColor: primaryColor ?? this.primaryColor,
      disableColor: disableColor ?? this.disableColor,
    );
  }

  @override
  FilesTheme lerp(ThemeExtension<dynamic>? other, double t) {
    if (other is! FilesTheme) return this;

    return FilesTheme(
      primaryColor: Color.lerp(primaryColor, other.primaryColor, t)!,
      disableColor: Color.lerp(disableColor, other.disableColor, t)!,
    );
  }
}
