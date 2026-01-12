import 'package:bluez/bluez.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_state.dart';
import 'package:mechanix_settings/src/features/bluetooth/models/bluetooth_device_icon.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/section_list/section_list_items_type.dart';

class BluetoothDeviceList extends StatefulWidget {
  const BluetoothDeviceList({
    super.key,
    required this.devices,
    required this.isPaired,
  });

  final List<BlueZDevice> devices;

  final bool isPaired;

  @override
  State<BluetoothDeviceList> createState() => _BluetoothDeviceListState();
}

class _BluetoothDeviceListState extends State<BluetoothDeviceList> {
  void onSettingsTap(BlueZDevice device) {
    context.read<BluetoothBloc>().add(SelectDevice(device));
    Navigator.pushNamed(
      context,
      AppRoutes.bluetoothDeviceInfo,
    );
  }

  void onDeviceTap(BlueZDevice device) {
    if (device.paired && !device.connected) {
      context.read<BluetoothBloc>().add(ConnectDevice(device.address));
    }
  }

  List<SectionListItems> getDeviceList({
    required List<BlueZDevice> devices,
  }) {
    final devicesList = devices.map((device) {
      return SectionListItems.leadingIcon(
        title: device.name,
        titleTextStyle: device.connected
            ? context.textTheme.labelMedium?.copyWith(color: context.primary)
            : context.textTheme.labelMedium,
        defaultTrailingIcon: false,
        onTap: () => onDeviceTap(device),
        iconPath: getBluetoothDeviceIcon(device.icon),
        activeIconColor: context.primary,
        isActive: device.connected,
        trailing: Row(
          children: [
            if (device.connected)
              IconWidget(
                iconPath: Images.circularCheckIcon,
                iconColor: context.primary,
                isActive: device.connected,
                iconHeight: 19,
                iconWidth: 19,
                activeIconColor: context.primary,
              ).padRight(8),
            IconButton(
              onPressed: () => onSettingsTap(device),
              icon: IconWidget(
                iconPath: Images.settings,
                iconColor: context.surfaceContainerHigh,
                isActive: device.connected,
                activeIconColor: context.onSurface,
              ),
            ),
          ],
        ),
      );
    }).toList();

    return devicesList;
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<BluetoothBloc, BluetoothState>(
      builder: (context, state) {
        return MechanixSectionList(
          title: widget.isPaired ? 'Paired Devices' : 'Available Devices',
          physics: const NeverScrollableScrollPhysics(),
          sectionListItems: getDeviceList(devices: widget.devices),
        );
      },
    );
  }
}
