part of 'appearance_bloc.dart';

class AppearanceState extends Equatable {
  const AppearanceState({
    this.variant = MechanixVariant.amber,
    this.appliedVariant = MechanixVariant.amber,
    this.themeMode = ThemeMode.dark,
    this.wallpaperFileName = Images.wallpaper1,
    this.appliedWallpaper = Images.wallpaper1,
  });

  final ThemeMode themeMode;
  final MechanixVariant variant;
  final MechanixVariant appliedVariant;
  final String wallpaperFileName;
  final String appliedWallpaper;

  AppearanceState copyWith({
    ThemeMode? themeMode,
    MechanixVariant? variant,
    MechanixVariant? appliedVariant,
    String? wallpaperFileName,
    String? appliedWallpaper,
  }) {
    return AppearanceState(
      variant: variant ?? this.variant,
      appliedVariant: appliedVariant ?? this.appliedVariant,
      themeMode: themeMode ?? this.themeMode,
      wallpaperFileName: wallpaperFileName ?? this.wallpaperFileName,
      appliedWallpaper: appliedWallpaper ?? this.appliedWallpaper,
    );
  }

  @override
  List<Object?> get props => [
        themeMode,
        variant,
        appliedVariant,
        wallpaperFileName,
        appliedWallpaper,
      ];
}
