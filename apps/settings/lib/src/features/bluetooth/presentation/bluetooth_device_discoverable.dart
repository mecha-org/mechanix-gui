import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_toggle.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_state.dart';
import 'package:widgets/widgets/listItems/mechanix_simple_list.dart';
import 'package:widgets/widgets/listItems/simple_list_items_type.dart';

class BluetoothDeviceDiscoverable extends StatefulWidget {
  const BluetoothDeviceDiscoverable({super.key});

  @override
  State<BluetoothDeviceDiscoverable> createState() =>
      _BluetoothDeviceDiscoverableState();
}

class _BluetoothDeviceDiscoverableState
    extends State<BluetoothDeviceDiscoverable> {
  void _backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  @override
  void initState() {
    super.initState();
    context.read<BluetoothBloc>().add(GetDiscoverable());
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<BluetoothBloc, BluetoothState>(
      builder: (context, state) {
        return Scaffold(
          appBar: CustomAppBar(
            title: 'Bluetooth',
            leftIcon: Image.asset(Images.back),
            leftIconOnTap: () => _backNavigation(context),
          ),
          body: SingleChildScrollView(
            child: ContainerWidget(
              child: MechanixSimpleList(listItems: [
                SimpleListItems(
                    title: 'Device discoverable to everyone',
                    trailing: CustomToggle(
                        value: state.isDiscoveryEnabled,
                        onChanged: (value) {
                          if (value) {
                            context
                                .read<BluetoothBloc>()
                                .add(DiscoveryEnabled(value));
                            startDiscovery(context);
                          } else {
                            context
                                .read<BluetoothBloc>()
                                .add(DiscoveryEnabled(value));
                            stopDiscovery(context);
                          }
                        }))
              ]),
            ),
          ),
        );
      },
    );
  }
}

void startDiscovery(BuildContext context) {
  context.read<BluetoothBloc>().add(StartDiscovery());
}

void stopDiscovery(BuildContext context) {
  context.read<BluetoothBloc>().add(StopDiscovery());
}
