import 'package:flutter/material.dart';
import 'package:mechanix_camera/src/home/bottom_actions.dart';
import 'package:mechanix_camera/src/home/capture_button.dart';
import 'package:mechanix_camera/src/home/circle_overlay_painter.dart';
import 'package:mechanix_camera/src/home/circular_frame_painter.dart';
import 'package:mechanix_camera/src/home/zoom_strips.dart';

import 'package:mechanix_camera/utils/icons/icon.dart';
import 'package:widgets/mechanix.dart';

class CameraScreen extends StatefulWidget {
  const CameraScreen({super.key});

  @override
  State<CameraScreen> createState() => _CameraScreenState();
}

class _CameraScreenState extends State<CameraScreen> {
  final ValueNotifier<double> _zoomLevel = ValueNotifier<double>(1.0);
  final ValueNotifier<bool> _isRecording = ValueNotifier<bool>(false);
  final ValueNotifier<double> _captureButtonOffset = ValueNotifier<double>(0.0);

  @override
  void dispose() {
    _zoomLevel.dispose();
    _isRecording.dispose();
    _captureButtonOffset.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final screenWidth = MediaQuery.of(context).size.width;
    final circleSize = screenWidth * 0.89;

    return Scaffold(
      body: Stack(
        children: [
          // Background Image
          Positioned.fill(
            child: Image.asset(
              CameraIcons.demoImage,
              fit: BoxFit.cover,
              errorBuilder: (context, error, stackTrace) {
                return Container(
                  decoration: BoxDecoration(
                    gradient: LinearGradient(
                      begin: Alignment.topCenter,
                      end: Alignment.bottomCenter,
                      colors: [Colors.grey[800]!, Colors.grey[900]!],
                    ),
                  ),
                );
              },
            ),
          ),
          // Bottom Action Buttons
          const BottomActions(),
          // Dark overlay outside circle (when zooming)
          ValueListenableBuilder<double>(
            valueListenable: _captureButtonOffset,
            builder: (context, offset, child) {
              if (offset == 0.0) return const SizedBox.shrink();
              return Positioned.fill(
                child: IgnorePointer(
                  child: CustomPaint(
                    painter: CircleOverlayPainter(
                      colorScheme: context.colorScheme,
                      circleSize: circleSize,
                      screenSize: MediaQuery.of(context).size,
                    ),
                  ),
                ),
              );
            },
          ),
          // Circular UI Overlay
          Center(
            child: CustomPaint(
              size: Size(circleSize, circleSize),
              painter: CircularFramePainter(colorScheme: context.colorScheme),
            ),
          ),

          // Capture Button with Drag Control
          CaptureButton(
            zoomLevel: _zoomLevel,
            isRecording: _isRecording,
            captureButtonOffset: _captureButtonOffset,
          ),

          // Zoom Strips Overlay (separate layer)
          ValueListenableBuilder<double>(
            valueListenable: _captureButtonOffset,
            builder: (context, offset, child) {
              if (offset == 0.0) return const SizedBox.shrink();
              return Center(
                child: ValueListenableBuilder<double>(
                  valueListenable: _zoomLevel,
                  builder: (context, zoom, child) {
                    return CustomPaint(
                      size: Size(circleSize, circleSize),
                      painter: ZoomStrips(zoomLevel: zoom),
                    );
                  },
                ),
              );
            },
          ),
        ],
      ),
    );
  }
}
