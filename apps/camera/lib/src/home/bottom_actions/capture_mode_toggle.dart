import 'package:flutter/material.dart';
import 'package:mechanix_camera/db/camera_config.dart';
import 'package:mechanix_camera/src/home/bottom_actions/mode_icon_button.dart';
import 'package:mechanix_camera/utils/icons/icon.dart';
import 'package:widgets/extensions/color.dart';

/// Toggle button for switching between photo and video capture modes
class CaptureModeToggle extends StatelessWidget {
  final CaptureMode currentMode;
  final VoidCallback onToggle;

  const CaptureModeToggle({
    super.key,
    required this.currentMode,
    required this.onToggle,
  });

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      cursor: SystemMouseCursors.click,
      child: GestureDetector(
        behavior: HitTestBehavior.translucent,
        onTap: onToggle,
        child: Container(
          padding: const EdgeInsets.all(2),
          decoration: BoxDecoration(
            color: context.secondaryContainer.withValues(alpha: 0.7),
            borderRadius: BorderRadius.circular(8),
          ),
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              // Video mode button
              ModeIconButton(
                iconPath: CameraIcons.videoIcon,
                isActive: currentMode == CaptureMode.video,
                backgroundColor: context.surfaceContainerHigh.withValues(
                  alpha: 0.7,
                ),
              ),
              const SizedBox(width: 4),

              // Photo mode button
              ModeIconButton(
                iconPath: CameraIcons.cameraIcon,
                isActive: currentMode == CaptureMode.photo,
                backgroundColor: context.surfaceContainerHigh.withValues(
                  alpha: 0.7,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
