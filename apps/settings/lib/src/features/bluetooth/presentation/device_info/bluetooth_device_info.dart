import 'package:bluez/bluez.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_icon.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_state.dart';
import 'package:mechanix_settings/src/features/bluetooth/models/device_type.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/list_items/simple_list_items_type.dart';

class BluetoothDeviceInfo extends StatefulWidget {
  const BluetoothDeviceInfo({super.key});

  @override
  State<BluetoothDeviceInfo> createState() => _BluetoothDeviceInfoState();
}

class _BluetoothDeviceInfoState extends State<BluetoothDeviceInfo> {
  void onForgetNetworkClick(BlueZDevice? device) {
    if (device != null) {
      context.read<BluetoothBloc>().add(RemoveDevice(device.address));
      Navigator.pop(context);
    }
  }

  void onUnLinkClick(BlueZDevice? device) {
    if (device != null) {
      context.read<BluetoothBloc>().add(DisconnectDevice(device.address));
      Navigator.pop(context);
    }
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<BluetoothBloc, BluetoothState>(
      builder: (context, state) {
        return Scaffold(
          appBar: PreferredSize(
            preferredSize: const Size.fromHeight(52),
            child: MechanixNavigationBar(
              title: state.selectedDevice?.alias ?? state.selectedDevice?.name,
              actionWidgets: [
                if (state.selectedDevice != null &&
                    (state.selectedDevice!.connected ||
                        state.selectedDevice!.paired))
                  IconButton(
                    onPressed: () => onForgetNetworkClick(state.selectedDevice),
                    style: ButtonStyle(
                      iconColor: WidgetStateProperty.all(Colors.white),
                      backgroundColor:
                          WidgetStateProperty.all(const Color(0xFFB71C1C)),
                      shape: WidgetStateProperty.all(
                        RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(8.0),
                        ),
                      ),
                      minimumSize: WidgetStateProperty.all(const Size(32, 32)),
                      fixedSize: WidgetStateProperty.all(const Size(32, 32)),
                      padding: WidgetStateProperty.all(EdgeInsets.zero),
                    ),
                    icon: Center(
                      child: CustomIcon(
                        icon: Image.asset(Images.trash),
                        width: 15,
                        height: 17,
                      ),
                    ),
                  ).padRight(16)
              ],
            ).padHorizontal(12),
          ),
          body: SingleChildScrollView(
            padding: const EdgeInsets.all(16.0),
            physics: const BouncingScrollPhysics(),
            child: ContainerWidget(
              child: Column(
                children: [
                  MechanixSimpleList(
                    physics: const NeverScrollableScrollPhysics(),
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
                    ],
                  ),
                  if (state.selectedDevice != null &&
                      (state.selectedDevice!.connected ||
                          state.selectedDevice!.paired))
                    SizedBox(
                      child: TextButton.icon(
                        onPressed: () => onUnLinkClick(state.selectedDevice),
                        style: ButtonStyle(
                          backgroundColor: WidgetStateProperty.all<Color>(
                            context.colorScheme.secondary,
                          ),
                        ),
                        label: Text(
                          'Unlink Device',
                          style:
                              TextStyle(color: context.colorScheme.onSurface),
                        ).padVertical(16),
                        icon: const IconWidget(
                          iconPath: Images.unlinkIcon,
                          iconHeight: 16,
                          iconWidth: 16,
                          iconColor: Colors.white,
                        ),
                      ),
                    ),
                ],
              ).padTop(8),
            ),
          ),
        );
      },
    );
  }
}
