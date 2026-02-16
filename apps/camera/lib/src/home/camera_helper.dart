import 'package:camera/camera.dart';

class CameraHelper {
  static Future<CameraDescription?> getBackCamera() async {
    try {
      final cameras = await availableCameras();

      if (cameras.isEmpty) return null;

      try {
        return cameras.firstWhere(
          (camera) => camera.lensDirection == CameraLensDirection.back,
        );
      } catch (e) {
        return cameras.first;
      }
    } catch (e) {
      return null;
    }
  }

  static Future<CameraDescription?> getFrontCamera() async {
    try {
      final cameras = await availableCameras();

      if (cameras.isEmpty) return null;

      return cameras.firstWhere(
        (camera) => camera.lensDirection == CameraLensDirection.front,
        orElse: () => cameras.first,
      );
    } catch (e) {
      return null;
    }
  }

  static double calculateZoomFromOffset({
    required double offset,
    required double maxOffset,
    required double minZoom,
    required double maxZoom,
  }) {
    final normalizedOffset = (offset / maxOffset).clamp(0.0, 1.0);
    return minZoom + (normalizedOffset * (maxZoom - minZoom));
  }

  static String formatZoomLevel(double zoom) {
    return '${zoom.toStringAsFixed(1)}x';
  }
}
