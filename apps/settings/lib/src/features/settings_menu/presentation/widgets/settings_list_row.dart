import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_icon.dart';
import 'package:mechanix_settings/src/commons/styles/custom_styles.dart';

class SettingsMenuListRow extends StatelessWidget {
  final Widget icon;
  final String title;
  final String trailingText;
  final VoidCallback onTap;
  final bool isBreak;
  const SettingsMenuListRow({
    super.key,
    required this.icon,
    required this.title,
    required this.trailingText,
    required this.onTap,
    this.isBreak = false,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Container(
          margin: const EdgeInsets.symmetric(vertical: 3),
          decoration: rowBoxDecoration,
          child: ListTile(
            contentPadding: const EdgeInsets.symmetric(horizontal: 16),
            leading: CustomIcon(
              icon: icon,
            ),
            title: Text(
              title,
              style: baseHeaderStyle.copyWith(fontSize: 18),
            ),
            trailing: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                Text(
                  trailingText,
                  style: baseHeaderStyle.copyWith(fontSize: 18),
                ),
                CustomIcon(
                  icon: Image.asset(Images.rightIconArrow),
                  height: 16,
                  width: 16,
                ),
              ],
            ),
            onTap: onTap,
          ),
        ),
        if (isBreak) const SizedBox(height: 15),
      ],
    );
  }
}
