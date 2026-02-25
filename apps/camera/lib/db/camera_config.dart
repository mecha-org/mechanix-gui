import 'package:hive/hive.dart';

part 'camera_config.g.dart';

@HiveType(typeId: 1)
enum CaptureMode {
  @HiveField(0)
  photo,
  @HiveField(1)
  video,
}

@HiveType(typeId: 2)
enum SoundMode {
  @HiveField(0)
  on,
  @HiveField(1)
  off,
}

@HiveType(typeId: 3)
enum CameraTimer {
  @HiveField(0)
  ten(10, '10s'),
  @HiveField(1)
  five(5, '5s'),
  @HiveField(2)
  three(3, '3s'),
  @HiveField(3)
  off(0, 'Off');

  final int seconds;
  final String label;
  const CameraTimer(this.seconds, this.label);
}

@HiveType(typeId: 4)
enum CameraGrid {
  @HiveField(0)
  threeByThree(3),
  @HiveField(1)
  fourByFour(4),
  @HiveField(2)
  off(0);

  final int divisions;
  const CameraGrid(this.divisions);

  bool get isEnabled => this != CameraGrid.off;
}

@HiveType(typeId: 5)
enum CameraResolution {
  @HiveField(0)
  eight(8, '8MP'),
  @HiveField(1)
  four(4, '4MP'),
  @HiveField(2)
  two(2, '2MP');

  final int megapixels;
  final String label;
  const CameraResolution(this.megapixels, this.label);
}

@HiveType(typeId: 6)
enum CameraAspectRatio {
  @HiveField(0)
  oneByOne(1, 1, '1:1'),
  @HiveField(1)
  fourByThree(4, 3, '4:3'),
  @HiveField(2)
  sixteenByNine(16, 9, '16:9'),
  @HiveField(3)
  off(0, 0, 'Off');

  final int width;
  final int height;
  final String label;
  const CameraAspectRatio(this.height, this.width, this.label);

  double? get value => this != CameraAspectRatio.off ? width / height : null;
  bool get isEnabled => this != CameraAspectRatio.off;
}

@HiveType(typeId: 0)
class CameraConfig extends HiveObject {
  @HiveField(0)
  CaptureMode captureMode;

  @HiveField(1)
  CameraAspectRatio aspectRatio;

  @HiveField(2)
  CameraResolution resolution;

  @HiveField(3)
  CameraGrid grid;

  @HiveField(4)
  CameraTimer timer;

  @HiveField(5)
  SoundMode soundMode;

  CameraConfig({
    this.captureMode = CaptureMode.photo,
    this.aspectRatio = CameraAspectRatio.fourByThree,
    this.resolution = CameraResolution.eight,
    this.grid = CameraGrid.off,
    this.timer = CameraTimer.off,
    this.soundMode = SoundMode.on,
  });

  CameraConfig copyWith({
    CaptureMode? captureMode,
    CameraAspectRatio? aspectRatio,
    CameraResolution? resolution,
    CameraGrid? grid,
    CameraTimer? timer,
    SoundMode? soundMode,
  }) {
    return CameraConfig(
      captureMode: captureMode ?? this.captureMode,
      aspectRatio: aspectRatio ?? this.aspectRatio,
      resolution: resolution ?? this.resolution,
      grid: grid ?? this.grid,
      timer: timer ?? this.timer,
      soundMode: soundMode ?? this.soundMode,
    );
  }

  @override
  String toString() =>
      'CameraConfig(\n'
      '  captureMode: $captureMode,\n'
      '  aspectRatio: ${aspectRatio.label},\n'
      '  resolution:  ${resolution.label},\n'
      '  grid:        ${grid.divisions} divisions,\n'
      '  timer:       ${timer.label},\n'
      '  soundMode:   $soundMode,\n'
      ')';
}
