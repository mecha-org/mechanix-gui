import 'package:equatable/equatable.dart';
import 'package:mechanix_camera/models/camera_models.dart';

class CameraState extends Equatable {
  final CaptureMode captureMode;
  final bool isSettingsOpen;
  final CameraSettingType? cameraSettingType;

  const CameraState({
    this.captureMode = CaptureMode.photo,
    this.isSettingsOpen = false,
    this.cameraSettingType,
  });

  CameraState copyWith({
    CaptureMode? captureMode,
    bool? isSettingsOpen,
    CameraSettingType? cameraSettingType,
  }) {
    return CameraState(
      captureMode: captureMode ?? this.captureMode,
      isSettingsOpen: isSettingsOpen ?? this.isSettingsOpen,
      cameraSettingType: cameraSettingType ?? this.cameraSettingType,
    );
  }

  @override
  List<Object?> get props => [captureMode, isSettingsOpen, cameraSettingType];
}
