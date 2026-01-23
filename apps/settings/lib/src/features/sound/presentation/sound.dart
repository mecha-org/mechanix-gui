import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_bloc.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_event.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_state.dart';
import 'package:mechanix_settings/src/features/sound/data/types.dart';
import 'package:mechanix_settings/src/features/sound/presentation/notification_sound.dart';
import 'package:mechanix_settings/src/features/sound/presentation/volume.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/list_items/simple_list_items_type.dart';
import 'package:widgets/widgets/switch/mechanix_switch.dart';
import 'package:widgets/widgets/switch/mechanix_switch_theme.dart';

class Sound extends StatelessWidget {
  const Sound({super.key});

  void backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  void _onNotificationSoundTap(BuildContext context) {
    final bloc = context.read<SoundBloc>();

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => BlocProvider.value(
          value: bloc,
          child: const NotificationSounds(),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SoundBloc, SoundState>(builder: (context, state) {
      return Scaffold(
        body: SingleChildScrollView(
          physics: const BouncingScrollPhysics(),
          child: ContainerWidget(
            child: Column(
              children: [
                const CustomTitle(title: "Sound"),
                const VolumeWidget(),
                MechanixSimpleList(
                  listItems: [
                    SimpleListItems(
                      title: 'Launcher Sound',
                      trailing: MechanixSwitch(
                        activeText: 'OFF',
                        inactiveText: 'ON',
                        style: MechanixSwitchStyle(
                          activeTrackColor: context.secondaryContainer,
                          inactiveTrackColor: context.secondaryContainer,
                        ),
                        value: state.enableLauncherSounds,
                        onChanged: (v) => context
                            .read<SoundBloc>()
                            .add(SetEnableLauncherSoundsEvent(v)),
                      ),
                    ),
                    SimpleListItems(
                      title: 'Vibration',
                      trailing: MechanixSwitch(
                        activeText: 'OFF',
                        inactiveText: 'ON',
                        style: MechanixSwitchStyle(
                          activeTrackColor: context.secondaryContainer,
                          inactiveTrackColor: context.secondaryContainer,
                        ),
                        value: state.enableVibration,
                        onChanged: (v) => context
                            .read<SoundBloc>()
                            .add(SetEnableVibrationEvent(v)),
                      ),
                    ),
                    SimpleListItems(
                      title: 'Notification Sound',
                      onTap: () => _onNotificationSoundTap(context),
                      trailing: CustomTrailingText(
                              title: notificationSoundLabel(
                                  state.notificationSound))
                          .padRight(8),
                    ),
                  ],
                ),
              ],
            ).padTop(8),
          ),
        ),
        bottomNavigationBar: MechanixBottomBar(
          leadingWidget: [context.backButton],
        ),
      );
    });
  }
}
