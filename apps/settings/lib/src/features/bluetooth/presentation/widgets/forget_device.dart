import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_state.dart';
import 'package:mechanix_settings/src/features/bluetooth/models/types.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/filled_button/mechanix_filled_button_theme.dart';

void forgetDeviceBottomSheet({
  required BuildContext context,
  required BluetoothState state,
  required BluetoothDeviceDetails? device,
}) {
  MechanixBottomSheet.show(
    context,
    sheetHeight: 210,
    wingWidth: 100,
    child: SizedBox(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisSize: MainAxisSize.min,
        children: [
          const Text(
            "Forget Device?",
            style: TextStyle(fontWeight: FontWeight.w600, fontSize: 24),
          ).padBottom(12),
          Text(
            "Your Device will no longer be paired with '${state.selectedDevice?.device.alias ?? ''}'",
          ).padBottom(20),
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              MechanixFilledButton(
                onPressed: () {
                  Navigator.of(context).pop();
                },
                label: "Cancel",
              ),
              MechanixFilledButton(
                onPressed: () {
                  context.read<BluetoothBloc>().add(
                        RemoveDevice(
                            state.selectedDevice?.device.address ?? ''),
                      );
                },
                theme: const MechanixFilledButtonThemeData(
                  buttonColor: Color.fromRGBO(211, 0, 42, 1),
                  pressedButtonColor: Color.fromRGBO(255, 0, 58, 1),
                ),
                label: "Forget",
              ),
            ],
          ),
        ],
      ),
    ),
  );
}
