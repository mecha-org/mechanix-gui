import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_toggle.dart';
import 'package:mechanix_settings/src/commons/styles/custom_styles.dart';

class CustomSwitchListTile extends StatelessWidget {
  final String title;
  final String? subtitle;
  final bool value;
  final ValueChanged<bool> onChanged;
  final TextStyle? titleStyle;

  const CustomSwitchListTile(
      {super.key,
      required this.title,
      this.subtitle,
      required this.value,
      required this.onChanged,
      this.titleStyle});

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Expanded(
          child: ListTile(
            contentPadding: EdgeInsets.zero,
            title: Text(
              title,
              style: titleStyle ?? baseHeaderStyle.copyWith(fontSize: 20),
            ),
            subtitle: subtitle != null
                ? Text(
                    subtitle!,
                    style: const TextStyle(fontSize: 16),
                  )
                : null,
          ),
        ),
        CustomToggle(
          value: value,
          onChanged: onChanged,
        ),
      ],
    );
  }
}
