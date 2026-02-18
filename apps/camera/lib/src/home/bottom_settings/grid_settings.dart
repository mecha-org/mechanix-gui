import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_camera/models/camera_models.dart';
import 'package:mechanix_camera/src/bloc/camera_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_event.dart';
import 'package:mechanix_camera/src/bloc/camera_state.dart';
import 'package:mechanix_camera/src/home/bottom_settings/settings_button.dart';
import 'package:mechanix_camera/src/home/bottom_settings/settings_option_bar.dart';

class GridSettings extends StatelessWidget {
  const GridSettings({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocSelector<CameraBloc, CameraState, CameraGrid>(
      selector: (state) => state.grid,
      builder: (context, grid) {
        return SettingsOptionsBar(
          children: [
            SettingsButton(
              onTap: () {
                context.read<CameraBloc>().add(
                  const UpdateGrid(CameraGrid.threeByThree),
                );
              },
              text: '3x3',
              isActive: grid == CameraGrid.threeByThree,
            ),
            SettingsButton(
              onTap: () {
                context.read<CameraBloc>().add(
                  const UpdateGrid(CameraGrid.fourByFour),
                );
              },
              text: '4x4',
              isActive: grid == CameraGrid.fourByFour,
            ),
            SettingsButton(
              onTap: () {
                context.read<CameraBloc>().add(
                  const UpdateGrid(CameraGrid.off),
                );
              },
              text: 'Off',
              isActive: grid == CameraGrid.off,
            ),
          ],
        );
      },
    );
  }
}
