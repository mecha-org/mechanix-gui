import 'package:equatable/equatable.dart';
import 'package:mechanix_camera/models/camera_models.dart';

abstract class CameraEvent extends Equatable {
  const CameraEvent();

  @override
  List<Object?> get props => [];
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

class UpdateAudioMode extends CameraEvent {
  final AudioMode audioMode;
  const UpdateAudioMode(this.audioMode);
}
