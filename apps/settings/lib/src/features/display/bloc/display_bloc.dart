import 'dart:developer';

import 'package:equatable/equatable.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/logger.dart';
import 'package:mechanix_settings/src/features/display/data/display_repository.dart';

part 'display_event.dart';
part 'display_state.dart';

class DisplayBloc extends Bloc<DisplayEvent, DisplayState> {
  final logger = Logger();
  final DisplayRepository displayRepository;

  DisplayBloc({required this.displayRepository}) : super(DisplayState()) {
    on<GetDefaultSettingsEvent>(_getDisplaySettings);
    on<SetBrightnessEvent>(_setBrightnessChangeEnd);
    on<SetBrightnessChangeEvent>(_setBrightnessChanging);
    on<SetAutoBrightnessEvent>(_setAutoBrightness);
    on<SetDisplayTimeoutEvent>(_setScreenTimeout);
    on<SetLockScreenTimeoutEvent>(_setLockScreenTimeout);
  }

  Future<void> _getDisplaySettings(
      GetDefaultSettingsEvent event, Emitter<DisplayState> emit) async {
    try {
      final defaultSetting = await displayRepository.getDisplaySettings();

      print('defaultSetting - ${defaultSetting.brightness / 254}');
      log('defaultSetting brightnessValue - ${defaultSetting.brightness / 254}');
      debugPrint(
          'defaultSetting brightnessValue - ${defaultSetting.brightness / 254}');

      emit(state.copyWith(
        brightness: (defaultSetting.brightness / 254).toDouble(),
        isAutoBrightness: defaultSetting.autoBrightness,
        lockScreenTimeout: defaultSetting.lockScreenTimeout.toDouble(),
        screenTimeout: defaultSetting.displayTimeout.toDouble(),
      ));
    } catch (error) {
      logger.e('get brightness value error $error');
    }
  }

  Future<void> _setBrightnessChangeEnd(
      SetBrightnessEvent event, Emitter<DisplayState> emit) async {
    try {
      logger.i('set brightness value ${event.brightness}');
      await displayRepository.setBrightness((event.brightness));
      emit(state.copyWith(brightness: event.brightness));
    } catch (error) {
      logger.e('set brightness value error $error');
    }
  }

  Future<void> _setBrightnessChanging(
      SetBrightnessChangeEvent event, Emitter<DisplayState> emit) async {
    try {
      logger.i('set brightness value ${event.brightness}');
      print('brightness changing - ${event.brightness}');
      log('brightness changing - ${event.brightness}');
      debugPrint('brightness changing - ${event.brightness}');

      emit(state.copyWith(brightness: event.brightness));
    } catch (error) {
      logger.e('set brightness value error $error');
    }
  }

  Future<void> _setAutoBrightness(
      SetAutoBrightnessEvent event, Emitter<DisplayState> emit) async {
    try {
      logger.i('Setting auto brightness ${event.isAutoBrightness}');
      await displayRepository.setAutoBrightness(event.isAutoBrightness);
      emit(state.copyWith(isAutoBrightness: event.isAutoBrightness));
    } catch (error) {
      logger.e('Error setting auto brightness $error');
    }
  }

  Future<void> _setScreenTimeout(
      SetDisplayTimeoutEvent event, Emitter<DisplayState> emit) async {
    try {
      logger.i('Setting screen timeout ${event.displayTimeout}');
      await displayRepository.setScreenTimeout(event.displayTimeout.toInt());
      emit(state.copyWith(screenTimeout: event.displayTimeout));
    } catch (error) {
      logger.e('Error setting screen timeout  $error');
    }
  }

  Future<void> _setLockScreenTimeout(
      SetLockScreenTimeoutEvent event, Emitter<DisplayState> emit) async {
    try {
      logger.i('Setting lock screen timeout ${event.lockScreenTimeout}');

      await displayRepository
          .setLockScreenTimeout(event.lockScreenTimeout.toInt());

      emit(state.copyWith(lockScreenTimeout: event.lockScreenTimeout));
    } catch (error) {
      logger.e('Error setting lock screen error $error');
    }
  }
}
