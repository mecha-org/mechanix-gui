import 'package:bluez/bluez.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_loader.dart';
import 'package:mechanix_settings/src/commons/styles/custom_styles.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/presentation/widgets/bluetooth_device_list.dart';

class BluetoothSearchNewDevice extends StatelessWidget {
  final List<BlueZDevice> devices;
  final bool isLoading;
  const BluetoothSearchNewDevice(
      {super.key, required this.devices, required this.isLoading});

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        SizedBox(
          height: 20,
        ),
        Flex(
          direction: Axis.horizontal,
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          crossAxisAlignment: CrossAxisAlignment.end,
          children: [
            Text(
                textAlign: TextAlign.left,
                style: baseHeaderStyle.copyWith(fontSize: 14),
                "Other Devices"),
            !isLoading
                ? TextButton(
                    onPressed: () => setDiscovery(context),
                    style: buttonStyle,
                    child: Text("Find new devices",
                        style:
                            baseHeaderStyle.copyWith(color: Color(0xFFF0F0F0))),
                  )
                : Padding(
                    padding: EdgeInsets.only(right: 25),
                    child: CustomLoader(),
                  )
          ],
        ),
        BluetoothDeviceList(
          devices: devices,
        ),
      ],
    );
  }
}

void setDiscovery(BuildContext context) {
  context.read<BluetoothBloc>().add(StartDiscovery());
}
