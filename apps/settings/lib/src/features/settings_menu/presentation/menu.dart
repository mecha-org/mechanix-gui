import 'package:flutter/material.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/section_list/section_list_items_type.dart';

class SettingMenu extends StatefulWidget {
  const SettingMenu({super.key, required this.openPath});
  final String openPath;

  @override
  State<SettingMenu> createState() => _SettingMenuState();
}

class _SettingMenuState extends State<SettingMenu> {
  @override
  void initState() {
    super.initState();

    WidgetsBinding.instance.addPostFrameCallback((_) {
      print("Menu openpath ${widget.openPath}");

      switch (widget.openPath) {
        case AppRoutes.wireless:
          Navigator.pushNamed(context, AppRoutes.wireless);
          break;
        case AppRoutes.bluetooth:
          Navigator.pushNamed(context, AppRoutes.bluetooth);
          break;
        case AppRoutes.battery:
          Navigator.pushNamed(context, AppRoutes.battery);
          break;
        case AppRoutes.display:
          Navigator.pushNamed(context, AppRoutes.display);
          break;
        case AppRoutes.sound:
          Navigator.pushNamed(context, AppRoutes.sound);
          break;
      }
    });
  }

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
                      titleTextStyle: const TextStyle(
                          fontSize: 24, fontWeight: FontWeight.bold),
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
                      onTap: () => onTap(context, AppRoutes.appearance),
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
                      iconPath: Images.launcherIcon,
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
    );
  }
}
