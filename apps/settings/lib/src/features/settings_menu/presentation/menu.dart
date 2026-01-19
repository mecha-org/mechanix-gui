import 'package:flutter/material.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/section_list/section_list_items_type.dart';

class SettingMenu extends StatelessWidget {
  const SettingMenu({super.key});

  void onTap(BuildContext context, String route) {
    Navigator.pushNamed(context, route);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SingleChildScrollView(
        physics: const BouncingScrollPhysics(),
        child: ContainerWidget(
          child: Column(
            children: [
              const CustomTitle(title: 'Settings', fontSize: 32),
              MechanixSectionList(
                  physics: const NeverScrollableScrollPhysics(),
                  sectionListItems: [
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'Network',
                      onTap: () => onTap(context, AppRoutes.wireless),
                      iconPath: Images.wifi,
                    ),
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'Cellular',
                      onTap: () {},
                      iconPath: Images.cellularIcon,
                    ),
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'Airplane Mode',
                      onTap: () {},
                      iconPath: Images.wifi,
                    ),
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'Bluetooth',
                      onTap: () => onTap(context, AppRoutes.bluetooth),
                      iconPath: Images.bluetooth,
                    ),
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'Battery',
                      onTap: () => onTap(context, AppRoutes.battery),
                      iconPath: Images.battery,
                    ),
                  ]),
              MechanixSectionList(
                  physics: const NeverScrollableScrollPhysics(),
                  sectionListItems: [
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'Extensions',
                      onTap: () {},
                      iconPath: Images.extensionIcon,
                    ),
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'Display & Brightness',
                      onTap: () => onTap(context, AppRoutes.display),
                      iconPath: Images.display,
                    ),
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'Appearance',
                      onTap: () {},
                      iconPath: Images.appearance,
                    ),
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'Sound & Haptics',
                      onTap: () => onTap(context, AppRoutes.sound),
                      iconPath: Images.sound,
                    ),
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'Notification',
                      onTap: () {},
                      iconPath: Images.notificationIcon,
                    ),
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'Language',
                      onTap: () {},
                      iconPath: Images.languageIcon,
                    ),
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'Date & Time',
                      onTap: () => onTap(context, AppRoutes.dateTime),
                      iconPath: Images.dateTime,
                    ),
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'Launcher',
                      onTap: () {},
                      iconPath: Images.languageIcon,
                    ),
                  ]),
              MechanixSectionList(
                  physics: const NeverScrollableScrollPhysics(),
                  sectionListItems: [
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'All Apps',
                      onTap: () {},
                      iconPath: Images.allAppsIcon,
                    ),
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'Security',
                      onTap: () {},
                      iconPath: Images.searchIcon,
                    ),
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'System Update',
                      onTap: () => onTap(context, AppRoutes.systemUpdates),
                      iconPath: Images.updateIcon,
                    ),
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'System Info',
                      onTap: () => onTap(context, AppRoutes.about),
                      iconPath: Images.cometIcon,
                    ),
                  ]),
            ],
          ).padOnly(top: 8, bottom: 0),
        ),
      ),
      floatingActionButton: IconButton.filled(
        onPressed: () {},
        constraints: const BoxConstraints(
          maxHeight: 44,
          maxWidth: 44,
        ),
        style: ButtonStyle(
          backgroundColor: WidgetStateProperty.all(context.surfaceContainer),
        ),
        icon: const IconWidget(
          iconPath: Images.searchIcon,
          boxWidth: 24,
          boxHeight: 24,
          iconWidth: 17,
          iconHeight: 17,
        ),
      ),
    );
  }
}
