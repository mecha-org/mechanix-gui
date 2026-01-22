import 'package:bloc/bloc.dart';
import 'package:collection/collection.dart';
import 'package:equatable/equatable.dart';
import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/features/appearance/data/appearance_repository.dart';
import 'package:widgets/mechanix.dart';

part 'appearance_event.dart';
part 'appearance_state.dart';

class AppearanceBloc extends Bloc<AppearanceEvent, AppearanceState> {
  final AppearanceRepository appearanceRepository;

  AppearanceBloc({required this.appearanceRepository})
      : super(const AppearanceState(
          themeMode: ThemeMode.dark,
        )) {
    on<AppearanceInit>(_onAppearanceInit);

    on<SetThemeModeEvent>(_onSetThemeMode);

    on<SetWallpaperEvent>(_onSetWallpaper);

    on<SetThemeVariantEvent>(_onSetThemeVariant);

    on<ApplyThemeVariantEvent>(_applyTheme);

    on<ApplyWallpaperEvent>(_applyWallpaper);
  }

  Future<void> _onAppearanceInit(
      AppearanceInit event, Emitter<AppearanceState> emit) async {
    final currentTheme = await appearanceRepository.onInit();

    final accent = currentTheme?['accent']?.toOKLCHStringToColor();

    final appliedVariant = MechanixVariant.getAllVariants()
        .firstWhereOrNull((variant) => variant.color == accent);

    if (appliedVariant != null) {
      emit(state.copyWith(variant: appliedVariant));
      emit(state.copyWith(appliedVariant: appliedVariant));
    }
  }

  Future<void> _onSetThemeMode(
      SetThemeModeEvent event, Emitter<AppearanceState> emit) async {
    emit(state.copyWith(themeMode: event.themeMode));
  }

  Future<void> _onSetWallpaper(
      SetWallpaperEvent event, Emitter<AppearanceState> emit) async {
    emit(state.copyWith(wallpaperFileName: event.fileName));
  }

  Future<void> _onSetThemeVariant(
      SetThemeVariantEvent event, Emitter<AppearanceState> emit) async {
    emit(state.copyWith(variant: event.themeVariant));
  }

  Future<void> _applyTheme(
      ApplyThemeVariantEvent event, Emitter<AppearanceState> emit) async {
    await appearanceRepository.applyTheme(state.variant);

    emit(state.copyWith(appliedVariant: state.variant));
  }

  Future<void> _applyWallpaper(
      ApplyWallpaperEvent event, Emitter<AppearanceState> emit) async {
    // TODO: add apply wallpaper implementation
    emit(state.copyWith(appliedWallpaper: state.wallpaperFileName));
  }
}
