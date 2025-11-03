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

  // @override
  // void initState() {
  //   super.initState();
  // }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<BluetoothBloc, BluetoothState>(
      builder: (context, state) {
        print("state.bluetoothAdapter?.discoverable ${state.bluetoothAdapter?.discoverable}");
        return Scaffold(
          appBar: PreferredSize(
              preferredSize: const Size.fromHeight(52),
              child: MechanixNavigationBar(
                      title: (state.bluetoothAdapter?.alias != null && state.bluetoothAdapter?.alias != '')
                          ? '${state.bluetoothAdapter?.alias}'
                          : 'Bluetooth')
                  .padHorizontal(12)),
          body: SingleChildScrollView(
            padding: const EdgeInsets.all(16.0),
            physics: const BouncingScrollPhysics(),
            child: ContainerWidget(
              child: MechanixSimpleList(
                  physics: const NeverScrollableScrollPhysics(),
                  listItems: [
                    SimpleListItems(
                        title: 'Device discoverable to everyone',
                        trailing: MechanixSwitch(
                            allowDrag: false,
                            value: state.bluetoothAdapter?.discoverable ?? false,
                            activeText: 'OFF',
                            inactiveText: 'ON',
                            onChanged: (value) {
                              context
                                  .read<BluetoothBloc>()
                                  .add(DiscoveryEnabled(value));
                           
                            }))
                  ]).padTop(8),
            ),
          ),
        );
      },
    );
  }
}
