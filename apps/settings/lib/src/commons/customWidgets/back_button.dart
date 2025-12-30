import 'package:flutter/widgets.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottom_bar/mechanix_bottom_bar_theme.dart';

extension BackButtonExtension on BuildContext {
  void _onBackIcon(BuildContext context) {
    Navigator.pop(context);
  }

  BottomBarButton get backButton => BottomBarButton(
        onPressed: () => _onBackIcon(this),
        iconTheme: const MechanixBottomBarIconThemeData(
          buttonSize: Size(44, 44),
          iconBoxSize: Size(28, 28),
          iconSize: Size(11.38, 20.13),
          buttonMargin: EdgeInsets.only(left: 12),
        ),
        iconPath: Images.back,
      );
}
