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
    switch (deviceType) {
      case 'audio-headphones':
        return Row(
          children: [
            IconWidget(iconPath: Images.audioHeadset).padRight(8),
            CustomTrailingText(
              title: 'Headphone',
            ).padRight(8),
            IconWidget(
              iconWidth: 10,
              iconHeight: 17,
              iconPath: Images.rightIconArrow,
            ).padRight(8)
          ],
        );

      case 'audio-headset':
        return Row(
          children: [
            IconWidget(iconPath: Images.audioHeadset).padRight(8),
            CustomTrailingText(
              title: 'Headphone',
            ).padRight(8),
            IconWidget(
              iconWidth: 10,
              iconHeight: 17,
              iconPath: Images.rightIconArrow,
            ).padRight(8)
          ],
        );

      case 'phone':
        return Row(
          children: [
            IconWidget(iconPath: Images.mobile).padRight(8),
            CustomTrailingText(
              title: 'Mobile',
            ).padRight(8),
            IconWidget(
              iconWidth: 10,
              iconHeight: 17,
              iconPath: Images.rightIconArrow,
            ).padRight(8)
          ],
        );

      case 'computer':
        return Row(
          children: [
            IconWidget(iconPath: Images.tv).padRight(8),
            CustomTrailingText(
              title: 'TV',
            ).padRight(8),
            IconWidget(
              iconWidth: 10,
              iconHeight: 17,
              iconPath: Images.rightIconArrow,
            ).padRight(8)
          ],
        );

      case 'multimedia-player':
        return Row(
          children: [
            IconWidget(iconPath: Images.speaker).padRight(8),
            CustomTrailingText(
              title: 'Car',
            ).padRight(8),
            IconWidget(
              iconWidth: 10,
              iconHeight: 17,
              iconPath: Images.rightIconArrow,
            ).padRight(8)
          ],
        );

      default:
        return Row(
          children: [
            CustomTrailingText(
              title: 'Other',
            ).padRight(8),
            IconWidget(
              iconWidth: 10,
              iconHeight: 17,
              iconPath: Images.rightIconArrow,
            ).padRight(8)
          ],
        );
    }
  }
}
