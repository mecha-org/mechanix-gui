import 'package:flutter/material.dart';
import 'package:widgets/extensions/color.dart';

class CaptureButton extends StatefulWidget {
  final ValueNotifier<double> zoomLevel;
  final ValueNotifier<bool> isRecording;
  final ValueNotifier<double> captureButtonOffset;
  const CaptureButton({
    super.key,
    required this.zoomLevel,
    required this.isRecording,
    required this.captureButtonOffset,
  });

  @override
  State<CaptureButton> createState() => _CaptureButtonState();
}

class _CaptureButtonState extends State<CaptureButton> {
  static const double maxDragDistance = 20.0;
  static const double minZoom = 1.0;
  static const double maxZoom = 2.0;

  void updateZoomFromOffset(double offset) {
    final normalizedOffset = (offset / maxDragDistance).clamp(-1.0, 1.0);
    final zoomRange = maxZoom - minZoom;
    widget.zoomLevel.value =
        minZoom + (zoomRange * ((normalizedOffset + 1) / 2));
  }

  @override
  Widget build(BuildContext context) {
    return Positioned(
      bottom: 60,
      left: 0,
      right: 0,
      child: Column(
        children: [
          GestureDetector(
            onTap: () {
              widget.isRecording.value = !widget.isRecording.value;
            },
            onVerticalDragUpdate: (details) {
              widget
                  .captureButtonOffset
                  .value = (widget.captureButtonOffset.value - details.delta.dy)
                  .clamp(-maxDragDistance, maxDragDistance);
              updateZoomFromOffset(widget.captureButtonOffset.value);
            },
            onVerticalDragEnd: (details) {
              widget.captureButtonOffset.value = 0.0;
              widget.zoomLevel.value = 1.0;
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
                                color:
                                    isRecording
                                        ? Colors.red
                                        : context.onSurface,
                                shape: BoxShape.circle,
                              ),
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
          const SizedBox(height: 30),
        ],
      ),
    );
  }
}
