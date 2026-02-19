import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_loader.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_bloc.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_event.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_state.dart';
import 'package:pulseaudio/pulseaudio.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottom_bar/mechanix_bottom_bar_theme.dart';
import 'package:widgets/widgets/list_items/simple_list_items_type.dart';
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
    return BlocSelector<
        SoundBloc,
        SoundState,
        ({
          bool loading,
          List<PulseAudioSource> devices,
          PulseAudioSource? defaultDevice
        })>(
      selector: (state) {
        return (
          loading: state.inputDeviceLoading,
          devices: state.inputDevices,
          defaultDevice: state.defaultInputDevice,
        );
      },
      builder: (context, data) {
        return Scaffold(
          body: SingleChildScrollView(
            child: ContainerWidget(
              child: Column(
                children: [
                  const CustomTitle(title: 'Input Devices'),
                  AnimatedSwitcher(
                    duration: const Duration(milliseconds: 300),
                    switchInCurve: Curves.easeOut,
                    switchOutCurve: Curves.easeIn,
                    transitionBuilder: (child, animation) {
                      return FadeTransition(
                        opacity: animation,
                        child: SizeTransition(
                          sizeFactor: animation,
                          axisAlignment: -1,
                          child: child,
                        ),
                      );
                    },
                    child: data.loading
                        ? MechanixSimpleList(listItems: [
                            SimpleListItems(
                              title: '',
                              leading: const CustomLoader(),
                            ),
                          ])
                        : MechanixSelect(
                            key:
                                ValueKey(data.defaultDevice?.name), // IMPORTANT
                            options:
                                getDevices(data.devices, data.defaultDevice),
                            onChanged: onChanged,
                            value: data.defaultDevice?.name,
                          ),
                  )
                ],
              ),
            ).padTop(8),
          ),
          bottomNavigationBar: MechanixBottomBar(
            leadingWidget: [context.backButton],
            anchorWidget: [
              BottomBarButton(
                onPressed: () {
                  context.read<SoundBloc>().add(RefreshInputDevicesList());
                },
                iconTheme: const MechanixBottomBarIconThemeData(
                  buttonSize: Size(44, 44),
                  iconBoxSize: Size(28, 28),
                  iconSize: Size(21.88, 21.45),
                  buttonMargin: EdgeInsets.only(right: 12),
                ),
                iconPath: Images.arrowCounterClockWise,
              )
            ],
          ),
        );
      },
    );
  }
}

List<SelectOption> getDevices(
    List<PulseAudioSource> inputDevices, PulseAudioSource? defaultInputDevice) {
  final devices = inputDevices
      .map((device) => SelectOption(
            label: device.description,
            value: device.name,
            leading: IconWidget(
              iconPath: defaultInputDevice?.name == device.name
                  ? Images.cometIcon
                  : Images.mic,
            ),
          ))
      .toList();

  return devices;
}
