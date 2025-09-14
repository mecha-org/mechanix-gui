import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_bloc.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_event.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_state.dart';
import 'package:mechanix_settings/src/features/sound/data/types.dart';
import 'package:mechanix_settings/src/features/sound/presentation/volume.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';
import 'package:widgets/widgets/switch/mechanix_switch.dart';

class Sound extends StatelessWidget {
  const Sound({super.key});

  void backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SoundBloc, SoundState>(builder: (context, state) {
      return Scaffold(
        appBar: MechanixNavigationBar(title: 'Sound'),
        body: SingleChildScrollView(
          child: ContainerWidget(
            child: Column(
              children: [
                VolumeWidget(),
                MechanixSectionList(
                  title: 'Sound Options',
                  sectionListItems: [
                    SectionListItems(
                      title: 'Launcher Sound',
                      defaultTrailingIcon: false,
                      trailing: MechanixSwitch(
                        activeText: 'OFF',
                        inactiveText: 'ON',
                        value: state.enableLauncherSounds,
                        onChanged: (v) => context
                            .read<SoundBloc>()
                            .add(SetEnableLauncherSoundsEvent(v)),
                      ),
                    ),
                    SectionListItems(
                      title: 'Vibration',
                      defaultTrailingIcon: false,
                      trailing: MechanixSwitch(
                        activeText: 'OFF',
                        inactiveText: 'ON',
                        value: state.enableVibration,
                        onChanged: (v) => context
                            .read<SoundBloc>()
                            .add(SetEnableVibrationEvent(v)),
                      ),
                    ),
                    SectionListItems(
                      title: 'Notification Sound',
                      onTap: () => Navigator.pushNamed(
                          context, AppRoutes.notificationSound),
                      trailing: CustomTrailingText(
                              title: notificationSoundLabel(
                                  state.notificationSound))
                          .padRight(8),
                    ),
                    SectionListItems(
                      title: 'Vibration Level',
                      onTap: () => Navigator.pushNamed(
                          context, AppRoutes.vibrationLevel),
                      trailing: CustomTrailingText(
                              title: vibrationLabel(state.vibrationLevel))
                          .padRight(8),
                    ),
                  ],
                ),
              ],
            ),
          ),
        ),
      );
    });
  }
}
