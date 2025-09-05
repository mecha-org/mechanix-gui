import 'package:flutter/material.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';

class SettingMenu extends StatelessWidget {
  SettingMenu({super.key});

  final trailingIcon = IconWidget(
    iconWidth: 10,
    iconHeight: 17,
    iconPath: Images.rightIconArrow,
  );

  void onTap(BuildContext context, String route) {
    Navigator.pushNamed(context, route);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: CustomAppBar(
        title: "Settings",
      ),
      body: SingleChildScrollView(
        child: ContainerWidget(
          child: Column(
            children: [
              MechanixSectionList(title: 'Connectivity', sectionListItems: [
                SectionListItems(
                  title: 'Network',
                  onTap: () => onTap(context, AppRoutes.wireless),
                  leading: IconWidget(
                    iconWidth: 22,
                    iconHeight: 18,
                    iconPath: Images.wifi,
                  ),
                  trailing: trailingIcon,
                ),
                SectionListItems(
                  title: 'Mobile Data',
                  onTap: () => onTap(context, AppRoutes.bluetooth),
                  leading: IconWidget(
                    iconWidth: 18,
                    iconHeight: 18,
                    iconPath: Images.networkIcon,
                  ),
                  trailing: trailingIcon,
                ),
                SectionListItems(
                  title: 'Bluetooth',
                  onTap: () => onTap(context, AppRoutes.bluetooth),
                  leading: IconWidget(
                    iconHeight: 20,
                    iconWidth: 20,
                    iconPath: Images.bluetooth,
                  ),
                  trailing: trailingIcon,
                ),
              ]),
              MechanixSectionList(title: 'Device', sectionListItems: [
                SectionListItems(
                  title: 'Battery',
                  onTap: () => onTap(context, AppRoutes.battery),
                  leading: IconWidget(
                    iconWidth: 23,
                    iconHeight: 13,
                    iconPath: Images.battery,
                  ),
                  trailing: trailingIcon,
                ),
                SectionListItems(
                  title: 'Date & Time',
                  onTap: () => onTap(context, AppRoutes.dateTime),
                  leading: IconWidget(
                    iconWidth: 20,
                    iconHeight: 20,
                    iconPath: Images.dateTime,
                  ),
                  trailing: trailingIcon,
                ),
                SectionListItems(
                  title: 'Language',
                  onTap: () => onTap(context, AppRoutes.display),
                  leading: IconWidget(
                    iconWidth: 18,
                    iconHeight: 18,
                    iconPath: Images.languageIcon,
                  ),
                  trailing: trailingIcon,
                ),
                SectionListItems(
                  title: 'Camera',
                  onTap: () => onTap(context, AppRoutes.display),
                  leading: IconWidget(
                    iconWidth: 20,
                    iconHeight: 18,
                    iconPath: Images.cameraIcon,
                  ),
                  trailing: trailingIcon,
                ),
                SectionListItems(
                  title: 'Display',
                  onTap: () => onTap(context, AppRoutes.display),
                  leading: IconWidget(
                    iconWidth: 20,
                    iconHeight: 20,
                    iconPath: Images.display,
                  ),
                  trailing: trailingIcon,
                ),
                SectionListItems(
                  title: 'Appearance',
                  onTap: () => onTap(context, AppRoutes.appearance),
                  leading: IconWidget(
                    iconWidth: 19,
                    iconHeight: 19,
                    iconPath: Images.appearance,
                  ),
                  trailing: trailingIcon,
                ),
                SectionListItems(
                  title: 'Sound & Haptics',
                  onTap: () => onTap(context, AppRoutes.sound),
                  leading: IconWidget(
                    iconWidth: 18,
                    iconHeight: 20,
                    iconPath: Images.sound,
                  ),
                  trailing: trailingIcon,
                ),
              ]),
              MechanixSectionList(title: 'Application', sectionListItems: [
                SectionListItems(
                  title: 'Notifications',
                  onTap: () => onTap(context, AppRoutes.about),
                  leading: IconWidget(
                    iconWidth: 19,
                    iconHeight: 20,
                    iconPath: Images.notificationIcon,
                  ),
                  trailing: trailingIcon,
                ),
                SectionListItems(
                  title: 'All Apps',
                  onTap: () => onTap(context, AppRoutes.about),
                  leading: IconWidget(
                    iconWidth: 18,
                    iconHeight: 18,
                    iconPath: Images.appsIcon,
                  ),
                  trailing: trailingIcon,
                ),
              ]),
            ],
          ),
        ),
      ),
    );
  }
}
