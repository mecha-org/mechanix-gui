import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_state.dart';
import 'package:mechanix_settings/src/features/bluetooth/models/device_type.dart';
import 'package:mechanix_settings/src/features/bluetooth/presentation/widgets/forget_device.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/list_items/simple_list_items_type.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';

class BluetoothDeviceInfo extends StatefulWidget {
  const BluetoothDeviceInfo({super.key});

  @override
  State<BluetoothDeviceInfo> createState() => _BluetoothDeviceInfoState();
}

class _BluetoothDeviceInfoState extends State<BluetoothDeviceInfo> {
  @override
  Widget build(BuildContext context) {
    return BlocBuilder<BluetoothBloc, BluetoothState>(
      builder: (context, state) {
        return Scaffold(
          body: SingleChildScrollView(
            physics: const BouncingScrollPhysics(),
            child: ContainerWidget(
              child: Column(
                children: [
                  CustomTitle(title: state.selectedDevice?.alias ?? ''),
                  MechanixSimpleList(
                    physics: const NeverScrollableScrollPhysics(),
                    isDividerRequired: false,
                    listItems: [
                      SimpleListItems(
                        title: 'Device Name',
                        trailing: CustomTrailingText(
                          title: state.selectedDevice?.alias ?? '',
                        ),
                      ),
                      SimpleListItems(
                        title: 'Device Type',
                        onTap: () => Navigator.pushNamed(
                            context, AppRoutes.bluetoothDeviceTypes),
                        trailing: DeviceType(
                          deviceType: state.selectedDevice?.icon ?? '',
                        ),
                      ),
                      SimpleListItems(
                        title: 'Device Status',
                        trailing: state.selectedDevice!.connected
                            ? Text(
                                "Connected",
                                style: TextStyle(color: context.primary),
                              )
                            : Text(
                                "Disconnected",
                                style:
                                    TextStyle(color: context.onSurfaceVariant),
                              ),
                      ),
                    ],
                  ),
                ],
              ).padTop(8),
            ),
          ),
          bottomNavigationBar: MechanixBottomBar(
            leadingWidget: [context.backButton],
            centerWidget: [
              if (state.selectedDevice != null)
                state.selectedDevice!.connected
                    ? BottomBarButton.widget(
                        widget: TextButton.icon(
                          onPressed: () {
                            context.read<BluetoothBloc>().add(DisconnectDevice(
                                state.selectedDevice?.address ?? ''));
                          },
                          icon: const IconWidget(
                            iconPath: Images.unlinkIcon,
                            iconHeight: 16,
                            iconWidth: 16,
                            iconColor: Colors.white,
                          ),
                          label: Text(
                            "Disconnect",
                            style: TextStyle(color: context.onSurface),
                          ),
                        ),
                      )
                    : BottomBarButton.widget(
                        widget: TextButton.icon(
                          onPressed: () {
                            if (state.selectedDevice != null &&
                                state.selectedDevice!.paired) {
                              context.read<BluetoothBloc>().add(ConnectDevice(
                                  state.selectedDevice?.address ?? ''));
                            } else {
                              context.read<BluetoothBloc>().add(PairDevice(
                                  state.selectedDevice?.address ?? ''));
                            }
                          },
                          icon: const IconWidget(
                            iconPath: Images.connectIcon,
                            iconHeight: 16,
                            iconWidth: 16,
                          ),
                          label: const Text("Connect"),
                        ),
                      )
            ],
            anchorWidget: [
              BottomBarButton.widget(
                widget: MechanixMenu(
                  offset: const Offset(0, -13),
                  dropdownPosition: DropdownPosition.topRight,
                  items: [
                    MechanixMenuItemsType(
                      title: "Forget",
                      onTap: () {
                        forgetDeviceBottomSheet(
                            context: context,
                            state: state,
                            device: state.selectedDevice);
                      },
                    ),
                  ],
                ).padRight(12),
              ),
            ],
          ),
        );
      },
    );
  }
}
