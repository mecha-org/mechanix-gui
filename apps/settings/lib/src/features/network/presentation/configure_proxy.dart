import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/select/select_type.dart';

class ConfigureProxyWidget extends StatefulWidget {
  const ConfigureProxyWidget({super.key});

  @override
  State<ConfigureProxyWidget> createState() => _ConfigureProxyWidgetState();
}

class _ConfigureProxyWidgetState extends State<ConfigureProxyWidget> {
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
                    label: 'Off',
                    value: 'OFF',
                  ),
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
            ],
          ),
        ),
      ).padHorizontal(16),
    );
  }
}
