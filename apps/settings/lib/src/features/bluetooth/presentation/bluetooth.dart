import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/switch_row.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_state.dart';
import 'package:mechanix_settings/src/features/bluetooth/presentation/widgets/bluetooth_device_list.dart';
import 'package:mechanix_settings/src/features/bluetooth/presentation/widgets/bluetooth_search_new_device.dart';

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
              CustomSwitchListTile(
                title: "Bluetooth",
                value: state.isPowered,
                onChanged: (val) =>
                    context.read<BluetoothBloc>().add(ToggleBluetooth(val)),
              ),
              state.isPowered
                  ? Text(
                      textAlign: TextAlign.left,
                      style: TextStyle(fontSize: 14, color: Color(0xFF898A8D)),
                      "This device is discoverable as ‘${state.adapterAlias}’ as the BT settings is open")
                  : SizedBox(),
              if (state.isPowered && state.devices.isNotEmpty)
                BluetoothDeviceList(
                  devices: connectedAndPairedDevices,
                ),
              if (state.isPowered)
                BluetoothSearchNewDevice(
                    devices: newDevices, isLoading: state.loading)
            ],
          ),
        ),
      );
    });
  }
}
