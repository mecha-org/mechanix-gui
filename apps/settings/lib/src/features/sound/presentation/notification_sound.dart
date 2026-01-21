import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_bloc.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_event.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_state.dart';
import 'package:mechanix_settings/src/features/sound/data/types.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/select/select_type.dart';

class NotificationSounds extends StatefulWidget {
  const NotificationSounds({super.key});

  @override
  State<NotificationSounds> createState() => _NotificationSoundsState();
}

class _NotificationSoundsState extends State<NotificationSounds> {
  void onChanged(SelectOption option) {
    context.read<SoundBloc>().add(SetNotificationSoundEvent(option.value));
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SoundBloc, SoundState>(
      builder: (context, state) {
        return Scaffold(
          body: SingleChildScrollView(
            child: ContainerWidget(
              child: Column(
                children: [
                  const CustomTitle(title: 'Notification Sound'),
                  MechanixSelect(
                    options: notificationSoundOptions,
                    value: state.notificationSound,
                    onChanged: onChanged,
                  ),
                ],
              ),
            ).padTop(8),
          ),
          bottomNavigationBar: MechanixBottomBar(
            leadingWidget: [context.backButton],
          ),
        );
      },
    );
  }
}
