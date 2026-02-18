import 'package:camera/camera.dart';
import 'package:flutter/material.dart';
import 'package:widgets/extensions/color.dart';

class VideoBottomActions extends StatefulWidget {
  final CameraController? cameraController;
  final VoidCallback? onCapturePhoto;

  const VideoBottomActions({
    super.key,
    this.cameraController,
    this.onCapturePhoto,
  });

  @override
  State<VideoBottomActions> createState() => _VideoBottomActionsState();
}

class _VideoBottomActionsState extends State<VideoBottomActions> {
  bool isPaused = false;
  bool isProcessing = false;

  Future<void> _handlePauseResume() async {
    if (widget.cameraController == null || isProcessing) return;

    if (!widget.cameraController!.value.isInitialized) {
      debugPrint('Camera controller is not initialized');
      return;
    }

    if (!widget.cameraController!.value.isRecordingVideo) {
      debugPrint('Camera is not recording video');
      return;
    }

    setState(() {
      isProcessing = true;
    });

    try {
      if (isPaused) {
        // Resume video recording
        await widget.cameraController!.resumeVideoRecording();
        debugPrint('Video recording resumed');
      } else {
        // Pause video recording
        await widget.cameraController!.pauseVideoRecording();
        debugPrint('Video recording paused');
      }

      setState(() {
        isPaused = !isPaused;
        isProcessing = false;
      });
    } catch (e) {
      debugPrint('Error toggling video recording: $e');
      setState(() {
        isProcessing = false;
      });

      // Show error to user
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(
              'Failed to ${isPaused ? "resume" : "pause"} recording',
            ),
            duration: const Duration(seconds: 2),
          ),
        );
      }
    }
  }

  Future<void> _handleCapturePhoto() async {
    if (widget.cameraController == null || isProcessing) return;

    if (!widget.cameraController!.value.isInitialized) {
      debugPrint('Camera controller is not initialized');
      return;
    }

    setState(() {
      isProcessing = true;
    });

    try {
      // Capture photo while recording video
      final XFile photo = await widget.cameraController!.takePicture();
      debugPrint('Photo captured: ${photo.path}');

      widget.onCapturePhoto?.call();

      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Photo captured'),
            duration: Duration(seconds: 1),
          ),
        );
      }
    } catch (e) {
      debugPrint('Error capturing photo: $e');

      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          const SnackBar(
            content: Text('Failed to capture photo'),
            duration: Duration(seconds: 2),
          ),
        );
      }
    } finally {
      if (mounted) {
        setState(() {
          isProcessing = false;
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Positioned(
      bottom: 20,
      left: 0,
      right: 0,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 24),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          mainAxisSize: MainAxisSize.max,
          children: [
            // Pause/Resume button
            GestureDetector(
              onTap: isProcessing ? null : _handlePauseResume,
              child: Container(
                width: 44,
                height: 44,
                decoration: BoxDecoration(
                  shape: BoxShape.circle,
                  color: context.onSurface,
                  border: Border.all(color: context.onSurface, width: 2),
                ),
                child: Center(
                  child: Icon(
                    isPaused ? Icons.play_arrow : Icons.pause,
                    color:
                        isPaused
                            ? const Color(0xFFD30000)
                            : context.onSurfaceVariant,
                    size: 34,
                  ),
                ),
              ),
            ),
            // Photo capture button
            GestureDetector(
              onTap: isProcessing ? null : _handleCapturePhoto,
              child: Container(
                width: 44,
                height: 44,
                decoration: BoxDecoration(
                  shape: BoxShape.circle,
                  color: context.onSurface,
                  border: Border.all(color: context.onSurface, width: 2),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
