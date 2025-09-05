import 'package:flutter/material.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_icon.dart';
import 'package:mechanix_settings/src/commons/styles/custom_styles.dart';
import 'package:mechanix_settings/src/features/battery/models/types.dart';
import 'package:mechanix_settings/src/features/battery/presentation/battery_indicator.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_bloc.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_state.dart';
import 'package:upower/upower.dart';

class Battery extends StatefulWidget {
  const Battery({super.key});

  @override
  State<Battery> createState() => BatteryScreenState();
}

class BatteryScreenState extends State<Battery> {
  void _backNavigation() {
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<BatteryBloc, BatteryState>(
      builder: (context, state) {
        final batteryPercentageValue = state.batteryPercentage.toInt();
        final totalSeconds = UPowerDeviceState.charging == state.status
            ? state.batteryChargingTime ?? 0
            : state.batteryRemainingTime ?? 0;

        final hours = totalSeconds ~/ 3600;
        final minutes = (totalSeconds % 3600) ~/ 60;

        final timeText = [
          if (hours > 0) "$hours hrs",
          if (minutes > 0) "$minutes mins"
        ].join(' ');

        final displayText = state.status == UPowerDeviceState.charging
            ? "$batteryPercentageValue% charging, $timeText until full charge"
            : "$batteryPercentageValue% remaining, $timeText";

        return Scaffold(
          appBar: CustomAppBar(
            title: "Battery",
            leftIcon: Image.asset(Images.back),
            leftIconOnTap: _backNavigation,
          ),
          body: ContainerWidget(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              spacing: 10,
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Text(
                      "Battery",
                      style: baseHeaderStyle.copyWith(fontSize: 20),
                    ),
                    Container(
                        margin: EdgeInsets.only(right: 30),
                        child: Text(
                          displayText,
                          style: baseHeaderStyle.copyWith(
                            fontWeight: FontWeight.w400,
                          ),
                        ))
                  ],
                ),
                BatteryIndicator(
                  width: MediaQuery.of(context).size.width - 70,
                  batteryPercentage: batteryPercentageValue,
                  isCharging: UPowerDeviceState.charging == state.status,
                ),
                const SizedBox(
                  height: 40,
                ),
                FixedHeightRow(
                  showTopBorder: false,
                  showBottomBorder: false,
                  child: ListTile(
                    title: const Text(
                      "Performance mode",
                      style: baseHeaderStyle,
                    ),
                    trailing: Row(
                      mainAxisAlignment: MainAxisAlignment.end,
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        if (batteryModes[state.mode] != null)
                          Text(
                            batteryModes[state.mode]!.mode,
                            style: secondaryHeaderStyle,
                          ),
                        const SizedBox(
                          width: 10,
                        ),
                        CustomIcon(
                          icon: Image.asset(Images.rightIconArrow),
                          height: 18,
                          width: 18,
                        ),
                      ],
                    ),
                    onTap: () {
                      Navigator.pushNamed(
                          context, AppRoutes.batteryPerformance);
                    },
                  ),
                ),
                if (batteryModes[state.mode] != null)
                  Text(
                    batteryModes[state.mode]!.content,
                    style: baseHeaderStyle,
                  )
              ],
            ),
          ),
        );
      },
    );
  }
}
