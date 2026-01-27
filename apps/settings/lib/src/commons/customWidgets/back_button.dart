import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';

extension BackButtonExtension on BuildContext {
  void _onBackIcon(BuildContext context) {
    Navigator.pop(context);
  }

  BottomBarButton get backButton => BottomBarButton.widget(
        widget: IconButton(
          onPressed: () => _onBackIcon(this),
          icon: const IconWidget(
            iconPath: Images.back,
            boxWidth: 48,
            boxHeight: 48,
            iconWidth: 12.13,
            iconHeight: 20.38,
          ),
        ).padBottom(12).padBottom(25),
      );
}
