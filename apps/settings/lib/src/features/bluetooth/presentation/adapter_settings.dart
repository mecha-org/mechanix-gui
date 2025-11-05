import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_state.dart';
import 'package:widgets/mechanix.dart';

class AdapterSettings extends StatelessWidget {
  const AdapterSettings({super.key});

  Future<void> _navigateAndDisplayValue(BuildContext context) async {
    final result = await Navigator.pushNamed(
      context,
      AppRoutes.adapterRename,
    );
    print('Result from RenameAdapter: $result');
  }

  void onTap(BuildContext context, String deviceName) {
    print('Tapped on device name: $deviceName');
    // var result = Navigator.pushNamed(
    //   context,
    //   AppRoutes.adapterRename,
    // );
    _navigateAndDisplayValue(context);
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<BluetoothBloc, BluetoothState>(
        builder: (context, state) {
      String deviceName = state.bluetoothAdapter?.alias ?? '';

      return Scaffold(
        appBar: const MechanixNavigationBar(title: 'Bluetooth Settings'),
        body: SingleChildScrollView(
          physics: const BouncingScrollPhysics(),
          child: ContainerWidget(
            child: ListTile(
              title: const Text('Device name',
                  style: TextStyle(color: Colors.white, fontSize: 24)),
              trailing: Text(
                deviceName,
                style: const TextStyle(
                    color: Color.fromARGB(197, 255, 255, 255), fontSize: 24),
              ),
              onTap: () => onTap(context, deviceName),
            ).padTop(8),
          ),
        ),
      );
    });
  }
}
