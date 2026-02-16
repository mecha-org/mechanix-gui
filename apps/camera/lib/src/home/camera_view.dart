import 'package:camera/camera.dart';
import 'package:flutter/material.dart';

class CameraView extends StatelessWidget {
  final CameraController? cameraController;
  final bool isCameraInitialized;

  const CameraView({
    super.key,
    this.cameraController,
    required this.isCameraInitialized,
  });

  @override
  Widget build(BuildContext context) {
    if (!isCameraInitialized || cameraController == null) {
      return const Center(child: CircularProgressIndicator());
    }
    return CameraPreview(cameraController!);
  }
}
