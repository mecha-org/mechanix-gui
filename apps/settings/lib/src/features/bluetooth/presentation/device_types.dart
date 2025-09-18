import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/features/bluetooth/models/types.dart';
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
      appBar: MechanixNavigationBar(title: 'Device types'),
      body: SingleChildScrollView(
        child: ContainerWidget(
            child: MechanixSelect(
                    options: bluetoothDeviceOptions,
                    onChanged: onChanged,
                    value: selectValue)
                .padTop(8)),
      ),
    );
  }
}
