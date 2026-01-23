part of 'appearance_bloc.dart';

abstract class AppearanceEvent extends Equatable {
  @override
  List<Object> get props => [];
}

class AppearanceInit extends AppearanceEvent {}

class SetWallpaperEvent extends AppearanceEvent {
  final WallpaperPreviewType fileName;

  SetWallpaperEvent(this.fileName);

  @override
  List<Object> get props => [fileName];
}

class SetThemeModeEvent extends AppearanceEvent {
  final ThemeMode themeMode;

  SetThemeModeEvent(this.themeMode);

  @override
  List<Object> get props => [themeMode];
}

class SetThemeVariantEvent extends AppearanceEvent {
  final MechanixVariant themeVariant;

  SetThemeVariantEvent(this.themeVariant);

  @override
  List<Object> get props => [themeVariant];
}

class ApplyThemeVariantEvent extends AppearanceEvent {}

class ApplyWallpaperEvent extends AppearanceEvent {}
