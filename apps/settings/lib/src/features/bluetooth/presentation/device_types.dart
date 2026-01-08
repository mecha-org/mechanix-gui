import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/models/types.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/select/select_type.dart';

class DeviceTypes extends StatefulWidget {
  const DeviceTypes({super.key});

  @override
  State<DeviceTypes> createState() => _DeviceTypesState();
}

class _DeviceTypesState extends State<DeviceTypes> {
  late String selectValue;

  @override
  void initState() {
    final state = context.read<BluetoothBloc>().state;

    if (state.selectedDevice != null) {
      setState(() {
        selectValue = state.selectedDevice?.icon ?? 'other';
      });
    }
    super.initState();
  }

  void onChanged(SelectOption option) {
    setState(() {
      selectValue = option.value;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SingleChildScrollView(
        child: ContainerWidget(
            child: Column(
          children: [
            const CustomTitle(title: "Device Types"),
            MechanixSelect(
              options: bluetoothDeviceOptions(selectValue),
              onChanged: onChanged,
              value: selectValue,
            ).padTop(8),
          ],
        )),
      ),
      bottomNavigationBar: MechanixBottomBar(
        leadingWidget: [context.backButton],
      ),
    );
  }
}
