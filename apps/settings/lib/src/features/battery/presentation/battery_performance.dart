import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_bloc.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_event.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_state.dart';
import 'package:mechanix_settings/src/features/battery/models/types.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/select/select_type.dart';

class BatteryPerformance extends StatefulWidget {
  const BatteryPerformance({super.key});

  @override
  State<BatteryPerformance> createState() => BatteryPerformanceState();
}

class BatteryPerformanceState extends State<BatteryPerformance> {
  void onChanged(SelectOption option) {
    context.read<BatteryBloc>().add(SetBatteryMode(option.value));
    Navigator.pop(context, option.value);
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<BatteryBloc, BatteryState>(builder: (context, state) {
      return Scaffold(
        body: ContainerWidget(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const CustomTitle(title: 'Performance Mode'),
              Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
                MechanixSelect(
                  options: performanceOptions,
                  onChanged: onChanged,
                  value: state.performanceMode,
                )
              ])
            ],
          ).padTop(8),
        ),
        bottomNavigationBar: MechanixBottomBar(
          leadingWidget: [context.backButton],
        ),
      );
    });
  }
}
