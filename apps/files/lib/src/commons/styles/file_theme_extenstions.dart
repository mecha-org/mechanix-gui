import 'package:flutter/material.dart';

class FilesTheme extends ThemeExtension<FilesTheme> {
  final Color primaryColor;
  final Color disableColor;
  final String defaultFontFamily;

  const FilesTheme({
    this.primaryColor = const Color(0xFFAA6400),
    this.disableColor = const Color(0xFF585858),
    this.defaultFontFamily = 'Overused Grotesk',
  });

  @override
  FilesTheme copyWith({
    Color? primaryColor,
    Color? disableColor,
    String? defaultFontFamily,
  }) {
    return FilesTheme(
      primaryColor: primaryColor ?? this.primaryColor,
      disableColor: disableColor ?? this.disableColor,
      defaultFontFamily: defaultFontFamily ?? this.defaultFontFamily,
    );
  }

  @override
  FilesTheme lerp(ThemeExtension<dynamic>? other, double t) {
    if (other is! FilesTheme) return this;

    return FilesTheme(
      primaryColor: Color.lerp(primaryColor, other.primaryColor, t)!,
      disableColor: Color.lerp(disableColor, other.disableColor, t)!,
      defaultFontFamily: t < 0.5 ? defaultFontFamily : other.defaultFontFamily,
    );
  }
}
