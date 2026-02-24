import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_loader.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_state.dart';
import 'package:mechanix_settings/src/features/bluetooth/models/types.dart';
import 'package:mechanix_settings/src/features/bluetooth/presentation/widgets/forget_device.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/list_items/simple_list_items_type.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';

class ManageDevice extends StatefulWidget {
  const ManageDevice({super.key});

  @override
  State<ManageDevice> createState() => _ManageDeviceState();
}

class _ManageDeviceState extends State<ManageDevice> {
  List<SimpleListItems> getDeviceList({
    required List<BluetoothDeviceDetails> devices,
    required BluetoothState state,
  }) {
    final devicesList = devices.map((device) {
      return SimpleListItems(
        title: device.device.name,
        trailing: MechanixMenu(
            dropdownSize: const Size(135, 128),
            wingSize: 50,
            dropdownPosition: MenuDropdownPosition.leftStart,
            items: [
              MechanixMenuItemsType(
                title: "About",
                leading: const IconWidget(
                  iconPath: Images.settings,
                  boxWidth: 20,
                  boxHeight: 20,
                  iconWidth: 20,
                  iconHeight: 20,
                ),
                onTap: () {
                  context.read<BluetoothBloc>().add(SelectDevice(device));
                  Navigator.pushNamed(context, AppRoutes.bluetoothDeviceInfo);
                },
              ),
              MechanixMenuItemsType(
                title: "Forget",
                leading: const IconWidget(
                  iconPath: Images.blockIcon,
                  boxWidth: 20,
                  boxHeight: 20,
                  iconWidth: 16,
                  iconHeight: 16,
                ),
                onTap: () {
                  forgetDeviceBottomSheet(
                    context: context,
                    state: state,
                    device: device,
                  );
                },
              ),
            ]),
      );
    }).toList();

    return devicesList;
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<BluetoothBloc, BluetoothState>(
      builder: (context, state) {
        final devices =
            state.devices.where((device) => device.device.paired).toList();

        return Scaffold(
          body: SingleChildScrollView(
            child: ContainerWidget(
                child: Column(
              children: [
                const CustomTitle(title: "Manage devices"),
                if (state.devices.isNotEmpty)
                  MechanixSimpleList(
                    isDividerRequired: false,
                    listItems: getDeviceList(devices: devices, state: state),
                  )
                else
                  MechanixSimpleList(
                    listItems: [
                      SimpleListItems(
                        title: '',
                        leading: const CustomLoader(),
                      ),
                    ],
                  ),
              ],
            )),
          ),
          bottomNavigationBar: MechanixBottomBar(
            leadingWidget: [context.backButton],
          ),
        );
      },
    );
  }
}
