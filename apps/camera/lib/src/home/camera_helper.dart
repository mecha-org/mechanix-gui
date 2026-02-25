import 'dart:ui';

import 'package:camera/camera.dart';
import 'package:mechanix_camera/db/camera_config.dart';

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

  static Rect calculate({
    required double screenWidth,
    required double screenHeight,
    required CameraAspectRatio aspectRatio,
  }) {
    if (aspectRatio.height == 0 || aspectRatio.width == 0) {
      return Rect.fromLTWH(0, 0, screenWidth, screenHeight);
    }

    final targetAspectRatio = aspectRatio.width / aspectRatio.height;
    final screenAspectRatio = screenWidth / screenHeight;

    double frameWidth;
    double frameHeight;

    if (targetAspectRatio > screenAspectRatio) {
      // Frame is wider — fit to width
      frameWidth = screenWidth;
      frameHeight = frameWidth / targetAspectRatio;
    } else {
      // Frame is taller — fit to height
      frameHeight = screenHeight;
      frameWidth = frameHeight * targetAspectRatio;
    }

    final frameLeft = (screenWidth - frameWidth) / 2;
    final frameTop = (screenHeight - frameHeight) / 2;

    return Rect.fromLTWH(frameLeft, frameTop, frameWidth, frameHeight);
  }
}
