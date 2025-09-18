import 'package:flutter/material.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';

class SettingMenu extends StatelessWidget {
  const SettingMenu({super.key});

  void onTap(BuildContext context, String route) {
    Navigator.pushNamed(context, route);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: MechanixNavigationBar(
        title: "Settings",
        titleSpacing: 16,
      ),
      body: SingleChildScrollView(
        child: ContainerWidget(
          child: Column(
            children: [
              // UpdateCard(),
              MechanixSectionList(title: 'Connectivity', sectionListItems: [
                SectionListItems(
                  title: 'Network',
                  onTap: () => onTap(context, AppRoutes.wireless),
                  leading: IconWidget(
                    iconWidth: 22,
                    iconHeight: 18,
                    iconPath: Images.wifi,
                  ),
                ),
                SectionListItems(
                  title: 'Bluetooth',
                  onTap: () => onTap(context, AppRoutes.bluetooth),
                  leading: IconWidget(
                    iconHeight: 20,
                    iconWidth: 20,
                    iconPath: Images.bluetooth,
                  ),
                ),
              ]).padVertical(8),
              MechanixSectionList(title: 'Device', sectionListItems: [
                SectionListItems(
                  title: 'Battery',
                  onTap: () => onTap(context, AppRoutes.battery),
                  leading: IconWidget(
                    iconWidth: 23,
                    iconHeight: 13,
                    iconPath: Images.battery,
                  ),
                ),
                SectionListItems(
                  title: 'Date & Time',
                  onTap: () => onTap(context, AppRoutes.dateTime),
                  leading: IconWidget(
                    iconWidth: 20,
                    iconHeight: 20,
                    iconPath: Images.dateTime,
                  ),
                ),
                SectionListItems(
                  title: 'Display',
                  onTap: () => onTap(context, AppRoutes.display),
                  leading: IconWidget(
                    iconWidth: 20,
                    iconHeight: 20,
                    iconPath: Images.display,
                  ),
                ),
                SectionListItems(
                  title: 'Sound & Haptics',
                  onTap: () => onTap(context, AppRoutes.sound),
                  leading: IconWidget(
                    iconWidth: 18,
                    iconHeight: 20,
                    iconPath: Images.sound,
                  ),
                ),
              ]),
              MechanixSectionList(title: 'System', sectionListItems: [
                SectionListItems(
                  title: 'About',
                  onTap: () => onTap(context, AppRoutes.about),
                  leading: IconWidget(
                    iconPath: Images.cometIcon,
                  ),
                ),
              ]),
            ],
          ).padTop(8),
        ),
      ),
    );
  }
}
