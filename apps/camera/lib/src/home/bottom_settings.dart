import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_camera/models/camera_models.dart';
import 'package:mechanix_camera/src/bloc/camera_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_event.dart';
import 'package:mechanix_camera/src/bloc/camera_state.dart';
import 'package:mechanix_camera/src/home/bottom_settings/sound_settings.dart';
import 'package:mechanix_camera/src/home/bottom_settings/frame_resize_settings.dart';
import 'package:mechanix_camera/src/home/bottom_settings/grid_settings.dart';
import 'package:mechanix_camera/src/home/bottom_settings/hd_settings.dart';
import 'package:mechanix_camera/src/home/bottom_settings/timer_settings.dart';
import 'package:mechanix_camera/utils/icons/icon.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottom_bar/mechanix_bottom_bar_theme.dart';

class BottomSettings extends StatelessWidget {
  const BottomSettings({super.key});

  @override
  Widget build(BuildContext context) {
    return Positioned(
      left: 0,
      right: 0,
      bottom: 0,
      child: BlocSelector<CameraBloc, CameraState, CameraSettingType>(
        selector: (state) => state.cameraSettingType,
        builder: (context, cameraSettingType) {
          return Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              if (cameraSettingType != CameraSettingType.none)
                _buildSettingWidget(context, cameraSettingType),

              // Bottom bar
              MechanixBottomBar(
                centerWidgetSpacing: 16,
                theme: MechanixBottomBarThemeData(
                  iconTheme: const MechanixBottomBarIconThemeData(
                    iconSize: Size(28, 28),
                    iconBoxSize: Size(44, 44),
                  ),
                  decoration: BoxDecoration(
                    color: context.secondaryContainer,
                    borderRadius:
                        cameraSettingType == CameraSettingType.none
                            ? const BorderRadius.only(
                              topLeft: Radius.circular(8),
                              topRight: Radius.circular(8),
                            )
                            : BorderRadius.zero,
                  ),
                ),
                leadingWidget: [
                  BottomBarButton(
                    iconTheme: const MechanixBottomBarIconThemeData(
                      buttonMargin: EdgeInsets.only(left: 10),
                      iconSize: Size(28, 28),
                      iconBoxSize: Size(44, 44),
                    ),
                    onPressed: () {
                      context.read<CameraBloc>().add(ToggleSettingsMode());
                    },
                    iconPath: CameraIcons.backIcon,
                  ),
                ],
                centerWidget: [
                  BottomBarButton(
                    onPressed: () {
                      context.read<CameraBloc>().add(
                        const SwitchCameraSettingType(CameraSettingType.frameResize),
                      );
                    },
                    iconPath: CameraIcons.frameResizeIcon,
                    isSelected:
                        cameraSettingType == CameraSettingType.frameResize,
                  ),
                  BottomBarButton(
                    onPressed: () {
                      context.read<CameraBloc>().add(
                        const SwitchCameraSettingType(CameraSettingType.hd),
                      );
                    },
                    iconPath: CameraIcons.hdIcon,
                    isSelected: cameraSettingType == CameraSettingType.hd,
                  ),
                  BottomBarButton(
                    onPressed: () {
                      context.read<CameraBloc>().add(
                        const SwitchCameraSettingType(CameraSettingType.grid),
                      );
                    },
                    iconPath: CameraIcons.gridIcon,
                    isSelected: cameraSettingType == CameraSettingType.grid,
                  ),
                  BottomBarButton(
                    onPressed: () {
                      context.read<CameraBloc>().add(
                        const SwitchCameraSettingType(CameraSettingType.timer),
                      );
                    },
                    iconPath: CameraIcons.timerIcon,
                    isSelected: cameraSettingType == CameraSettingType.timer,
                  ),
                  BottomBarButton(
                    onPressed: () {
                      context.read<CameraBloc>().add(
                        const SwitchCameraSettingType(CameraSettingType.shutterSound),
                      );
                    },
                    iconPath: CameraIcons.soundIcon,
                    isSelected:
                        cameraSettingType == CameraSettingType.shutterSound,
                  ),
                ],
              ),
            ],
          );
        },
      ),
    );
  }

  Widget _buildSettingWidget(BuildContext context, CameraSettingType type) {
    switch (type) {
      case CameraSettingType.frameResize:
        return const FrameResizeSettings();
      case CameraSettingType.hd:
        return const HdSettings();
      case CameraSettingType.grid:
        return const GridSettings();
      case CameraSettingType.timer:
        return const TimerSettings();
      case CameraSettingType.shutterSound:
        return const SoundSettings();
      default:
        return const SizedBox.shrink();
    }
  }
}
