import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_state.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/listItems/simple_list_items_type.dart';
import 'package:widgets/widgets/switch/mechanix_switch.dart';

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
    // context.read<BluetoothBloc>().add(DiscoveryEnabled());
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<BluetoothBloc, BluetoothState>(
      builder: (context, state) {
        return Scaffold(
          appBar: MechanixNavigationBar(title: 'Bluetooth'),
          body: SingleChildScrollView(
            child: ContainerWidget(
              child: MechanixSimpleList(listItems: [
                SimpleListItems(
                    title: 'Device discoverable to everyone',
                    trailing: MechanixSwitch(
                        allowDrag: false,
                        value: state.isDiscoveryEnabled,
                        activeText: 'OFF',
                        inactiveText: 'ON',
                        onChanged: (value) {
                          context
                              .read<BluetoothBloc>()
                              .add(DiscoveryEnabled(value));
                          if (value) {
                            context.read<BluetoothBloc>().add(StartDiscovery());
                          } else {
                            context.read<BluetoothBloc>().add(StopDiscovery());
                          }
                          Navigator.pop(context);
                        }))
              ]).padTop(8),
            ),
          ),
        );
      },
    );
  }
}
