import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_state.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/select/select_type.dart';

class BluetoothDeviceDiscoverable extends StatefulWidget {
  const BluetoothDeviceDiscoverable({super.key});

  @override
  State<BluetoothDeviceDiscoverable> createState() =>
      _BluetoothDeviceDiscoverableState();
}

class _BluetoothDeviceDiscoverableState
    extends State<BluetoothDeviceDiscoverable> {
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
                  const CustomTitle(title: "Device Discoverability"),
                  MechanixSelect(
                    value: state.isDiscoveryEnabled,
                    options: const [
                      SelectOption(value: true, label: 'Yes'),
                      SelectOption(value: false, label: 'No'),
                    ],
                    onChanged: (value) {
                      context
                          .read<BluetoothBloc>()
                          .add(DiscoveryEnabled(value.value));
                    },
                  ),
                ],
              ),
            ),
          ),
          bottomSheet: MechanixBottomBar(
            leadingWidget: [
              context.backButton,
            ],
          ),
        );
      },
    );
  }
}
