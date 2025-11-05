import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/styles/text.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_bloc.dart';
import 'package:mechanix_settings/src/features/bluetooth/blocs/bluetooth_state.dart';
import 'package:widgets/mechanix.dart';

class RenameAdapter extends StatelessWidget {
  const RenameAdapter({super.key});

  @override
  Widget build(BuildContext context) {
    final formKey = GlobalKey<FormState>();

  

    // void saveInfo(BuildContext context, String newName) async {
    //   print("1. navigating back with new name: $newName");

    //   if (formKey.currentState!.validate()) {
    //     context.read<BluetoothBloc>().add(RenameAdapterEvent(newName));
    //     print("2. navigating back with new name: $newName");
    //     Navigator.pop(context, newName);
    //   }
    // }

    return BlocBuilder<BluetoothBloc, BluetoothState>(
      builder: (context, state) {
        final nameController = TextEditingController();
        nameController.text = state.bluetoothAdapter?.alias ?? '';

        return Scaffold(
          appBar: const MechanixNavigationBar(title: "Rename Device"),
          body: ContainerWidget(
            child: Form(
              key: formKey,
              child: Column(
                children: [
                  const SizedBox(height: 10),
                  Row(
                    crossAxisAlignment: CrossAxisAlignment.center,
                    children: [
                      const SizedBox(
                        width: 120,
                        child: Text("Name", style: labelTextStyle),
                      ),
                      Expanded(
                        child: TextFormField(
                          controller: nameController,
                          style: inputFieldTextStyle,
                          decoration: const InputDecoration(
                            hintText: "Enter name",
                            border: OutlineInputBorder(),
                            focusedBorder: OutlineInputBorder(
                              borderSide: BorderSide(color: Colors.blue),
                            ),
                            isDense: true,
                            contentPadding: EdgeInsets.symmetric(
                                horizontal: 12, vertical: 10),
                          ),
                          validator: (value) {
                            if (value == null || value.isEmpty) {
                              return 'Please enter bluetooth name';
                            }
                            return null;
                          },
                        ),
                      ),
                    ],
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
