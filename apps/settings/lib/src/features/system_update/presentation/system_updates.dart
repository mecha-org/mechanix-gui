import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/section_list/section_list_items_type.dart';
import 'package:widgets/widgets/switch/mechanix_switch.dart';
import 'package:widgets/widgets/switch/mechanix_switch_theme.dart';

class SystemUpdates extends StatefulWidget {
  const SystemUpdates({super.key});

  @override
  State<SystemUpdates> createState() => _SystemUpdatesState();
}

class _SystemUpdatesState extends State<SystemUpdates> {
  void backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SingleChildScrollView(
        physics: const BouncingScrollPhysics(),
        child: ContainerWidget(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const CustomTitle(title: "System Updates"),
              MechanixSectionList(
                sectionListItems: [
                  SectionListItems(
                    title: 'Auto update',
                    defaultTrailingIcon: false,
                    trailing: MechanixSwitch(
                      value: true,
                      style: MechanixSwitchStyle(
                        activeTrackColor: context.secondaryContainer,
                        inactiveTrackColor: context.secondaryContainer,
                      ),
                      inactiveText: 'ON',
                      onChanged: (v) => {},
                    ),
                  ),
                  SectionListItems(
                    height: 128,
                    titleWidget: SizedBox(
                        width: 476,
                        height: 128,
                        child: ClipRRect(
                          borderRadius: BorderRadius.circular(8),
                          child: Image.asset(
                            Images.blogImage,
                            fit: BoxFit.fitHeight,
                          ),
                        )),
                    defaultTrailingIcon: false,
                  ),
                  SectionListItems(
                      title: 'Mecha V1.1',
                      titleTextStyle: TextStyle(
                        color: context.onSurface,
                        fontWeight: FontWeight.w600,
                      ),
                      trailing: const CustomTrailingText(title: "800 MB"),
                      defaultTrailingIcon: false),
                  SectionListItems(
                    titleWidget: SizedBox(
                      width: 476,
                      child: Text(
                        'This update introduces two more language/ script support in keyboard, bug fixes and security updates for the MX device.',
                        softWrap: true,
                        style: context.textTheme.labelSmall
                            ?.copyWith(color: context.onSecondaryFixed),
                      ),
                    ),
                    defaultTrailingIcon: false,
                  ),
                  SectionListItems(
                    titleWidget: SizedBox(
                      width: 476,
                      child: Text(
                        ' For information on the security content of Mecha Systems, check out the documentation on mechasystems.com',
                        softWrap: true,
                        style: context.textTheme.labelSmall
                            ?.copyWith(color: context.onSecondaryFixed),
                      ),
                    ),
                    defaultTrailingIcon: false,
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
      bottomNavigationBar: MechanixBottomBar(
        leadingWidget: [context.backButton],
      ),
    );
  }
}
