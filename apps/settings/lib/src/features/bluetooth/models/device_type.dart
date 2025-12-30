// "audio-card"
// "audio-headphones"
// "audio-headset"
// "camera-photo"
// "camera-video"
// "computer"
// "input-gaming"
// "input-keyboard"
// "input-mouse"
// "input-tablet"
// "modem"
// "multimedia-player"
// "network-wireless"
// "phone"
// "printer"
// "scanner"
// "unknown"
// "video-display"

import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:widgets/mechanix.dart';

class DeviceType extends StatelessWidget {
  const DeviceType({super.key, this.deviceType});

  final String? deviceType;

  @override
  Widget build(BuildContext context) {
    final icon = IconWidget(
      iconPath: Images.rightIconArrow,
      iconWidth: 9,
      iconHeight: 15,
      boxWidth: 20,
      boxHeight: 20,
      iconColor: context.onSurfaceVariant,
    );

    switch (deviceType) {
      case 'audio-headphones':
        return Row(
          children: [
            const CustomTrailingText(title: 'Headphone').padRight(8),
            icon
          ],
        );

      case 'audio-headset':
        return Row(
          children: [
            const CustomTrailingText(title: 'Headphone').padRight(8),
            icon
          ],
        );

      case 'phone':
        return Row(
          children: [
            const CustomTrailingText(title: 'Mobile').padRight(8),
            icon
          ],
        );

      case 'computer':
        return Row(
          children: [const CustomTrailingText(title: 'TV').padRight(8), icon],
        );

      case 'multimedia-player':
        return Row(
          children: [const CustomTrailingText(title: 'Car').padRight(8), icon],
        );

      // TODO: add more device types like mouse , keyboard
      default:
        return Row(
          children: [
            const CustomTrailingText(title: 'Other').padRight(8),
            icon
          ],
        );
    }
  }
}
