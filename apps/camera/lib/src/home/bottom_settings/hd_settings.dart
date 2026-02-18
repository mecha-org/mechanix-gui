import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_camera/models/camera_models.dart';
import 'package:mechanix_camera/src/bloc/camera_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_event.dart';
import 'package:mechanix_camera/src/bloc/camera_state.dart';
import 'package:mechanix_camera/src/home/bottom_settings/settings_button.dart';
import 'package:mechanix_camera/src/home/bottom_settings/settings_option_bar.dart';

class HdSettings extends StatelessWidget {
  const HdSettings({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocSelector<CameraBloc, CameraState, CameraResolution>(
      selector: (state) => state.resolution,
      builder: (context, resolution) {
        return SettingsOptionsBar(
          children: [
            SettingsButton(
              onTap: () {
                context.read<CameraBloc>().add(
                  const UpdateResolution(CameraResolution.eight),
                );
              },
              text: CameraResolution.eight.label,
              isActive: resolution == CameraResolution.eight,
            ),
            SettingsButton(
              onTap: () {
                context.read<CameraBloc>().add(
                  const UpdateResolution(CameraResolution.four),
                );
              },
              text: CameraResolution.four.label,
              isActive: resolution == CameraResolution.four,
            ),
            SettingsButton(
              onTap: () {
                context.read<CameraBloc>().add(
                  const UpdateResolution(CameraResolution.two),
                );
              },
              text: CameraResolution.two.label,
              isActive: resolution == CameraResolution.two,
            ),
          ],
        );
      },
    );
  }
}
