import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_loader.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_state.dart';
import 'package:mechanix_settings/src/features/bluetooth/presentation/widgets/bluetooth_device_list.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/list_items/simple_list_items_type.dart';
import 'package:widgets/widgets/section_list/section_list_items_type.dart';
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
        // appBar: const PreferredSize(
        //     preferredSize: Size.fromHeight(52),
        //     child: MechanixNavigationBar(title: "Bluetooth")),
        body: SingleChildScrollView(
          physics: const BouncingScrollPhysics(),
          child: ContainerWidget(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const CustomTitle(title: "Bluetooth"),
                MechanixSimpleList(
                    isDividerRequired: false,
                    physics: const BouncingScrollPhysics(),
                    listItems: [
                      SimpleListItems(
                        title: state.bluetoothAdapter?.alias ?? 'Bluetooth',
                        // titleTextStyle: TextStyle(color: Colors.red),
                        trailing: MechanixSwitch(
                          activeText: 'OFF',
                          inactiveText: 'ON',
                          style: MechanixSwitchStyle(
                            activeTrackColor: context.secondaryContainer,
                            inactiveTrackColor: context.secondaryContainer,
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
                            crossAxisAlignment: CrossAxisAlignment.center,
                            children: [
                              CustomTrailingText(
                                      title: state.bluetoothAdapter != null &&
                                              state.bluetoothAdapter!
                                                  .discoverable
                                          ? 'Yes'
                                          : 'No')
                                  .padRight(8),
                              IconWidget(
                                iconPath: Images.rightIconArrow,
                                iconWidth: 9,
                                iconHeight: 15,
                                boxWidth: 20,
                                boxHeight: 20,
                                iconColor: context.onSurfaceVariant,
                              ),
                            ],
                          ),
                        )
                    ]),
                if (state.isPowered &&
                    connectedAndPairedDevices.isEmpty &&
                    state.loading)
                  MechanixSectionList(
                    physics: const BouncingScrollPhysics(),
                    title: 'Paired Devices',
                    sectionListItems: [
                      SectionListItems(
                        title: '',
                        backgroundColor: Colors.transparent,
                        defaultTrailingIcon: false,
                        leading: const CustomLoader(),
                      ),
                    ],
                  ),
                if (state.isPowered && connectedAndPairedDevices.isNotEmpty)
                  BluetoothDeviceList(
                    isPaired: true,
                    devices: connectedAndPairedDevices,
                  ),
                if (state.isPowered && newDevices.isEmpty && state.loading)
                  MechanixSectionList(
                    physics: const BouncingScrollPhysics(),
                    title: 'Available Devices',
                    sectionListItems: [
                      SectionListItems(
                        title: '',
                        backgroundColor: Colors.transparent,
                        defaultTrailingIcon: false,
                        leading: const CustomLoader(),
                      ),
                    ],
                  ),
                if (state.isPowered && newDevices.isNotEmpty)
                  BluetoothDeviceList(
                    isPaired: false,
                    devices: newDevices,
                  ),
                MechanixSimpleList(
                  listItems: [
                    SimpleListItems(
                      title: 'Manage Device',
                      onTap: () {
                        Navigator.pushNamed(context, AppRoutes.manageDevice);
                      },
                      trailing: IconWidget(
                        iconPath: Images.rightIconArrow,
                        iconWidth: 9,
                        iconHeight: 15,
                        boxWidth: 20,
                        boxHeight: 20,
                        iconColor: context.onSurfaceVariant,
                      ),
                    ),
                  ],
                )
              ],
            ),
          ),
        ),
        bottomNavigationBar: MechanixBottomBar(
          leadingWidget: [
            context.backButton,
          ],
          anchorWidget: [
            BottomBarButton.widget(
                widget: IconButton(
              onPressed: () {
                context.read<BluetoothBloc>().add(RefreshDeviceList());
              },
              icon: const IconWidget(
                iconPath: Images.arrowCounterClockWise,
                boxWidth: 48,
                boxHeight: 48,
                iconHeight: 21,
                iconWidth: 21,
              ),
            ).padRight(8))
          ],
        ),
      );
    });
  }
}
