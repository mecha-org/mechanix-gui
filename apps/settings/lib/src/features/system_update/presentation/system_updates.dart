import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/section_list/section_list_items_type.dart';
import 'package:widgets/widgets/switch/mechanix_switch.dart';

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
      appBar: const MechanixNavigationBar(title: 'System Updates'),
      body: SingleChildScrollView(
        physics: const BouncingScrollPhysics(),
        child: ContainerWidget(
            child: Column(
          children: [
            MechanixSectionList(
              title: '',
              sectionListItems: [
                SectionListItems(
                  title: 'Auto update',
                  defaultTrailingIcon: false,
                  trailing: MechanixSwitch(
                    value: true,
                    inactiveText: 'ON',
                    onChanged: (v) => {},
                  ),
                ),
                SectionListItems(
                  //  TODO : add custom height in dart component
                  titleWidget: SizedBox(
                      width: 476,
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
                  // TODO : add soft wrap property for text in dart component
                  title:
                      'This update introduces two more language/ script support in keyboard, bug fixes and security updates for the MX device.',
                  defaultTrailingIcon: false,
                  titleTextStyle: TextStyle(
                    color: context.onSurface,
                    fontWeight: FontWeight.w400,
                  ),
                ),
                SectionListItems(
                  title:
                      ' For information on the security content of Mecha Systems, check out the documentation on mechasystems.com',
                  defaultTrailingIcon: false,
                  titleTextStyle: TextStyle(
                    color: context.onSurface,
                    fontWeight: FontWeight.w400,
                  ),
                ),
              ],
            ),
          ],
        ).padTop(8)),
      ),
    );
  }
}
