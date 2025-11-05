import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_icon.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_loader.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_state.dart';
import 'package:mechanix_settings/src/features/bluetooth/presentation/widgets/bluetooth_device_list.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/listItems/simple_list_items_type.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';
import 'package:widgets/widgets/switch/mechanix_switch.dart';
import 'package:widgets/widgets/switch/mechanix_switch_theme.dart';

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

      final connectedDevices =
          devices.where((d) => d.connected && d.paired).toList();

      final pairedDevices =
          devices.where((d) => d.paired && !d.connected).toList();

      final newDevices =
          devices.where((d) => !d.paired && !d.connected).toList();

      final connectedAndPairedDevices = [
        ...connectedDevices,
        ...pairedDevices,
      ].toList();

      return Scaffold(
        appBar: PreferredSize(
            preferredSize: const Size.fromHeight(52),
            child: MechanixNavigationBar(title: "Bluetooth").padHorizontal(12)),
        body: SingleChildScrollView(
          padding: const EdgeInsets.all(16.0),
          physics: const BouncingScrollPhysics(),
          child: ContainerWidget(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                MechanixSimpleList(
                    physics: const BouncingScrollPhysics(),
                    listItems: [
                      SimpleListItems(
                        title: state.bluetoothAdapter?.alias ?? 'Bluetooth',
                        trailing: MechanixSwitch(
                          activeText: 'OFF',
                          inactiveText: 'ON',
                          style: const MechanixSwitchStyle(
                            inactiveThumbColor: Color(0xFF989898),
                            inactiveTrackColor: Color(0xFF252525),
                          ),
                          value: state.isPowered,
                          onChanged: (val) => context
                              .read<BluetoothBloc>()
                              .add(ToggleBluetooth(val)),
                        ),
                      ),
                      if (state.isPowered)
                        SimpleListItems(
                          title: 'Device discoverable',
                          onTap: () => Navigator.pushNamed(
                              context, AppRoutes.bluetoothDiscoverable),
                          trailing: Row(
                            children: [
                              CustomTrailingText(
                                      title: state.bluetoothAdapter != null && state.bluetoothAdapter!.discoverable
                                          ? 'Yes'
                                          : 'No')
                                  .padRight(8),
                              CustomIcon(
                                icon: Image.asset(Images.rightIconArrow),
                                height: 16,
                                width: 16,
                              ).padRight(8),
                            ],
                          ),
                        )
                    ]),

                // paired
                if (state.isPowered && state.devices.isEmpty)
                  MechanixSectionList(
                    physics: const BouncingScrollPhysics(),
                    title: 'Paired Devices',
                    sectionListItems: [
                      SectionListItems(
                        title: '',
                        backgroundColor: Colors.transparent,
                        defaultTrailingIcon: false,
                        leading: CustomLoader(),
                      ),
                    ],
                  ),
                if (state.isPowered &&
                    connectedAndPairedDevices.isNotEmpty)
                  BluetoothDeviceList(
                    isPaired: true,
                    devices: connectedAndPairedDevices,
                  ),

                // availble
                if (state.isPowered && state.devices.isEmpty)
                  MechanixSectionList(
                    physics: const BouncingScrollPhysics(),
                    title: 'Available Devices',
                    sectionListItems: [
                      SectionListItems(
                        title: '',
                        backgroundColor: Colors.transparent,
                        defaultTrailingIcon: false,
                        leading: CustomLoader(),
                      ),
                    ],
                  ),
                if (state.isPowered && newDevices.isNotEmpty)
                  BluetoothDeviceList(
                    isPaired: false,
                    devices: newDevices,
                  ),
              ],
            ),
          ).padTop(8),
        ),
      );
    });
  }
}
