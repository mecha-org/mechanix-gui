import 'package:flutter/material.dart';
import 'package:mechanix_camera/utils/styles/custom_text_styles.dart';
import 'package:widgets/mechanix.dart';

class SettingsButton extends StatelessWidget {
  final VoidCallback onTap;
  final String text;
  final bool isActive;
  const SettingsButton({
    super.key,
    required this.onTap,
    required this.text,
    this.isActive = false,
  });

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onTap,
      child: Container(
        alignment: Alignment.center,
        child: Text(
          text,
          style: settingsTextStyle(context, isActive: isActive),
        ).padOnly(left: 10, right: 10, bottom: 4),
      ),
    );
  }
}
