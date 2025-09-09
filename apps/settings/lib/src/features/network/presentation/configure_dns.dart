import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';
import 'package:widgets/widgets/select/select_type.dart';

class ConfigureDnsWidget extends StatefulWidget {
  const ConfigureDnsWidget({super.key});

  @override
  State<ConfigureDnsWidget> createState() => _ConfigureDnsWidgetState();
}

class _ConfigureDnsWidgetState extends State<ConfigureDnsWidget> {
  void backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  String? selectedValue = '';

  void onChange(SelectOption value) {
    setState(() {
      selectedValue = value.value;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: CustomAppBar(
        title: "Configure DNS",
        leftIcon: Image.asset(Images.back),
        leftIconOnTap: () => backNavigation(context),
      ),
      body: ContainerWidget(
        child: SingleChildScrollView(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              MechanixSelect(
                selectValue: selectedValue,
                onChanged: onChange,
                onTap: () {},
                options: [
                  SelectOption(
                    label: 'Automatic',
                    value: 'AUTOMATIC',
                  ),
                  SelectOption(
                    label: 'Static',
                    value: 'STATIC',
                  ),
                ],
              ),
              MechanixSectionList(
                title: 'Servers',
                sectionListItems: [
                  SectionListItems(title: '196.284.765.1'),
                  SectionListItems(title: '2405:201:2026:18db:c0a8:1d01'),
                ],
              ).padOnly(top: 40, bottom: 8)
            ],
          ),
        ),
      ).padHorizontal(16),
    );
  }
}
