import 'package:bluez/bluez.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/presentation/widgets/bluetooth_list_row.dart';

class BluetoothDeviceList extends StatelessWidget {
  final List<BlueZDevice> devices;

  const BluetoothDeviceList({
    super.key,
    required this.devices,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        SizedBox(height: 10),
        ListView.builder(
            shrinkWrap: true,
            padding: EdgeInsets.zero,
            itemCount: devices.length,
            itemBuilder: (context, index) {
              final device = devices[index];

              return BluetoothListRow(
                title: device.name,
                isConnected: device.connected,
                isAvailable: device.adapter.powered,
                onDeviceTap: () => onDeviceTap(device, context),
                onDeleteTap: () => onDeleteTap(device, context),
                onSettingsTap: () => onSettingsTap(device, context),
              );
            })
      ],
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
    context.read<BluetoothBloc>().add(ConnectDevice(device.address));
  }
}

void onSettingsTap(BlueZDevice device, BuildContext context) {
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
