import 'package:bluez/bluez.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_event.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_state.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_sheet_modals/mechanix_bottom_sheet_theme.dart';
import 'package:widgets/widgets/filled_button/mechanix_filled_button_theme.dart';

void forgetDeviceBottomSheet({
  required BuildContext context,
  required BluetoothState state,
  required BlueZDevice? device,
}) {
  return MechanixBottomSheet.show(
    context,
    theme: MechanixBottomSheetThemeData(
        decoration: BoxDecoration(color: context.surfaceContainerHigh),
        padding: const EdgeInsets.fromLTRB(16, 30, 16, 29)),
    child: SizedBox(
      height: 130,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisAlignment: MainAxisAlignment.start,
        children: [
          const Text(
            "Forget Device?",
            style: TextStyle(fontWeight: FontWeight.w500, fontSize: 20),
          ).padBottom(12),
          Text("Your Device will no longer be paired with '${state.selectedDevice?.alias ?? ''}'")
              .padBottom(24),
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
                  context
                      .read<BluetoothBloc>()
                      .add(RemoveDevice(state.selectedDevice?.address ?? ''));
                },
                theme: const MechanixFilledButtonThemeData(
                  buttonColor: Color.fromRGBO(211, 0, 42, 1),
                  pressedButtonColor: Color.fromRGBO(255, 0, 58, 1),
                ),
                label: "Forget",
              )
            ],
          )
        ],
      ),
    ),
  );
}
