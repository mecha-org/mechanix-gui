import 'package:flutter/material.dart';
import 'package:mechanix_camera/utils/icons/icon.dart';
import 'package:widgets/extensions/color.dart';
import 'package:widgets/widgets/icon_widget.dart';

class BottomActions extends StatefulWidget {
  const BottomActions({super.key});

  @override
  State<BottomActions> createState() => _BottomActionsState();
}

class _BottomActionsState extends State<BottomActions> {
  @override
  Widget build(BuildContext context) {
    return Positioned(
      bottom: 20,
      left: 0,
      right: 0,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 30),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          mainAxisSize: MainAxisSize.max,
          children: [
            Container(
              width: 50,
              height: 50,
              decoration: BoxDecoration(borderRadius: BorderRadius.circular(8)),
              child: ClipRRect(
                borderRadius: BorderRadius.circular(6),
                child: Image.asset(
                  CameraIcons.demoImage,
                  fit: BoxFit.cover,
                  errorBuilder: (context, error, stackTrace) {
                    return Container(
                      color: Colors.orange[300],
                      child: const Icon(
                        Icons.image,
                        color: Colors.white,
                        size: 24,
                      ),
                    );
                  },
                ),
              ),
            ),

            Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                Container(
                  padding: const EdgeInsets.symmetric(
                    horizontal: 8,
                    vertical: 6,
                  ),
                  decoration: BoxDecoration(
                    color: context.secondaryContainer.withValues(alpha: 0.5),
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      // Video Button
                      _buildTopIconButton(CameraIcons.videoIcon, () {}),
                      const SizedBox(width: 8),

                      // Camera Button
                      _buildTopIconButton(CameraIcons.cameraIcon, () {}),
                    ],
                  ),
                ),

                const SizedBox(width: 12),
                //Settings Button
                Container(
                  padding: const EdgeInsets.symmetric(
                    horizontal: 6,
                    vertical: 6,
                  ),
                  decoration: BoxDecoration(
                    color: context.secondaryContainer.withValues(alpha: 0.5),
                    borderRadius: BorderRadius.circular(8),
                  ),
                  child: _buildTopIconButton(CameraIcons.settingsIcon, () {}),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildTopIconButton(String iconPath, VoidCallback onTap) {
    return GestureDetector(
      onTap: onTap,
      child: IconWidget(iconPath: iconPath, boxWidth: 32, boxHeight: 32),
    );
  }
}
