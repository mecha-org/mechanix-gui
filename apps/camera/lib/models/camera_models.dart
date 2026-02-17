enum CaptureMode { photo, video }

enum CameraSettingType { frameResize, hd, grid, timer, audio, none }

enum AudioMode { on, off }

enum CameraTimer {
  fifteen(15, '15s'),
  ten(10, '10s'),
  five(5, '5s'),
  off(0, 'Off');

  final int seconds;
  final String label;

  const CameraTimer(this.seconds, this.label);
}

enum CameraGrid {
  threeByThree(3),
  fourByFour(4),
  off(0);

  final int divisions;
  const CameraGrid(this.divisions);

  bool get isEnabled => this != CameraGrid.off;
}

enum CameraResolution {
  eight(8, '8MP'),
  four(4, '4MP'),
  two(2, '2MP');

  final int megapixels;
  final String label;

  const CameraResolution(this.megapixels, this.label);
}

enum CameraAspectRatio {
  oneByOne(1, 1, '1:1'),
  fourByThree(4, 3, '4:3'),
  sixteenByNine(16, 9, '16:9'),
  off(0, 0, 'Off');

  final int width;
  final int height;
  final String label;

  const CameraAspectRatio(this.height, this.width, this.label);

  double? get value => this == CameraAspectRatio.off ? null : width / height;

  bool get isEnabled => this != CameraAspectRatio.off;
}
