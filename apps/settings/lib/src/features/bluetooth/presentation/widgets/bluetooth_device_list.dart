import 'package:bluez/bluez.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_state.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';

class BluetoothDeviceList extends StatefulWidget {
  const BluetoothDeviceList({
    super.key,
    required this.devices,
    required this.isPaired,
  });

  final List<BlueZDevice> devices;

  final bool isPaired;

  @override
  State<BluetoothDeviceList> createState() => _BluetoothDeviceListState();
}

class _BluetoothDeviceListState extends State<BluetoothDeviceList> {
  void onSettingsTap(BlueZDevice device) {
    context.read<BluetoothBloc>().add(SelectDevice(device));
    Navigator.pushNamed(
      context,
      AppRoutes.bluetoothDeviceInfo,
    );
  }

  void onDeviceTap(BlueZDevice device) {
    if (device.paired && !device.connected) {
      context.read<BluetoothBloc>().add(ConnectDevice(device.address));
    } else if (device.connected && device.paired) {
      context.read<BluetoothBloc>().add(DisconnectDevice(device.address));
    } else {
      context.read<BluetoothBloc>().add(PairDevice(device.address));
      context.read<BluetoothBloc>().add(ConnectDevice(device.address));
    }
  }

  List<SectionListItems> getDeviceList({
    required List<BlueZDevice> devices,
  }) {
    final devicesList = devices
        .map((device) => SectionListItems(
            title: device.name,
            defaultTrailingIcon: false,
            onTap: () => onDeviceTap(device),
            leading: IconWidget(
              iconPath: Images.audioHeadset,
              isActive: device.connected && device.paired,
            ),
            trailing: IconButton(
                onPressed: () => onSettingsTap(device),
                icon: IconWidget(iconPath: Images.settings))))
        .toList();

    return devicesList;
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<BluetoothBloc, BluetoothState>(
      builder: (context, state) {
        return MechanixSectionList(
          title: widget.isPaired ? 'Paired Devices' : 'Available Devices',
          physics: const NeverScrollableScrollPhysics(),
          sectionListItems: getDeviceList(devices: widget.devices),
        );
      },
    );
  }
}

void onDeviceTap(BlueZDevice device, BuildContext context) {
  context.read<BluetoothBloc>().add(SelectDevice(device));
  if (device.paired && !device.connected) {
    context.read<BluetoothBloc>().add(ConnectDevice(device.address));
  } else if (device.connected && device.paired) {
    context.read<BluetoothBloc>().add(DisconnectDevice(device.address));
  } else {
    context.read<BluetoothBloc>().add(PairDevice(device.address));
    // context.read<BluetoothBloc>().add(ConnectDevice(device.address));
  }
}

void onDeleteTap(BlueZDevice device, BuildContext context) {
  context.read<BluetoothBloc>().add(RemoveDevice(device.address));
}
