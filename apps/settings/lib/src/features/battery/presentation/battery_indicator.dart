import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_bloc.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_state.dart';
import 'package:widgets/mechanix.dart';

import '../models/types.dart';

class BatteryIndicator extends StatelessWidget {
  final double height;
  final bool isCharging;
  final double tipHeight;
  final double tipWidth;

  const BatteryIndicator({
    super.key,
    required this.isCharging,
    this.height = 72,
    this.tipHeight = 28.0,
    this.tipWidth = 8.0,
  });

  Color _getBatteryColor(BatteryState state) {
    if (state.batteryPercentage > 20) {
      return getModeDetails(state.performanceMode ?? '').color;
    }
    return const Color(0xFFB90C2C);
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<BatteryBloc, BatteryState>(
      builder: (context, state) {
        return LayoutBuilder(
          builder: (context, constraints) {
            final double availableWidth = constraints.maxWidth;
            final double batteryWidth =
                availableWidth - tipWidth; // Subtract space for battery tip

            return SizedBox(
              width: availableWidth,
              height: height,
              child: Row(
                children: [
                  Container(
                    width: batteryWidth,
                    height: height,
                    padding: EdgeInsets.all(8),
                    decoration: BoxDecoration(
                      borderRadius: BorderRadius.circular(8),
                      color: context.colorScheme.secondary,
                    ),
                    child: ClipRRect(
                      child: Stack(
                        children: [
                          AnimatedContainer(
                            duration: const Duration(milliseconds: 700),
                            width:
                                batteryWidth * (state.batteryPercentage / 100),
                            height: height,
                            decoration: BoxDecoration(
                              color: _getBatteryColor(state),
                            ),
                          ),
                          // Percentage text
                          const Center(
                            child: IconWidget(
                                boxWidth: 24,
                                boxHeight: 28,
                                iconWidth: 17,
                                iconHeight: 26,
                                iconPath: Images.chargingIcon),
                          ),
                        ],
                      ),
                    ),
                  ),

                  // Battery tip
                  Container(
                    width: tipWidth,
                    height: tipHeight,
                    decoration: BoxDecoration(
                      color: context.colorScheme.secondary,
                      borderRadius: HorizontalRadius.rightAll(2),
                    ),
                  ),
                ],
              ),
            );
          },
        );
      },
    );
  }
}
