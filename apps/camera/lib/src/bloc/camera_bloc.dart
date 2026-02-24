import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/logger.dart';
import 'package:mechanix_camera/data/camera_repository.dart';
import 'package:mechanix_camera/db/camera_config.dart';
import 'package:mechanix_camera/models/camera_models.dart';
import 'package:mechanix_camera/src/bloc/camera_event.dart';
import 'package:mechanix_camera/src/bloc/camera_state.dart';

class CameraBloc extends Bloc<CameraEvent, CameraState> {
  final Logger _logger = Logger();
  final CameraRepository _repository;

  CameraBloc({required CameraRepository repository})
    : _repository = repository,
      super(const CameraState(captureMode: CaptureMode.photo)) {
    on<InitialiseCamera>(_initialiseCamera);
    on<ToggleCaptureMode>(_toggleCaptureMode);
    on<ToggleSettingsMode>(_toggleSettingsMode);
    on<SwitchCameraSettingType>(_switchCameraSettingType);
    on<UpdateAspectRatio>(_updateAspectRatio);
    on<UpdateResolution>(_updateResolution);
    on<UpdateGrid>(_updateGrid);
    on<UpdateTimer>(_updateTimer);
    on<UpdateSoundMode>(_updateSoundMode);
    on<UpdateMediaPath>(_updateLastMediaPath);
    add(const InitialiseCamera());
  }

  Future<void> _initialiseCamera(
    InitialiseCamera event,
    Emitter<CameraState> emit,
  ) async {
    try {
      final config = await _repository.getConfig();
      _logger.i('Camera config loaded: $config');

      emit(
        state.copyWith(
          captureMode: config.captureMode,
          aspectRatio: config.aspectRatio,
          resolution: config.resolution,
          grid: config.grid,
          timer: config.timer,
          soundMode: config.soundMode,
        ),
      );
    } catch (e, st) {
      _logger.e('Failed to load camera config', error: e, stackTrace: st);
      // State stays at default — app still works
    }
  }

  Future<void> _toggleCaptureMode(
    ToggleCaptureMode event,
    Emitter<CameraState> emit,
  ) async {
    final updated =
        state.captureMode == CaptureMode.photo
            ? CaptureMode.video
            : CaptureMode.photo;

    _logger.i('Toggling capture mode to $updated');

    final config = await _repository.updateConfig(captureMode: updated);
    emit(state.copyWith(captureMode: config.captureMode));
  }

  void _toggleSettingsMode(
    ToggleSettingsMode event,
    Emitter<CameraState> emit,
  ) {
    _logger.i('Toggling settings mode to ${!state.isSettingsOpen}');
    emit(
      state.copyWith(
        isSettingsOpen: !state.isSettingsOpen,
        cameraSettingType: CameraSettingType.none,
      ),
    );
  }

  void _switchCameraSettingType(
    SwitchCameraSettingType event,
    Emitter<CameraState> emit,
  ) {
    _logger.i('Switching camera setting type to ${event.cameraSettingType}');
    final updated =
        event.cameraSettingType != state.cameraSettingType
            ? event.cameraSettingType
            : CameraSettingType.none;
    emit(state.copyWith(cameraSettingType: updated));
  }

  Future<void> _updateAspectRatio(
    UpdateAspectRatio event,
    Emitter<CameraState> emit,
  ) async {
    _logger.i('Updating aspect ratio to ${event.aspectRatio.label}');
    final config = await _repository.updateConfig(
      aspectRatio: event.aspectRatio,
    );
    emit(state.copyWith(aspectRatio: config.aspectRatio));
  }

  Future<void> _updateResolution(
    UpdateResolution event,
    Emitter<CameraState> emit,
  ) async {
    _logger.i('Updating resolution to ${event.resolution.label}');
    final config = await _repository.updateConfig(resolution: event.resolution);
    emit(state.copyWith(resolution: config.resolution));
  }

  Future<void> _updateGrid(UpdateGrid event, Emitter<CameraState> emit) async {
    _logger.i(
      'Updating grid to ${event.grid.name} (${event.grid.divisions} divisions)',
    );
    final config = await _repository.updateConfig(grid: event.grid);
    emit(state.copyWith(grid: config.grid));
  }

  Future<void> _updateTimer(
    UpdateTimer event,
    Emitter<CameraState> emit,
  ) async {
    _logger.i('Updating timer to ${event.timer.label}');
    final config = await _repository.updateConfig(timer: event.timer);
    emit(state.copyWith(timer: config.timer));
  }

  Future<void> _updateSoundMode(
    UpdateSoundMode event,
    Emitter<CameraState> emit,
  ) async {
    _logger.i('Updating sound mode to ${event.soundMode.name}');
    final config = await _repository.updateConfig(soundMode: event.soundMode);
    emit(state.copyWith(soundMode: config.soundMode));
  }

  void _updateLastMediaPath(UpdateMediaPath event, Emitter<CameraState> emit) {
    _logger.i('Updating last media path to ${event.path}');
    if (state.mediaPath.contains(event.path)) return;
    final updatedList = List<String>.from(state.mediaPath)..add(event.path);
    emit(state.copyWith(mediaPath: updatedList));
  }
}
