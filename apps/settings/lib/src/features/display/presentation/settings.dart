import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/features/display/bloc/display_bloc.dart';
import 'package:mechanix_settings/src/features/display/models/types.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/select/select_type.dart';

class ScreenOffTimeSettings extends StatefulWidget {
  const ScreenOffTimeSettings({super.key});

  @override
  State<ScreenOffTimeSettings> createState() => ScreenOffTimeSettingsState();
}

class ScreenOffTimeSettingsState extends State<ScreenOffTimeSettings> {
  @override
  void initState() {
    super.initState();
  }

  void onChanged(SelectOption option) {
    if (option.value is DisplayScreenOffTime) {
      context
          .read<DisplayBloc>()
          .add(SetDisplayTimeoutEvent(displayTime(option.value)));
    }
    Navigator.pop(context, option.value);
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<DisplayBloc, DisplayState>(
      builder: (context, state) {
        return Scaffold(
          appBar: MechanixNavigationBar(
            title: 'Screen Off Time',
          ),
          body: ContainerWidget(
              child: MechanixSelect(
            options: screenOffOptions,
            onChanged: onChanged,
            value: getValue(state.screenTimeout),
          )),
        );
      },
    );
  }
}
