import 'package:camera/camera.dart';
import 'package:flutter/material.dart';
import 'package:mechanix_camera/src/home/widgets/camera_frame_scope.dart';
import 'package:mechanix_camera/src/home/widgets/grid_layout.dart';
import 'package:mechanix_camera/src/home/widgets/reframe_size.dart';

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
    return Stack(
      fit: StackFit.expand,
      children: [
        // Camera Preview
        CameraPreview(cameraController!),
        // Image.asset("assets/images/demo.jpg", fit: BoxFit.cover),
        const Positioned.fill(
          child: CameraFrameProvider(
            child: Stack(
              fit: StackFit.expand,
              children: [ReframeSize(), GridLayout()],
            ),
          ),
        ),
      ],
    );
  }
}
