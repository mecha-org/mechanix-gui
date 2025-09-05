import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/features/settings_menu/models/types.dart';
import 'package:mechanix_settings/src/features/settings_menu/presentation/widgets/settings_list_row.dart';
import 'package:mechanix_settings/app_route.dart';

class SettingMenu extends StatelessWidget {
  SettingMenu({super.key});

  final List<SettingsMenuItem> settingsMenuItems = [
    SettingsMenuItem(
      title: 'Wi-Fi',
      value: null,
      icon: Images.wifi,
      route: AppRoutes.wireless,
    ),
    SettingsMenuItem(
      title: 'Bluetooth',
      value: null,
      icon: Images.bluetooth,
      route: AppRoutes.bluetooth,
    ),
    SettingsMenuItem(
      title: 'Network',
      value: null,
      icon: Images.networkIcon,
      route: AppRoutes.bluetooth,
    ),
    SettingsMenuItem(
      title: 'Battery',
      value: null,
      icon: Images.battery,
      route: AppRoutes.battery,
      isBreak: true,
    ),
    SettingsMenuItem(
      title: 'General',
      value: null,
      icon: Images.settings,
      route: AppRoutes.battery,
    ),
    SettingsMenuItem(
      title: 'Date & Time',
      value: null,
      icon: Images.dateTime,
      route: AppRoutes.dateTime,
    ),
    SettingsMenuItem(
      title: 'Language',
      value: null,
      icon: Images.languageIcon,
      route: AppRoutes.dateTime,
      isBreak: true,
    ),
    SettingsMenuItem(
      title: 'Camera',
      value: null,
      icon: Images.dateTime,
      route: AppRoutes.dateTime,
    ),
    SettingsMenuItem(
      title: 'Display',
      value: null,
      icon: Images.display,
      route: AppRoutes.display,
    ),
    SettingsMenuItem(
        title: 'Appearance',
        value: null,
        icon: Images.appearance,
        route: AppRoutes.appearance,
        isBreak: true),
    SettingsMenuItem(
      title: 'Notifications',
      value: null,
      icon: Images.notificationIcon,
      route: AppRoutes.sound,
    ),

    SettingsMenuItem(
        title: 'Sounds',
        value: null,
        icon: Images.sound,
        route: AppRoutes.sound,
        isBreak: true),

    SettingsMenuItem(
      title: 'Apps',
      value: null,
      icon: Images.about,
      route: AppRoutes.about,
    ),
    // SettingsMenuItem(
    //   title: 'About',
    //   value: null,
    //   icon: Images.about,
    //   route: AppRoutes.about,
    // ),
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: CustomAppBar(
        title: "Settings",
      ),
      body: ContainerWidget(
        child: ListView(
          children: settingsMenuItems.map((menuItem) {
            return SettingsMenuListRow(
              icon: Image.asset(menuItem.icon),
              title: menuItem.title,
              trailingText: menuItem.value ?? '',
              isBreak: menuItem.isBreak,
              onTap: () {
                if (menuItem.route != '') {
                  Navigator.pushNamed(context, menuItem.route!);
                }
              },
            );
          }).toList(),
        ),
      ),
    );
  }
}
