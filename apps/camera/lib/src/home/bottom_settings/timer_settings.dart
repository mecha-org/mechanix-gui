import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_camera/models/camera_models.dart';
import 'package:mechanix_camera/src/bloc/camera_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_event.dart';
import 'package:mechanix_camera/src/bloc/camera_state.dart';
import 'package:mechanix_camera/src/home/bottom_settings/settings_button.dart';
import 'package:mechanix_camera/src/home/bottom_settings/settings_option_bar.dart';

class TimerSettings extends StatelessWidget {
  const TimerSettings({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocSelector<CameraBloc, CameraState, CameraTimer>(
      selector: (state) => state.timer,
      builder: (context, timer) {
        return SettingsOptionsBar(
          children: [
            SettingsButton(
              onTap: () {
                context.read<CameraBloc>().add(
                  const UpdateTimer(CameraTimer.fifteen),
                );
              },
              text: CameraTimer.fifteen.label,
              isActive: timer == CameraTimer.fifteen,
            ),
            SettingsButton(
              onTap: () {
                context.read<CameraBloc>().add(
                  const UpdateTimer(CameraTimer.ten),
                );
              },
              text: CameraTimer.ten.label,
              isActive: timer == CameraTimer.ten,
            ),
            SettingsButton(
              onTap: () {
                context.read<CameraBloc>().add(
                  const UpdateTimer(CameraTimer.five),
                );
              },
              text: CameraTimer.five.label,
              isActive: timer == CameraTimer.five,
            ),
            SettingsButton(
              onTap: () {
                context.read<CameraBloc>().add(
                  const UpdateTimer(CameraTimer.off),
                );
              },
              text: CameraTimer.off.label,
              isActive: timer == CameraTimer.off,
            ),
          ],
        );
      },
    );
  }
}
