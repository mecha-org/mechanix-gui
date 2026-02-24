import 'package:flutter/material.dart';
import 'package:mechanix_camera/utils/camera_colors.dart';
import 'package:widgets/extensions/color.dart';

class VideoTimer extends StatelessWidget {
  const VideoTimer({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(8),
        color: CameraColors.redColor,
      ),
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      child: Text(
        "00:00",
        style: TextStyle(color: context.onSecondaryFixedVariant),
      ),
    );
  }
}
