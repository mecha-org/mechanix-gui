import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_event.dart';
import 'package:mechanix_camera/src/bloc/camera_state.dart';
import 'package:mechanix_camera/models/camera_models.dart';
import 'package:mechanix_camera/src/home/bottom_actions/capture_mode_toggle.dart';
import 'package:mechanix_camera/src/home/bottom_actions/gallery_thumbnail.dart';
import 'package:mechanix_camera/src/home/bottom_actions/settings_button.dart';

/// Main bottom actions bar for the camera interface
class BottomActions extends StatelessWidget {
  const BottomActions({super.key});

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
            // Left side: Gallery thumbnail
            const GalleryThumbnail(),

            // Right side: Camera controls
            Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                BlocSelector<CameraBloc, CameraState, CaptureMode>(
                  selector: (state) => state.captureMode,
                  builder:
                      (context, captureMode) => CaptureModeToggle(
                        currentMode: captureMode,
                        onToggle: () {
                          context.read<CameraBloc>().add(ToggleCaptureMode());
                        },
                      ),
                ),
                const SizedBox(width: 12),

                SettingsButton(
                  onTap: () {
                    context.read<CameraBloc>().add(ToggleSettingsMode());
                  },
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
