import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_bloc.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_event.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_state.dart';
import 'package:mechanix_settings/src/features/battery/models/types.dart';
import 'package:widgets/widgets.dart';
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
        appBar: MechanixNavigationBar(title: 'Performance Mode'),
        body: ContainerWidget(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
                MechanixSelect(
                  options: performanceOptions,
                  onChanged: onChanged,
                  value: state.performanceMode,
                )
              ])
            ],
          ),
        ),
      );
    });
  }
}
