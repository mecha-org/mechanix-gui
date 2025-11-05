import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_bloc.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_state.dart';
import 'package:mechanix_settings/src/features/battery/models/types.dart';
import 'package:mechanix_settings/src/features/battery/presentation/battery_indicator.dart';
import 'package:upower/upower.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';

class Battery extends StatefulWidget {
  const Battery({super.key});

  @override
  State<Battery> createState() => BatteryScreenState();
}

class BatteryScreenState extends State<Battery> {
  @override
  Widget build(BuildContext context) {
    return BlocBuilder<BatteryBloc, BatteryState>(
      builder: (context, state) {
        print('state.batteryStatus');
        print('${state.batteryStatus}');
        final batteryPercentageValue = state.batteryPercentage.toInt();
        final totalSeconds = UPowerDeviceState.charging == state.batteryStatus
            ? state.batteryChargingTime ?? 0
            : state.batteryRemainingTime ?? 0;

        final hours = totalSeconds ~/ 3600;
        final minutes = (totalSeconds % 3600) ~/ 60;

        final timeText = [
          if (hours > 0) "$hours hrs",
          if (minutes > 0) "$minutes mins"
        ].join(' ');

        return Scaffold(
          appBar: const MechanixNavigationBar(
            title: "Battery",
          ),
          body: SingleChildScrollView(
            physics: const BouncingScrollPhysics(),
            child: ContainerWidget(
              child: Column(
                // crossAxisAlignment: CrossAxisAlignment.start,
                // spacing: 10,
                children: [
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      Text(
                        state.batteryStatus == UPowerDeviceState.charging
                            ? "$timeText to Full Charge"
                            : "$timeText Left",
                        style: context.textTheme.labelLarge,
                      ),
                      Text(
                        '$batteryPercentageValue %',
                        style: context.textTheme.labelLarge,
                      )
                    ],
                  ).padVertical(8),
                  BatteryIndicator(
                    isCharging:
                        UPowerDeviceState.charging == state.batteryStatus,
                  ),
                  MechanixSectionList(title: 'Battery Mode', sectionListItems: [
                    SectionListItems(
                        title: 'Performance',
                        onTap: () => Navigator.pushNamed(
                            context, AppRoutes.batteryPerformance),
                        trailing: CustomTrailingText(
                                title:
                                    getModeDetails(state.performanceMode ?? '')
                                        .mode)
                            .padRight(8))
                  ]).padTop(40),
                ],
              ).padTop(8),
            ),
          ),
        );
      },
    );
  }
}
