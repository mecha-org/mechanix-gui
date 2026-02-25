import 'package:equatable/equatable.dart';
import 'package:mechanix_camera/db/camera_config.dart';
import 'package:mechanix_camera/models/camera_models.dart';

class CameraState extends Equatable {
  final CaptureMode captureMode;
  final bool isSettingsOpen;
  final CameraSettingType cameraSettingType;
  final CameraAspectRatio aspectRatio;
  final CameraResolution resolution;
  final CameraGrid grid;
  final CameraTimer timer;
  final SoundMode soundMode;
  final List<String> mediaPath;

  const CameraState({
    this.captureMode = CaptureMode.photo,
    this.isSettingsOpen = false,
    this.cameraSettingType = CameraSettingType.none,
    this.aspectRatio = CameraAspectRatio.off,
    this.resolution = CameraResolution.eight,
    this.grid = CameraGrid.off,
    this.timer = CameraTimer.off,
    this.soundMode = SoundMode.on,
    this.mediaPath = const [],
  });

  CameraState copyWith({
    CaptureMode? captureMode,
    bool? isSettingsOpen,
    CameraSettingType? cameraSettingType,
    CameraAspectRatio? aspectRatio,
    CameraResolution? resolution,
    CameraGrid? grid,
    CameraTimer? timer,
    SoundMode? soundMode,
    List<String>? mediaPath,
  }) {
    return CameraState(
      captureMode: captureMode ?? this.captureMode,
      isSettingsOpen: isSettingsOpen ?? this.isSettingsOpen,
      cameraSettingType: cameraSettingType ?? this.cameraSettingType,
      aspectRatio: aspectRatio ?? this.aspectRatio,
      resolution: resolution ?? this.resolution,
      grid: grid ?? this.grid,
      timer: timer ?? this.timer,
      soundMode: soundMode ?? this.soundMode,
      mediaPath: mediaPath ?? this.mediaPath,
    );
  }

  @override
  List<Object?> get props => [
    captureMode,
    isSettingsOpen,
    cameraSettingType,
    aspectRatio,
    resolution,
    grid,
    timer,
    soundMode,
    mediaPath,
  ];
}
