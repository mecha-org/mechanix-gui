import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/styles/text.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_bloc.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_event.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_state.dart';

class InputDevices extends StatelessWidget {
  const InputDevices({super.key});

  void backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {

    return BlocBuilder<SoundBloc, SoundState>(
      builder: (context, state) {

        return Scaffold(
          appBar: CustomAppBar(
            title: "Sound",
            leftIcon: Image.asset(Images.back),
            leftIconOnTap: () => backNavigation(context),
          ),
          body: ContainerWidget(
            child: Column(
              children: [
                Align(
                  alignment: Alignment.centerLeft,
                  child: Text("Input Devices", style: subHeaderTextStyle),
                ),
                Column(
                  children: state.inputDevices.map((device) {
                    return RadioListTile<String>(
                      title: Text(
                        device.description,
                        style: const TextStyle(fontSize: 24),
                      ),
                      value: device.name,
                      groupValue: state.defaultInputDevice?.name,
                      onChanged: (value) {
                        context
                            .read<SoundBloc>()
                            .add(SetInputDevice(device.name));
                        Navigator.pop(context);
                      },
                    );
                  }).toList(),
                )
              ],
            ),
          ),
        );
      },
    );
  }
}
