import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_bloc.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_event.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_state.dart';
import 'package:mechanix_settings/src/features/sound/data/types.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/select/select_type.dart';

class NotificationSound extends StatefulWidget {
  const NotificationSound({super.key});

  @override
  State<NotificationSound> createState() => _NotificationSoundState();
}

class _NotificationSoundState extends State<NotificationSound> {
  void onChanged(SelectOption option) {
    context.read<SoundBloc>().add(SetNotificationSoundEvent(option.value));
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SoundBloc, SoundState>(
      builder: (context, state) {
        return Scaffold(
          appBar: MechanixNavigationBar(title: 'Notification Sound'),
          body: ContainerWidget(
            child: MechanixSelect(
              options: notificationSoundOptions,
              value: state.notificationSound,
              onChanged: onChanged,
            ),
          ).padTop(8),
        );
      },
    );
  }
}
