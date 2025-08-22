import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/styles/custom_styles.dart';

class BluetoothListRow extends StatelessWidget {
  final String title;
  final bool isAvailable;
  final bool isConnected;
  final VoidCallback? onDeleteTap;
  final VoidCallback? onDeviceTap;
  final VoidCallback? onSettingsTap;

  const BluetoothListRow({
    super.key,
    required this.title,
    required this.isConnected,
    required this.isAvailable,
    this.onDeleteTap,
    this.onDeviceTap,
    this.onSettingsTap,
  });

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
        cursor: SystemMouseCursors.click,
        child: Container(
          decoration: rowBoxDecoration,
          margin: const EdgeInsets.only(bottom: 10),
          padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 4),
          child: GestureDetector(
            behavior: HitTestBehavior.opaque,
            onTap: onDeviceTap,
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                SizedBox(
                  child: Text(
                    title,
                    style: isAvailable
                        ? baseHeaderStyle
                        : baseHeaderStyle.copyWith(color: Color(0xFF898A8D)),
                  ),
                ),
                Row(
                  children: [
                    if (isConnected)
                      IconButton(
                        padding: EdgeInsets.zero,
                        onPressed: null,
                        icon: Image.asset(
                          Images.bluetoothConnection,
                          width: 20,
                          height: 20,
                        ),
                      ),
                    const SizedBox(width: 10),
                    IconButton(
                      onPressed: onSettingsTap,
                      icon: Image.asset(
                        Images.bluetoothSetting,
                        width: 24,
                        height: 24,
                      ),
                    ),
                  ],
                ),
              ],
            ),
          ),
        ));
  }
}
