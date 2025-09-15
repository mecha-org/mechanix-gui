import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_state.dart';
import 'package:mechanix_settings/src/features/bluetooth/presentation/widgets/bluetooth_device_list.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';
import 'package:widgets/widgets/switch/mechanix_switch.dart';

class Bluetooth extends StatefulWidget {
  const Bluetooth({super.key});

  @override
  State<Bluetooth> createState() => _BluetoothState();
}

class _BluetoothState extends State<Bluetooth> {
  @override
  Widget build(BuildContext context) {
    return BlocBuilder<BluetoothBloc, BluetoothState>(
        builder: (context, state) {
      final devices = state.devices;

      final connectedDevices = devices.where((d) => d.connected).toList();

      final pairedDevices =
          devices.where((d) => d.paired && !d.connected).toList();

      final newDevices =
          devices.where((d) => !d.paired && !d.connected).toList();

      final connectedAndPairedDevices = [
        ...connectedDevices,
        ...pairedDevices,
      ].toList();

      return Scaffold(
        appBar: MechanixNavigationBar(title: "Bluetooth"),
        body: ContainerWidget(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisAlignment: MainAxisAlignment.start,
            children: [
              MechanixSectionList(sectionListItems: [
                SectionListItems(
                  title: '${state.adapterAlias ?? 'Bluetooth'}',
                  defaultTrailingIcon: false,
                  trailing: MechanixSwitch(
                      value: state.isPowered,
                      inactiveText: 'ON',
                      activeText: 'OFF',
                      onChanged: (val) => context
                          .read<BluetoothBloc>()
                          .add(ToggleBluetooth(val))),
                ),
                SectionListItems(
                  title: 'Device Discoverable',
                  onTap: () => Navigator.pushNamed(
                      context, AppRoutes.bluetoothDiscoverable),
                  trailing: Row(
                    children: [
                      CustomTrailingText(
                              title: state.isDiscoveryEnabled ? 'Yes' : 'No')
                          .padRight(8),
                    ],
                  ),
                )
              ]),
              if (state.isPowered && state.devices.isNotEmpty)
                BluetoothDeviceList(
                  isPaired: true,
                  devices: connectedAndPairedDevices,
                ),
              if (state.isPowered)
                BluetoothDeviceList(
                  isPaired: false,
                  devices: newDevices,
                )
            ],
          ),
        ),
      );
    });
  }
}
