import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/features/network/models/types.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/section_list/section_list_items_type.dart';
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
      appBar: const MechanixNavigationBar(title: "Configure DNS"),
      body: ContainerWidget(
        child: SingleChildScrollView(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              MechanixSelect(
                value: selectedValue,
                onChanged: onChange,
                options: dnsOptions,
              ),
              MechanixSectionList(
                title: 'Servers',
                sectionListItems: [
                  SectionListItems(title: '196.284.765.1'),
                  SectionListItems(title: '2405:201:2026:18db:c0a8:1d01'),
                ],
              ).padOnly(top: 40, bottom: 8)
            ],
          ).padTop(8),
        ),
      ),
    );
  }
}
