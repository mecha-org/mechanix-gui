import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_event.dart';
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
      child: MechanixBottomBar(
        centerWidgetSpacing: 16,
        theme: MechanixBottomBarThemeData(
          iconTheme: MechanixBottomBarIconThemeData(
            iconSize: const Size(28, 28),
            iconBoxSize: const Size(44, 44),
          ),
          decoration: BoxDecoration(
            color: Theme.of(context).colorScheme.secondaryContainer,
            borderRadius: const BorderRadius.only(
              topLeft: Radius.circular(8),
              topRight: Radius.circular(8),
            ),
          ),
        ),
        leadingWidget: [
          BottomBarButton(
            iconTheme: MechanixBottomBarIconThemeData(
              buttonMargin: const EdgeInsets.only(left: 10),
              iconSize: const Size(28, 28),
              iconBoxSize: const Size(44, 44),
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
              // Handle frame resize
            },
            iconPath: CameraIcons.frameResizeIcon,
          ),
          BottomBarButton(
            onPressed: () {
              // Handle HD toggle
            },
            iconPath: CameraIcons.hdIcon,
          ),
          BottomBarButton(
            onPressed: () {
              // Handle grid toggle
            },
            iconPath: CameraIcons.gridIcon,
          ),
          BottomBarButton(
            onPressed: () {
              // Handle timer
            },
            iconPath: CameraIcons.timerIcon,
          ),
          BottomBarButton(
            onPressed: () {
              // Handle audio toggle
            },
            iconPath: CameraIcons.audioIcon,
          ),
        ],
      ),
    );
  }
}
