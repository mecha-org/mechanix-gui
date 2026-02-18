import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/logger.dart';
import 'package:mechanix_camera/models/camera_models.dart';
import 'package:mechanix_camera/src/bloc/camera_event.dart';
import 'package:mechanix_camera/src/bloc/camera_state.dart';

class CameraBloc extends Bloc<CameraEvent, CameraState> {
  final Logger logger = Logger();
  CameraBloc() : super(CameraState(captureMode: CaptureMode.photo)) {
    on<ToggleCaptureMode>(_toggleCaptureMode);
    on<ToggleSettingsMode>(_toggleSettingsMode);
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
    emit(state.copyWith(isSettingsOpen: !state.isSettingsOpen));
  }
}
