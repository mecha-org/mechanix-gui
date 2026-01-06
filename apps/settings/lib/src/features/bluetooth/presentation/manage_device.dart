import 'package:bluez/bluez.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_loader.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_state.dart';
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
    required List<BlueZDevice> devices,
    required BluetoothState state,
  }) {
    final devicesList = devices.map((device) {
      return SimpleListItems(
        title: device.name,
        trailing: MechanixMenu(
            dropdownPosition: DropdownPosition.bottomRight,
            offset: const Offset(-35, -35),
            items: [
              MechanixMenuItemsType(
                title: 'Forget',
                onTap: () {
                  forgetDeviceBottomSheet(
                    context: context,
                    state: state,
                    device: device,
                  );
                },
              ),
              MechanixMenuItemsType(
                title: 'About',
                onTap: () {
                  context.read<BluetoothBloc>().add(SelectDevice(device));
                },
              )
            ]),
      );
    }).toList();

    return devicesList;
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<BluetoothBloc, BluetoothState>(
      builder: (context, state) {
        final devices = state.devices;

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
          bottomSheet: MechanixBottomBar(
            leadingWidget: [context.backButton],
          ),
        );
      },
    );
  }
}
