import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_state.dart';
import 'package:mechanix_settings/src/features/bluetooth/models/types.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/select/select_type.dart';

class DeviceTypes extends StatelessWidget {
  const DeviceTypes({super.key});

  void _onChanged(
    BuildContext context,
    BluetoothDeviceDetails? device,
    BluetoothDeviceCategory type,
  ) {
    if (device == null) return;

    context.read<BluetoothBloc>().add(
          BluetoothDevicesUpdate(device: device, category: type),
        );
  }

  SelectOption<BluetoothDeviceCategory> _selectOptionLabel({
    required BluetoothDeviceCategory value,
    required String label,
    required String icon,
    required BluetoothDeviceCategory selectedDeviceType,
  }) {
    return SelectOption(
      value: value,
      label: label,
      leading: IconWidget(
        iconPath: icon,
        iconWidth: 16,
        iconHeight: 20,
        isActive: value == selectedDeviceType,
      ),
    );
  }

  List<SelectOption<BluetoothDeviceCategory>> _selectOptions(
    BluetoothDeviceCategory deviceType,
  ) {
    return [
      _selectOptionLabel(
          value: BluetoothDeviceCategory.speaker,
          label: "Speaker",
          icon: Images.speaker,
          selectedDeviceType: deviceType),
      _selectOptionLabel(
          value: BluetoothDeviceCategory.headphones,
          label: "Headphone",
          icon: Images.audioHeadset,
          selectedDeviceType: deviceType),
      _selectOptionLabel(
          value: BluetoothDeviceCategory.mobile,
          label: "Mobile",
          icon: Images.mobile,
          selectedDeviceType: deviceType),
      _selectOptionLabel(
          value: BluetoothDeviceCategory.computer,
          label: "PC",
          icon: Images.tv,
          selectedDeviceType: deviceType),
      _selectOptionLabel(
          value: BluetoothDeviceCategory.tv,
          label: "TV",
          icon: Images.tv,
          selectedDeviceType: deviceType),
      _selectOptionLabel(
          value: BluetoothDeviceCategory.car,
          label: "Car",
          icon: Images.car,
          selectedDeviceType: deviceType),
      _selectOptionLabel(
          value: BluetoothDeviceCategory.unknown,
          label: "Other",
          icon: Images.audioHeadset,
          selectedDeviceType: deviceType),
    ];
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SingleChildScrollView(
        child: ContainerWidget(
          child: Column(
            children: [
              const CustomTitle(title: "Device type"),
              BlocSelector<BluetoothBloc, BluetoothState,
                  BluetoothDeviceDetails?>(
                selector: (state) => state.selectedDevice,
                builder: (context, device) {
                  return MechanixSelect(
                    value: device?.deviceType,
                    options: _selectOptions(
                        device?.deviceType ?? BluetoothDeviceCategory.unknown),
                    onChanged: (v) => _onChanged(context, device, v.value),
                  );
                },
              ).padTop(8),
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
