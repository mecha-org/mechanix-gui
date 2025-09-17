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

class OutputDevices extends StatefulWidget {
  const OutputDevices({super.key});

  @override
  State<OutputDevices> createState() => _OutputDevicesState();
}

class _OutputDevicesState extends State<OutputDevices> {
  void backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  void onChanged(SelectOption option) {
    context.read<SoundBloc>().add(SetOutputDevice(option.value));
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SoundBloc, SoundState>(
      builder: (context, state) {
        return Scaffold(
          appBar: MechanixNavigationBar(title: 'Output Devices'),
          body: ContainerWidget(
            child: MechanixSelect(
              options: getDevices(state.outputDevices),
              onChanged: onChanged,
              value: state.defaultOutputDevice?.name,
            ),
          ).padTop(8),
        );
      },
    );
  }
}

List<SelectOption> getDevices(List<PulseAudioSink> outputDevices) {
  final devices = outputDevices
      .map((device) => SelectOption(
            label: device.description,
            value: device.name,
            leading: IconWidget(iconPath: Images.speaker),
          ))
      .toList();

  return devices;
}
