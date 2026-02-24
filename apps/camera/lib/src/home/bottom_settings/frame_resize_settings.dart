import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_camera/db/camera_config.dart';
import 'package:mechanix_camera/src/bloc/camera_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_event.dart';
import 'package:mechanix_camera/src/bloc/camera_state.dart';
import 'package:mechanix_camera/src/home/bottom_settings/settings_button.dart';
import 'package:mechanix_camera/src/home/bottom_settings/settings_option_bar.dart';

class FrameResizeSettings extends StatelessWidget {
  const FrameResizeSettings({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocSelector<CameraBloc, CameraState, CameraAspectRatio>(
      selector: (state) => state.aspectRatio,
      builder: (context, aspectRatio) {
        return SettingsOptionsBar(
          children: [
            SettingsButton(
              onTap: () {
                context.read<CameraBloc>().add(
                  const UpdateAspectRatio(CameraAspectRatio.oneByOne),
                );
              },
              text: CameraAspectRatio.oneByOne.label,
              isActive: aspectRatio == CameraAspectRatio.oneByOne,
            ),
            SettingsButton(
              onTap: () {
                context.read<CameraBloc>().add(
                  const UpdateAspectRatio(CameraAspectRatio.fourByThree),
                );
              },
              text: CameraAspectRatio.fourByThree.label,
              isActive: aspectRatio == CameraAspectRatio.fourByThree,
            ),
            SettingsButton(
              onTap: () {
                context.read<CameraBloc>().add(
                  const UpdateAspectRatio(CameraAspectRatio.sixteenByNine),
                );
              },
              text: CameraAspectRatio.sixteenByNine.label,
              isActive: aspectRatio == CameraAspectRatio.sixteenByNine,
            ),
            SettingsButton(
              onTap: () {
                context.read<CameraBloc>().add(
                  const UpdateAspectRatio(CameraAspectRatio.off),
                );
              },
              text: CameraAspectRatio.off.label,
              isActive: aspectRatio == CameraAspectRatio.off,
            ),
          ],
        );
      },
    );
  }
}
