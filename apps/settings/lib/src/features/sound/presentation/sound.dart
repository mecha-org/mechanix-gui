import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/styles/color.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_bloc.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_event.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_state.dart';

class Sound extends StatelessWidget {
  const Sound({super.key});

  void backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SoundBloc, SoundState>(builder: (context, state) {
      final defaultOutputDevice = state.defaultOutputDevice;
      final defaultInputDevice = state.defaultInputDevice;

      return Scaffold(
        appBar: CustomAppBar(
          title: "Sound",
          leftIcon: Image.asset(Images.back),
          leftIconOnTap: () => backNavigation(context),
        ),
        body: ContainerWidget(
          child: Column(
            children: [
              FixedHeightRow(
                showTopBorder: true,
                showBottomBorder: false,
                child: ListTile(
                  title: Text("Output",
                      style:
                          const TextStyle(color: Colors.white, fontSize: 24)),
                  trailing: Text(
                    state.defaultOutputDevice?.description ?? "Select Output Device",
                    style: const TextStyle(color: selectColor, fontSize: 20),
                  ),
                  onTap: () {
                    Navigator.pushNamed(context, AppRoutes.soundOutputDevices);
                  },
                ),
              ),
              const SizedBox(height: 30),
              Padding(
                padding: const EdgeInsets.only(left: 20.0),
                child: Row(children: [
                  IconButton(
                    onPressed: () {
                         context.read<SoundBloc>().add(SetOutputDeviceMute(defaultOutputDevice!.name , !defaultOutputDevice.mute));
                    },
                    icon: (defaultOutputDevice?.mute ?? true)
                        ? Icon(Icons.volume_mute_rounded)
                        : Image.asset(
                            Images.sound,
                            width: 24,
                            height: 24,
                          ),
                  ),
                  Expanded(
                    child: Slider(
                      value: ((defaultOutputDevice?.volume ?? 0.0) * 100).round().toDouble(),
                      min: 0.0,
                      max: 100.0,
                      onChanged: (double value) {
                         context.read<SoundBloc>().add(SetOutputDeviceVolume(defaultOutputDevice!.name , (value/100)));
                      },
                    ),
                  )
                ]),
              ),
              const SizedBox(height: 60),
              FixedHeightRow(
                showTopBorder: true,
                showBottomBorder: false,
                child: ListTile(
                  title: Text("Input",
                      style:
                          const TextStyle(color: Colors.white, fontSize: 24)),
                  trailing: Text(
                    defaultInputDevice?.description ?? "Select Input Device",
                    style: const TextStyle(color: selectColor, fontSize: 20),
                  ),
                  onTap: () {
                    Navigator.pushNamed(context, AppRoutes.soundInputDevices);
                  },
                ),
              ),

              const SizedBox(height: 30),
              Padding(
                padding: const EdgeInsets.only(left: 20.0),
                child: Row(children: [
                  IconButton(
                    onPressed: () {
                      context.read<SoundBloc>().add(SetInputDeviceMute(defaultInputDevice!.name, !defaultInputDevice.mute));
                    },
                    icon: (defaultInputDevice?.mute ?? true)
                        ? Icon(Icons.volume_mute_rounded)
                        : Image.asset(
                            Images.sound,
                            width: 24,
                            height: 24,
                          ),
                  ),
                  Expanded(
                    child: Slider(
                      value: ((defaultInputDevice?.volume ?? 0.0)*100).round().toDouble(),
                      min: 0.0,
                      max: 100.0,
                      onChanged: (double value) {
                         context.read<SoundBloc>().add(SetInputDeviceVolume(defaultInputDevice!.name , (value/100)));
                      },
                    ),
                  )
                ]),
              )
            ],
          ),
        ),
      );
    });
  }
}
