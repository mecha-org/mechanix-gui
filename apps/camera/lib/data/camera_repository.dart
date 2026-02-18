import 'package:mechanix_camera/db/camera_config.dart';

abstract class CameraRepository {
  Future<void> init();

  bool get isBoxOpen;

  Future<CameraConfig> getConfig();
  Future<CameraConfig> updateConfig({
    CaptureMode? captureMode,
    CameraAspectRatio? aspectRatio,
    CameraResolution? resolution,
    CameraGrid? grid,
    CameraTimer? timer,
    SoundMode? soundMode,
  });
}
