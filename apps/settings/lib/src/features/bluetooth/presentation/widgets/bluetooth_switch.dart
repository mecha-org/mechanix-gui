import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_toggle.dart';

class BluetoothSwitch extends StatelessWidget {
  final String title;
  final String? subtitle;
  final bool value;
  final ValueChanged<bool> onChanged;

  const BluetoothSwitch({
    super.key,
    required this.title,
    this.subtitle,
    required this.value,
    required this.onChanged,
  });

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Expanded(
          child: ListTile(
            contentPadding: EdgeInsets.zero,
            title: Text(
              title,
              style: const TextStyle(fontSize: 24),
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

