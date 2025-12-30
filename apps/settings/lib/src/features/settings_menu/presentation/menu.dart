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
                  title: 'Connectivity',
                  sectionListItems: [
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'Network',
                      onTap: () => onTap(context, AppRoutes.wireless),
                      iconPath: Images.wifi,
                    ),
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'Bluetooth',
                      onTap: () => onTap(context, AppRoutes.bluetooth),
                      iconPath: Images.bluetooth,
                    ),
                  ]).padVertical(8),
              MechanixSectionList(
                  physics: const NeverScrollableScrollPhysics(),
                  title: 'Device',
                  sectionListItems: [
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'Battery',
                      onTap: () => onTap(context, AppRoutes.battery),
                      iconPath: Images.battery,
                    ),
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'Date & Time',
                      onTap: () => onTap(context, AppRoutes.dateTime),
                      iconPath: Images.dateTime,
                    ),
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'Display',
                      onTap: () => onTap(context, AppRoutes.display),
                      iconPath: Images.display,
                    ),
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'Sound & Haptics',
                      onTap: () => onTap(context, AppRoutes.sound),
                      iconPath: Images.sound,
                    ),
                  ]),
              MechanixSectionList(
                  physics: const NeverScrollableScrollPhysics(),
                  title: 'System',
                  sectionListItems: [
                    SectionListItems.leadingIcon(
                      iconColor: context.primary,
                      title: 'About',
                      onTap: () => onTap(context, AppRoutes.about),
                      iconPath: Images.cometIcon,
                    ),
                  ]),
            ],
          ).padTop(8),
        ),
      ),
    );
  }
}
