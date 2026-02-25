import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_camera/db/camera_config.dart';
import 'package:mechanix_camera/src/bloc/camera_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_event.dart';
import 'package:mechanix_camera/src/bloc/camera_state.dart';
import 'package:mechanix_camera/src/home/bottom_settings/settings_button.dart';
import 'package:mechanix_camera/src/home/bottom_settings/settings_option_bar.dart';

class SoundSettings extends StatelessWidget {
  const SoundSettings({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocSelector<CameraBloc, CameraState, SoundMode>(
      selector: (state) => state.soundMode,
      builder: (context, soundMode) {
        return SettingsOptionsBar(
          children: [
            SettingsButton(
              onTap: () {
                context.read<CameraBloc>().add(
                  const UpdateSoundMode(SoundMode.on),
                );
              },
              text: 'On',
              isActive: soundMode == SoundMode.on,
            ),
            SettingsButton(
              onTap: () {
                context.read<CameraBloc>().add(
                  const UpdateSoundMode(SoundMode.off),
                );
              },
              text: 'Off',
              isActive: soundMode == SoundMode.off,
            ),
          ],
        );
      },
    );
  }
}
