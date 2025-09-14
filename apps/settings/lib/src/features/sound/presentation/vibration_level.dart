import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_bloc.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_event.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_state.dart';
import 'package:mechanix_settings/src/features/sound/data/types.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/select/select_type.dart';

class VibrationLevel extends StatefulWidget {
  const VibrationLevel({super.key});

  @override
  State<VibrationLevel> createState() => _VibrationLevelState();
}

class _VibrationLevelState extends State<VibrationLevel> {
  void onChanged(SelectOption value) {
    context.read<SoundBloc>().add(SetVibrationLevelEvent(value.value));
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SoundBloc, SoundState>(
      builder: (context, state) {
        return Scaffold(
          appBar: MechanixNavigationBar(
            title: 'Vibration Level',
          ),
          body: SingleChildScrollView(
            child: ContainerWidget(
              child: MechanixSelect(
                value: state.vibrationLevel,
                options: vibrationLevelOptions,
                onChanged: onChanged,
              ),
            ),
          ),
        );
      },
    );
  }
}
