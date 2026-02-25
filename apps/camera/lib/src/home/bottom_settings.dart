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

class BottomSettings extends StatefulWidget {
  const BottomSettings({super.key});

  @override
  State<BottomSettings> createState() => _BottomSettingsState();
}

class _BottomSettingsState extends State<BottomSettings>
    with SingleTickerProviderStateMixin {
  late final AnimationController _animationController;
  late final Animation<Offset> _slideAnimation;
  late final Animation<double> _fadeAnimation;

  CameraSettingType _visibleSettingType = CameraSettingType.none;
  CameraSettingType _previousSettingType = CameraSettingType.none;

  @override
  void initState() {
    super.initState();
    _animationController = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 280),
    );

    _slideAnimation = Tween<Offset>(
      begin: const Offset(0, 1),
      end: Offset.zero,
    ).animate(
      CurvedAnimation(
        parent: _animationController,
        curve: Curves.easeOutCubic,
        reverseCurve: Curves.easeInCubic,
      ),
    );

    _fadeAnimation = Tween<double>(begin: 0.0, end: 1.0).animate(
      CurvedAnimation(parent: _animationController, curve: Curves.easeOut),
    );

    // Once the reverse animation finishes, clear the visible widget.
    _animationController.addStatusListener((status) {
      if (status == AnimationStatus.dismissed) {
        setState(() {
          _visibleSettingType = CameraSettingType.none;
        });
      }
    });
  }

  @override
  void dispose() {
    _animationController.dispose();
    super.dispose();
  }

  void _handleSettingTypeChange(CameraSettingType newType) {
    if (newType == _previousSettingType) return;

    _previousSettingType = newType;

    if (newType != CameraSettingType.none) {
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (!mounted) return;
        setState(() => _visibleSettingType = newType);
        if (_animationController.status == AnimationStatus.dismissed ||
            _animationController.status == AnimationStatus.reverse) {
          _animationController.forward();
        }
      });
    } else {
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (!mounted) return;
        _animationController.reverse();
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return Positioned(
      left: 0,
      right: 0,
      bottom: 0,
      child: BlocSelector<CameraBloc, CameraState, CameraSettingType>(
        selector: (state) => state.cameraSettingType,
        builder: (context, cameraSettingType) {
          _handleSettingTypeChange(cameraSettingType);

          return Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              if (_visibleSettingType != CameraSettingType.none)
                SlideTransition(
                  position: _slideAnimation,
                  child: FadeTransition(
                    opacity: _fadeAnimation,
                    child: _buildSettingWidget(context, _visibleSettingType),
                  ),
                ),

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
                        const SwitchCameraSettingType(
                          CameraSettingType.frameResize,
                        ),
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
                        const SwitchCameraSettingType(
                          CameraSettingType.shutterSound,
                        ),
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
