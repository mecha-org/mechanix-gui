import 'package:hive/hive.dart';
import 'package:mechanix_camera/data/camera_repository.dart';
import 'package:mechanix_camera/db/camera_config.dart';
import 'package:mechanix_camera/utils/constants.dart';

class CameraRepositoryImpl extends CameraRepository {
  static const String _configKey = 'camera_config';

  static final CameraConfig _defaultConfig = CameraConfig(
    captureMode: CaptureMode.photo,
    aspectRatio: CameraAspectRatio.fourByThree,
    resolution: CameraResolution.eight,
    grid: CameraGrid.off,
    timer: CameraTimer.off,
    soundMode: SoundMode.on,
  );

  Box<CameraConfig>? _box;

  @override
  bool get isBoxOpen => _box?.isOpen ?? false;

  Box<CameraConfig> get _openBox {
    if (!isBoxOpen) {
      throw StateError(
        'CameraRepository box is not open. '
        'Did you forget to call init()?',
      );
    }
    return _box!;
  }

  @override
  Future<void> init() async {
    _box = await Hive.openBox<CameraConfig>(Constants.cameraConfigTable);
  }

  @override
  Future<CameraConfig> getConfig() async {
    await init();

    final box = _openBox;

    // If no config stored yet → persist defaults and return them
    if (!box.containsKey(_configKey)) {
      await box.put(_configKey, _defaultConfig);
    }

    return box.get(_configKey)!;
  }

  @override
  Future<CameraConfig> updateConfig({
    CaptureMode? captureMode,
    CameraAspectRatio? aspectRatio,
    CameraResolution? resolution,
    CameraGrid? grid,
    CameraTimer? timer,
    SoundMode? soundMode,
  }) async {
    await init();
    final current = await getConfig();

    final updated = current.copyWith(
      captureMode: captureMode,
      aspectRatio: aspectRatio,
      resolution: resolution,
      grid: grid,
      timer: timer,
      soundMode: soundMode,
    );

    await _openBox.put(_configKey, updated);
    return updated;
  }
}
