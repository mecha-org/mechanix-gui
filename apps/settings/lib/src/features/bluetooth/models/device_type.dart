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
            Text(
              'Headphone',
              style: context.textTheme.labelLarge,
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
            Text(
              'Headphone',
              style: context.textTheme.labelLarge,
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
            Text(
              'Mobile',
              style: context.textTheme.labelLarge,
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
            Text(
              'TV',
              style: context.textTheme.labelLarge,
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
            Text(
              'Car',
              style: context.textTheme.labelLarge,
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
            Text(
              'Other',
              style: TextStyle(color: context.colorScheme.surfaceDim),
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



    // switch (deviceType) {
    //   case 'audio-headphones':
    //     return Row(
    //       children: [
    //         IconWidget(iconPath: Images.audioHeadset).padRight(8),
    //         Text('Headphone').padRight(8),
    //         IconWidget(
    //           iconWidth: 10,
    //           iconHeight: 17,
    //           iconPath: Images.rightIconArrow,
    //         ).padRight(8)
    //       ],
    //     );

    //   case 'audio-headset':
    //     return Row(
    //       children: [
    //         IconWidget(iconPath: Images.audioHeadset).padRight(8),
    //         Text('Headphone').padRight(8),
    //         IconWidget(
    //           iconWidth: 10,
    //           iconHeight: 17,
    //           iconPath: Images.rightIconArrow,
    //         ).padRight(8)
    //       ],
    //     );

    //   case 'phone':
    //     return Row(
    //       children: [
    //         IconWidget(iconPath: Images.mobile).padRight(8),
    //         Text('Mobile').padRight(8),
    //         IconWidget(
    //           iconWidth: 10,
    //           iconHeight: 17,
    //           iconPath: Images.rightIconArrow,
    //         ).padRight(8)
    //       ],
    //     );

    //   case 'computer':
    //     return Row(
    //       children: [
    //         IconWidget(iconPath: Images.tv).padRight(8),
    //         Text('TV').padRight(8),
    //         IconWidget(
    //           iconWidth: 10,
    //           iconHeight: 17,
    //           iconPath: Images.rightIconArrow,
    //         ).padRight(8)
    //       ],
    //     );

    //   case 'multimedia-player':
    //     return Row(
    //       children: [
    //         IconWidget(iconPath: Images.speaker).padRight(8),
    //         Text('Car').padRight(8),
    //         IconWidget(
    //           iconWidth: 10,
    //           iconHeight: 17,
    //           iconPath: Images.rightIconArrow,
    //         ).padRight(8)
    //       ],
    //     );

    //   default:
    //     return null;
    // }