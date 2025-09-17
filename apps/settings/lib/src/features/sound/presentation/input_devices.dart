import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_bloc.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_event.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_state.dart';
import 'package:pulseaudio/pulseaudio.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/select/select_type.dart';

class InputDevices extends StatefulWidget {
  const InputDevices({super.key});

  @override
  State<InputDevices> createState() => _InputDevicesState();
}

class _InputDevicesState extends State<InputDevices> {
  void onChanged(SelectOption option) {
    context.read<SoundBloc>().add(SetInputDevice(option.value));
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SoundBloc, SoundState>(
      builder: (context, state) {
        return Scaffold(
          appBar: MechanixNavigationBar(title: 'Input Devices'),
          body: ContainerWidget(
            child: MechanixSelect(
              options: getDevices(state.inputDevices),
              onChanged: onChanged,
              value: state.defaultInputDevice?.name,
            ),
          ).padTop(8),
        );
      },
    );
  }
}

List<SelectOption> getDevices(List<PulseAudioSource> inputDevices) {
  final devices = inputDevices
      .map((device) => SelectOption(
            label: device.description,
            value: device.name,
            leading: IconWidget(iconPath: Images.mic),
          ))
      .toList();

  return devices;
}
