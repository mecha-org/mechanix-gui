import 'package:bluez/bluez.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:widgets/widgets/icon_widget.dart';
import 'package:widgets/widgets/sectionList/mechanix_section_list.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';

class BluetoothDeviceList extends StatelessWidget {
  final List<BlueZDevice> devices;

  const BluetoothDeviceList({
    super.key,
    required this.devices,
  });

  @override
  Widget build(BuildContext context) {
    return MechanixSectionList(
      title: 'Paired Devices',
      sectionListItems: getDeviceList(context, devices),
    );
  }
}

List<SectionListItems> getDeviceList(
    BuildContext context, List<BlueZDevice> devices) {
  final devicesList = devices
      .map((device) => SectionListItems(
          title: device.name,
          onTap: () => onSettingsTap(context, device),
          leading: IconWidget(iconPath: Images.audioHeadset),
          trailing: IconWidget(iconPath: Images.settings)))
      .toList();

  return devicesList;
}

void onDeviceTap(BlueZDevice device, BuildContext context) {
  context.read<BluetoothBloc>().add(SelectDevice(device));
  if (device.paired && !device.connected) {
    context.read<BluetoothBloc>().add(ConnectDevice(device.address));
  } else if (device.connected && device.paired) {
    context.read<BluetoothBloc>().add(DisconnectDevice(device.address));
  } else {
    context.read<BluetoothBloc>().add(PairDevice(device.address));
    context.read<BluetoothBloc>().add(ConnectDevice(device.address));
  }
}

void onSettingsTap(BuildContext context, BlueZDevice device) {
  // context.read<BluetoothBloc>().add(SelectDevice(device));
  Navigator.pushNamed(
    context,
    AppRoutes.bluetoothDeviceInfo,
    arguments: {'selectedDevice': device},
  );
}

void onDeleteTap(BlueZDevice device, BuildContext context) {
  context.read<BluetoothBloc>().add(RemoveDevice(device.address));
}
