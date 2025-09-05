import 'package:flutter/material.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';

class NotificationSound extends StatelessWidget {
  const NotificationSound({super.key});

  void _backNavigation(BuildContext context) {
    Navigator.pushNamed(context, AppRoutes.notificationSound);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: CustomAppBar(
        title: "Bluetooth",
        leftIcon: Image.asset(Images.back),
        leftIconOnTap: () => _backNavigation(context),
      ),
      // body: ContainerWidget(
      //     child: MechanixSelect(
      //         options: options,
      //         onChanged: onChanged,
      //         selectValue: selectValue)),
    );
  }
}
