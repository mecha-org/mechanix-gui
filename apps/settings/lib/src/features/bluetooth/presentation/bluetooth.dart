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
import 'package:mechanix_settings/src/features/bluetooth/models/types.dart';
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
    return BlocSelector<
        BluetoothBloc,
        BluetoothState,
        ({
          List<BluetoothDeviceDetails> devices,
          BluetoothAdapter? bluetoothAdapter,
          bool isPowered,
          bool loading,
        })>(selector: (state) {
      return (
        devices: state.devices,
        bluetoothAdapter: state.bluetoothAdapter,
        isPowered: state.isPowered,
        loading: state.loading,
      );
    }, builder: (context, data) {
      final devices = data.devices;

      final connectedDevices =
          devices.where((d) => d.device.connected && d.device.paired).toList();

      final pairedDevices =
          devices.where((d) => d.device.paired && !d.device.connected).toList();

      final newDevices = devices
          .where((d) => !d.device.paired && !d.device.connected)
          .toList();

      final connectedAndPairedDevices = [
        ...connectedDevices,
        ...pairedDevices,
      ].toList();

      return Scaffold(
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
                        title: data.bluetoothAdapter?.alias ?? 'Bluetooth',
                        trailing: MechanixSwitch(
                          activeText: 'OFF',
                          inactiveText: 'ON',
                          style: MechanixSwitchStyle(
                            activeTrackColor: context.secondaryContainer,
                            inactiveTrackColor: context.secondaryContainer,
                          ),
                          value: data.isPowered,
                          onChanged: (val) => context
                              .read<BluetoothBloc>()
                              .add(ToggleBluetooth(val)),
                        ),
                      ),
                      if (data.isPowered)
                        SimpleListItems(
                          title: 'Device discoverable',
                          onTap: () => Navigator.pushNamed(
                              context, AppRoutes.bluetoothDiscoverable),
                          trailing: Row(
                            crossAxisAlignment: CrossAxisAlignment.center,
                            children: [
                              CustomTrailingText(
                                      title: data.bluetoothAdapter != null &&
                                              data.bluetoothAdapter!
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
                if (data.isPowered &&
                    connectedAndPairedDevices.isEmpty &&
                    data.loading)
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
                if (data.isPowered && connectedAndPairedDevices.isNotEmpty)
                  BluetoothDeviceList(
                    isPaired: true,
                    devices: connectedAndPairedDevices,
                  ),
                if (data.isPowered && newDevices.isEmpty && data.loading)
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
                if (data.isPowered && newDevices.isNotEmpty)
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
