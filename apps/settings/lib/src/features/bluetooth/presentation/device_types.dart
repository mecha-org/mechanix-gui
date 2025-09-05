import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/select/select_type.dart';

class DeviceTypes extends StatefulWidget {
  const DeviceTypes({super.key});

  @override
  State<DeviceTypes> createState() => _DeviceTypesState();
}

class _DeviceTypesState extends State<DeviceTypes> {
  void _backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  String selectValue = '';

  void onChanged(SelectOption option) {
    setState(() {
      selectValue = option.value;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: CustomAppBar(
        title: 'Device types',
        leftIcon: Image.asset(Images.back),
        leftIconOnTap: () => _backNavigation(context),
      ),
      body: SingleChildScrollView(
        child: ContainerWidget(
            child: MechanixSelect(options: [
          SelectOption(
              label: 'Speaker',
              value: 'speaker',
              leading: IconWidget(
                  iconWidth: 16, iconHeight: 20, iconPath: Images.speaker)),
          SelectOption(
              label: 'Headphone',
              value: 'headphone',
              leading: IconWidget(
                  iconWidth: 20,
                  iconHeight: 18,
                  iconPath: Images.audioHeadset)),
          SelectOption(
              label: 'Mobile',
              value: 'mobile',
              leading: IconWidget(
                  iconWidth: 16, iconHeight: 20, iconPath: Images.mobile)),
          SelectOption(
              label: 'TV',
              value: 'tv',
              leading: IconWidget(
                  iconWidth: 20, iconHeight: 20, iconPath: Images.tv)),
          SelectOption(
              label: 'Car',
              value: 'car',
              leading: IconWidget(
                  iconWidth: 24, iconHeight: 20, iconPath: Images.car)),
          SelectOption(
            label: 'Other',
            value: 'other',
          ),
        ], onChanged: onChanged, selectValue: selectValue)),
      ),
    );
  }
}
