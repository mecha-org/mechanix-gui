import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_camera/models/camera_models.dart';
import 'package:mechanix_camera/src/bloc/camera_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_state.dart';
import 'package:widgets/extensions/color.dart';

class CaptureButton extends StatefulWidget {
  final ValueNotifier<double> zoomLevel;
  final ValueNotifier<bool> isRecording;
  final ValueNotifier<double> captureButtonOffset;
  final VoidCallback onCapture;
  final VoidCallback onStartRecording;
  final VoidCallback onStopRecording;
  final Function(double) onZoomChange;
  final double minZoom;
  final double maxZoom;

  const CaptureButton({
    super.key,
    required this.zoomLevel,
    required this.isRecording,
    required this.captureButtonOffset,
    required this.onCapture,
    required this.onStartRecording,
    required this.onStopRecording,
    required this.onZoomChange,
    required this.minZoom,
    required this.maxZoom,
  });

  @override
  State<CaptureButton> createState() => _CaptureButtonState();
}

class _CaptureButtonState extends State<CaptureButton> {
  static const double maxDragDistance = 30.0;

  void updateZoomFromOffset(double offset) {
    final normalizedOffset = (offset / maxDragDistance).clamp(-1.0, 1.0);
    final zoomRange = widget.maxZoom - widget.minZoom;
    final newZoom = widget.minZoom + (zoomRange * ((normalizedOffset + 1) / 2));
    widget.onZoomChange(newZoom);
  }

  void _handleTap(bool isVideoMode) {
    if (isVideoMode) {
      // Video mode: toggle recording
      if (!widget.isRecording.value) {
        widget.isRecording.value = true;
        widget.onStartRecording();
      } else {
        widget.onStopRecording();
        widget.isRecording.value = false;
      }
    } else {
      // Photo mode: capture photo
      widget.onCapture();
    }
  }

  @override
  Widget build(BuildContext context) {
    return Positioned(
      bottom: 60,
      left: 0,
      right: 0,
      child: Column(
        children: [
          BlocSelector<CameraBloc, CameraState, bool>(
            selector: (state) => state.captureMode == CaptureMode.video,
            builder:
                (context, isVideoMode) => GestureDetector(
                  onTap: () => _handleTap(isVideoMode),
                  onVerticalDragUpdate: (details) {
                    widget.captureButtonOffset.value =
                        (widget.captureButtonOffset.value - details.delta.dy)
                            .clamp(-maxDragDistance, maxDragDistance);
                    updateZoomFromOffset(widget.captureButtonOffset.value);
                  },
                  onVerticalDragEnd: (details) {
                    widget.captureButtonOffset.value = 0.0;
                    // widget.onZoomChange(widget.minZoom);
                  },
                  child: ValueListenableBuilder<double>(
                    valueListenable: widget.captureButtonOffset,
                    builder: (context, offset, child) {
                      return Container(
                        width: 60,
                        height: 60,
                        decoration: BoxDecoration(
                          shape: BoxShape.circle,
                          border: Border.all(
                            color: context.onSecondaryFixed,
                            width: 1.5,
                          ),
                        ),
                        clipBehavior: Clip.none,
                        child: Stack(
                          clipBehavior: Clip.none,
                          children: [
                            Positioned(
                              top: 6 - offset,
                              left: 6,
                              right: 6,
                              bottom: 6 + offset,
                              child: ValueListenableBuilder<bool>(
                                valueListenable: widget.isRecording,
                                builder: (context, isRecording, child) {
                                  return Container(
                                    width: 44,
                                    height: 44,
                                    decoration: BoxDecoration(
                                      shape: BoxShape.circle,
                                      color:
                                          isVideoMode && !isRecording
                                              ? Color(0xFFD30000)
                                              : context.onSurface,
                                      border: Border.all(
                                        color: context.onSurface,
                                        width: 2.87,
                                      ),
                                    ),
                                    child:
                                        isRecording
                                            ? Center(
                                              child: Container(
                                                width: 20,
                                                height: 20,
                                                decoration: BoxDecoration(
                                                  color:
                                                      context.onSurfaceVariant,
                                                  borderRadius:
                                                      BorderRadius.circular(
                                                        3.3,
                                                      ),
                                                ),
                                              ),
                                            )
                                            : null,
                                  );
                                },
                              ),
                            ),
                          ],
                        ),
                      );
                    },
                  ),
                ),
          ),
          const SizedBox(height: 30),
        ],
      ),
    );
  }
}