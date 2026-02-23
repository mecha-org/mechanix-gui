import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_loader.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_state.dart';
import 'package:mechanix_settings/src/features/bluetooth/models/bluetooth_device_classifier.dart';
import 'package:mechanix_settings/src/features/bluetooth/models/types.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/section_list/section_list_items_type.dart';

class BluetoothDeviceList extends StatefulWidget {
  const BluetoothDeviceList({
    super.key,
    required this.devices,
    required this.isPaired,
  });

  final List<BluetoothDeviceDetails> devices;

  final bool isPaired;

  @override
  State<BluetoothDeviceList> createState() => _BluetoothDeviceListState();
}

class _BluetoothDeviceListState extends State<BluetoothDeviceList> {
  void onSettingsTap(BluetoothDeviceDetails device) {
    context.read<BluetoothBloc>().add(SelectDevice(device));
    Navigator.pushNamed(context, AppRoutes.bluetoothDeviceInfo);
  }

  void onDeviceTap(BluetoothDeviceDetails device) {
    if (device.device.paired && !device.device.connected) {
      context.read<BluetoothBloc>().add(ConnectDevice(device.device.address));
    }
  }

  List<SectionListItems> getDeviceList({
    required List<BluetoothDeviceDetails> devices,
    required BluetoothState state,
  }) {
    final devicesList = devices.map((deviceDetail) {
      final device = deviceDetail.device;

      return SectionListItems.leadingIcon(
        title: device.name,
        titleTextStyle: device.connected
            ? context.textTheme.labelMedium?.copyWith(color: context.primary)
            : context.textTheme.labelMedium,
        defaultTrailingIcon: false,
        onTap: () => onDeviceTap(deviceDetail),
        iconPath: BluetoothDeviceClassifier.deviceIcon(deviceDetail.deviceType),
        iconColor: context.onSecondary,
        activeIconColor: context.primary,
        isActive: device.connected,
        trailing: Row(
          children: [
            if (state.connection != null &&
                state.connection?.address == device.address &&
                state.connection!.connectionLoading)
              const CustomLoader().padRight(8),
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
              onPressed: () => onSettingsTap(deviceDetail),
              icon: IconWidget(
                iconPath: Images.settings,
                iconColor: context.onSecondaryFixed,
                isActive: device.connected,
                activeIconColor: context.onSecondaryFixed,
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
    final state = context.read<BluetoothBloc>().state;

    return MechanixSectionList(
      title: widget.isPaired ? 'Paired Devices' : 'Available Devices',
      physics: const NeverScrollableScrollPhysics(),
      sectionListItems: getDeviceList(devices: widget.devices, state: state),
    );
  }
}
