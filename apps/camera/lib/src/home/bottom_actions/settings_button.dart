import 'package:flutter/material.dart';
import 'package:mechanix_camera/src/home/bottom_actions/mode_icon_button.dart';
import 'package:mechanix_camera/utils/icons/icon.dart';
import 'package:widgets/extensions/color.dart';


/// Settings button for opening camera settings panel
class SettingsButton extends StatelessWidget {
  final VoidCallback onTap;

  const SettingsButton({
    super.key,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      cursor: SystemMouseCursors.click,
      child: GestureDetector(
        behavior: HitTestBehavior.translucent,
        onTap: onTap,
        child: Container(
          padding: const EdgeInsets.all(2),
          decoration: BoxDecoration(
            color: context.secondaryContainer.withValues(alpha: 0.5),
            borderRadius: BorderRadius.circular(8),
          ),
          child: ModeIconButton(
            iconPath: CameraIcons.settingsIcon,
          ),
        ),
      ),
    );
  }
}