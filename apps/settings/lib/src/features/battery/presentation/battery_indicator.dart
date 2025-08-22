import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_icon.dart';

class BatteryIndicator extends StatelessWidget {
  final int batteryPercentage;

  final double width;

  final double height;

  final bool isCharging;

  const BatteryIndicator({
    super.key,
    required this.batteryPercentage,
    required this.isCharging,
    this.width = 300,
    this.height = 55,
  });

  Color _getBatteryColor(int percentage) {
    if (percentage > 20) {
      return Color(0xFF34C759);
    }
    return Color(0xFFB90C2C);
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: width + 15,
      height: height,
      child: Row(
        children: [
          Container(
            width: width,
            height: height,
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(8),
              border: Border.all(color: Color(0xFF777777), width: 2),
              color: Color(0xFF151515),
              boxShadow: [
                BoxShadow(
                  color: Color(0x66000000), // #00000040 → 40 hex = ~25% opacity
                  offset: Offset(0, 4),
                  blurRadius: 16,
                  spreadRadius: 0,
                ),
                BoxShadow(
                  color: Color(0xFF151515),
                  offset: Offset(-4, 4),
                  blurRadius: 16,
                  spreadRadius: 0,
                ),
                BoxShadow(
                  color: Color(0xFF151515),
                  offset: Offset(4, -4),
                  blurRadius: 16,
                  spreadRadius: 0,
                ),
              ],
            ),
            child: ClipRRect(
              borderRadius: BorderRadius.circular(8 - 2),
              child: Stack(
                children: [
                  AnimatedContainer(
                    duration: Duration(milliseconds: 300),
                    width: width * (batteryPercentage / 100),
                    height: height,
                    decoration: BoxDecoration(
                      color: _getBatteryColor(batteryPercentage),
                    ),
                  ),
                  // Percentage text
                  Center(
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        if (isCharging)
                          CustomIcon(icon: Image.asset(Images.chargingIcon)),
                        Text(
                          '$batteryPercentage%',
                          style: TextStyle(
                            color: Colors.white,
                            fontSize: height * 0.35,
                            fontWeight: FontWeight.bold,
                            shadows: [
                              Shadow(
                                offset: Offset(1, 1),
                                blurRadius: 2,
                                color: Colors.black54,
                              ),
                            ],
                          ),
                        )
                      ],
                    ),
                  ),
                ],
              ),
            ),
          ),

          // Battery tip
          Container(
            width: 10,
            height: height * 0.6,
            margin: EdgeInsets.only(left: 3),
            decoration: BoxDecoration(
              color: Colors.grey[600],
              borderRadius: BorderRadius.only(
                topRight: Radius.circular(8 / 2),
                bottomRight: Radius.circular(8 / 2),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
