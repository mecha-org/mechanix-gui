import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_toggle.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_state.dart';
import 'package:mechanix_settings/src/features/bluetooth/presentation/widgets/bluetooth_device_list.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/listItems/simple_list_items_type.dart';

class Bluetooth extends StatelessWidget {
  const Bluetooth({super.key});

  void _backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

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
        appBar: CustomAppBar(
          title: "Bluetooth",
          leftIcon: Image.asset(Images.back),
          leftIconOnTap: () => _backNavigation(context),
        ),
        body: ContainerWidget(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            mainAxisAlignment: MainAxisAlignment.start,
            children: [
              MechanixSimpleList(padding: Spacing.bottom(10), listItems: [
                SimpleListItems(
                    title: '${state.adapterAlias ?? 'Bluetooth'}',
                    trailing: CustomToggle(
                        value: state.isPowered,
                        onChanged: (val) => context
                            .read<BluetoothBloc>()
                            .add(ToggleBluetooth(val))))
              ]),
              if (state.isPowered)
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Text('Make device discoverable to everyone').padBottom(40),
                    TextButton(
                        onPressed: () => Navigator.pushNamed(
                            context, AppRoutes.bluetoothDiscoverable),
                        child: Text('Change'))
                  ],
                ),
              if (state.isPowered && state.devices.isNotEmpty)
                BluetoothDeviceList(
                  devices: connectedAndPairedDevices,
                ),
              if (state.isPowered)
                BluetoothDeviceList(
                  devices: newDevices,
                )
            ],
          ),
        ),
      );
    });
  }
}
