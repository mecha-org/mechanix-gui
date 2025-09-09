import 'package:bluez/bluez.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/models/device_type.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/listItems/simple_list_items_type.dart';

class BluetoothDeviceInfo extends StatelessWidget {
  const BluetoothDeviceInfo({super.key});

  void _backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    final args = ModalRoute.of(context)?.settings.arguments;
    if (args is! Map<String, dynamic> || !args.containsKey('selectedDevice')) {
      return const Scaffold(
        body: Center(
          child: Text('No device information provided.'),
        ),
      );
    }

    final BlueZDevice selectedDevice = args['selectedDevice'];

    return Scaffold(
        appBar: CustomAppBar(
          title: selectedDevice.alias,
          leftIcon: Image.asset(Images.back),
          leftIconOnTap: () => _backNavigation(context),
          rightIcon1: Image.asset(Images.delete),
          rightIcon1OnTap: () => _onForgetNetworkClick(context, selectedDevice),
        ),
        body: SingleChildScrollView(
          child: ContainerWidget(
            child: MechanixSimpleList(listItems: [
              SimpleListItems(
                  title: 'Device Name',
                  trailing: Text(
                    selectedDevice.alias,
                    style: context.textTheme.labelLarge,
                  )),
              SimpleListItems(
                  title: 'Device Type',
                  onTap: () => Navigator.pushNamed(
                      context, AppRoutes.bluetoothDeviceTypes),
                  trailing: DeviceType(
                    deviceType: selectedDevice.icon,
                  )),
            ]),
          ),
        ));
  }
}

void _onForgetNetworkClick(BuildContext context, BlueZDevice device) {
  context.read<BluetoothBloc>().add(RemoveDevice(device.address));
  Navigator.pop(context);
}
