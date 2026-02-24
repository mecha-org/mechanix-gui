import 'package:equatable/equatable.dart';
import 'package:mechanix_camera/db/camera_config.dart';
import 'package:mechanix_camera/models/camera_models.dart';

abstract class CameraEvent extends Equatable {
  const CameraEvent();

  @override
  List<Object?> get props => [];
}

class InitialiseCamera extends CameraEvent {
  const InitialiseCamera();
}

class ToggleCaptureMode extends CameraEvent {}

class ToggleSettingsMode extends CameraEvent {}

class SwitchCameraSettingType extends CameraEvent {
  final CameraSettingType cameraSettingType;
  const SwitchCameraSettingType(this.cameraSettingType);
}

class UpdateAspectRatio extends CameraEvent {
  final CameraAspectRatio aspectRatio;
  const UpdateAspectRatio(this.aspectRatio);
}

class UpdateResolution extends CameraEvent {
  final CameraResolution resolution;
  const UpdateResolution(this.resolution);
}

class UpdateGrid extends CameraEvent {
  final CameraGrid grid;
  const UpdateGrid(this.grid);
}

class UpdateTimer extends CameraEvent {
  final CameraTimer timer;
  const UpdateTimer(this.timer);
}

class UpdateSoundMode extends CameraEvent {
  final SoundMode soundMode;
  const UpdateSoundMode(this.soundMode);
}

class UpdateMediaPath extends CameraEvent {
  final String path;
  const UpdateMediaPath(this.path);
}
