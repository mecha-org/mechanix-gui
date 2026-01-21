import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:widgets/mechanix.dart';

class RightIconArrowWidget extends StatelessWidget {
  const RightIconArrowWidget({super.key});

  @override
  Widget build(BuildContext context) {
    return IconWidget(
      iconPath: Images.rightIconArrow,
      boxWidth: 20,
      boxHeight: 20,
      iconWidth: 8.13,
      iconHeight: 14.38,
      iconColor: context.onSecondaryFixed,
    );
  }
}
