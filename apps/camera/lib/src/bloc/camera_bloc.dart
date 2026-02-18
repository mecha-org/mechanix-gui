import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/logger.dart';
import 'package:mechanix_camera/models/camera_models.dart';
import 'package:mechanix_camera/src/bloc/camera_event.dart';
import 'package:mechanix_camera/src/bloc/camera_state.dart';

class CameraBloc extends Bloc<CameraEvent, CameraState> {
  final Logger logger = Logger();
  CameraBloc() : super(const CameraState(captureMode: CaptureMode.photo)) {
    on<ToggleCaptureMode>(_toggleCaptureMode);
    on<ToggleSettingsMode>(_toggleSettingsMode);
    on<SwitchCameraSettingType>(_switchCameraSettingType);
    on<UpdateAspectRatio>(_updateAspectRatio);
    on<UpdateResolution>(_updateResolution);
    on<UpdateGrid>(_updateGrid);
    on<UpdateTimer>(_updateTimer);
    on<UpdateAudioMode>(_updateAudioMode);
  }

  void _toggleCaptureMode(ToggleCaptureMode event, Emitter emit) {
    final updatedCaptureMode =
        state.captureMode == CaptureMode.photo
            ? CaptureMode.video
            : CaptureMode.photo;

    logger.i('Toggling capture mode to $updatedCaptureMode');

    emit(state.copyWith(captureMode: updatedCaptureMode));
  }

  void _toggleSettingsMode(ToggleSettingsMode event, Emitter emit) {
    logger.i('Toggling settings mode to ${!state.isSettingsOpen}');
    emit(
      state.copyWith(
        isSettingsOpen: !state.isSettingsOpen,
        cameraSettingType: CameraSettingType.none,
      ),
    );
  }

  void _switchCameraSettingType(SwitchCameraSettingType event, Emitter emit) {
    logger.i('Switching camera setting type to ${event.cameraSettingType}');
    final updatedSettingType =
        event.cameraSettingType != state.cameraSettingType
            ? event.cameraSettingType
            : CameraSettingType.none;
    emit(state.copyWith(cameraSettingType: updatedSettingType));
  }

  void _updateAspectRatio(UpdateAspectRatio event, Emitter emit) {
    logger.i('Updating aspect ratio to ${event.aspectRatio.label}');
    emit(state.copyWith(aspectRatio: event.aspectRatio));
  }

  void _updateResolution(UpdateResolution event, Emitter emit) {
    logger.i('Updating resolution to ${event.resolution.label}');
    emit(state.copyWith(resolution: event.resolution));
  }

  void _updateGrid(UpdateGrid event, Emitter emit) {
    logger.i(
      'Updating grid to ${event.grid.name} (${event.grid.divisions} divisions)',
    );
    emit(state.copyWith(grid: event.grid));
  }

  void _updateTimer(UpdateTimer event, Emitter emit) {
    logger.i('Updating timer to ${event.timer.label}');
    emit(state.copyWith(timer: event.timer));
  }

  void _updateAudioMode(UpdateAudioMode event, Emitter emit) {
    logger.i('Updating audio mode to ${event.audioMode.name}');
    emit(state.copyWith(audioMode: event.audioMode));
  }
}
