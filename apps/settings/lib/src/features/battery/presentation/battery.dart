import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_bloc.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_state.dart';
import 'package:mechanix_settings/src/features/battery/models/types.dart';
import 'package:mechanix_settings/src/features/battery/presentation/battery_indicator.dart';
import 'package:mechanix_settings/src/features/battery/presentation/battery_performance.dart';
import 'package:upower/upower.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/list_items/mechanix_simple_list_theme.dart';
import 'package:widgets/widgets/list_items/simple_list_items_type.dart';

class Battery extends StatefulWidget {
  const Battery({super.key});

  @override
  State<Battery> createState() => BatteryScreenState();
}

class BatteryScreenState extends State<Battery> {
  String timeText = '';
  void _onPerformanceTap() {
    final bloc = context.read<BatteryBloc>();

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => BlocProvider.value(
          value: bloc,
          child: const BatteryPerformance(),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<BatteryBloc, BatteryState>(
      builder: (context, state) {
        final batteryPercentageValue = state.batteryPercentage.toInt();
        final totalSeconds = UPowerDeviceState.charging == state.batteryStatus
            ? state.batteryChargingTime ?? 0
            : state.batteryRemainingTime ?? 0;

        final hours = totalSeconds ~/ 3600;
        final minutes = (totalSeconds % 3600) ~/ 60;

        if (hours > 0 || minutes > 0) {
          timeText = [
            if (hours > 0) "$hours hrs",
            if (minutes > 0) "$minutes mins"
          ].join(' ');

          if (state.batteryStatus == UPowerDeviceState.charging) {
            timeText = "$timeText to full charge";
          } else {
            timeText = "$timeText left";
          }
        }

        return Scaffold(
          body: SingleChildScrollView(
            physics: const BouncingScrollPhysics(),
            child: ContainerWidget(
              child: Column(
                children: [
                  const CustomTitle(title: "Battery"),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      Text(
                        timeText,
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
                  MechanixSimpleList(
                      theme: const MechanixSimpleListThemeData(
                        widgetMargin: EdgeInsets.only(bottom: 12),
                      ),
                      listItems: [
                        SimpleListItems(
                            title: 'System Performance',
                            // onTap: () => Navigator.pushNamed(
                            //     context, AppRoutes.batteryPerformance),
                            onTap: _onPerformanceTap,
                            trailing: Row(
                              children: [
                                CustomTrailingText(
                                        title: getModeDetails(
                                                state.performanceMode ?? '')
                                            .mode)
                                    .padRight(8),
                                const IconWidget(
                                  iconWidth: 9,
                                  iconHeight: 18,
                                  iconPath: Images.rightIconArrow,
                                )
                              ],
                            ))
                      ]).padTop(40),
                  CustomTrailingText(
                    title:
                        getModeDetails(state.performanceMode ?? '').content ??
                            '',
                    textAlign: TextAlign.left,
                    titleStyle: const TextStyle(
                        fontWeight: FontWeight.w400, fontSize: 16),
                  )
                ],
              ).padTop(8),
            ),
          ),
          bottomNavigationBar: MechanixBottomBar(
            leadingWidget: [context.backButton],
          ),
        );
      },
    );
  }
}
