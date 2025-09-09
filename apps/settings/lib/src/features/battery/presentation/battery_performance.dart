import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/styles/custom_styles.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_bloc.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_event.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_state.dart';
import 'package:mechanix_settings/src/features/battery/models/types.dart';

class BatteryPerformance extends StatefulWidget {
  const BatteryPerformance({super.key});

  @override
  State<BatteryPerformance> createState() => BatteryPerformanceState();
}

class BatteryPerformanceState extends State<BatteryPerformance> {
  String? selectedMode;
  void _backNavigation() {
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<BatteryBloc, BatteryState>(builder: (context, state) {
      selectedMode = state.mode;
      return Scaffold(
        appBar: CustomAppBar(
          title: "Battery",
          leftIcon: Image.asset(Images.back),
          leftIconOnTap: _backNavigation,
        ),
        body: ContainerWidget(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                "Performance Mode",
                style: baseHeaderStyle.copyWith(fontSize: 20),
              ),
              const SizedBox(height: 16),
              Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
                ListView.builder(
                    shrinkWrap: true,
                    physics: NeverScrollableScrollPhysics(),
                    itemCount: batteryModes.length,
                    itemBuilder: (context, index) {
                      var bModeKey = batteryModes.keys.elementAt(index);
                      var bModeData = batteryModes[bModeKey]!;
                      return Container(
                        height: 48,
                        padding: EdgeInsets.symmetric(horizontal: 10),
                        alignment: Alignment.center,
                        decoration: _addBoxDecorationStyle(index),
                        child: RadioListTile<String>(
                          title: Text(
                            bModeData.mode,
                            style: baseHeaderStyle,
                          ),
                          controlAffinity: ListTileControlAffinity.trailing,
                          value: bModeKey,
                          groupValue: selectedMode,
                          onChanged: (value) {
                            setState(() {
                              if (value != null && value != '') {
                                context
                                    .read<BatteryBloc>()
                                    .add(SetBatteryMode(value));
                              }

                              selectedMode = value;
                            });
                          },
                          activeColor: Color(0xFF2D8AFF),
                          materialTapTargetSize:
                              MaterialTapTargetSize.shrinkWrap,
                          contentPadding: EdgeInsets.symmetric(horizontal: 10),
                          dense: true,
                        ),
                      );
                    }),
                const SizedBox(
                  height: 10,
                ),
                if (selectedMode != null &&
                    selectedMode != '' &&
                    batteryModes[selectedMode] != null)
                  Text(
                    batteryModes[selectedMode]!.content,
                    style: baseHeaderStyle,
                  )
              ])
            ],
          ),
        ),
      );
    });
  }
}

BoxDecoration _addBoxDecorationStyle(int index) {
  if (index + 1 == batteryModes.length) {
    return rowBoxDecoration.copyWith(
      borderRadius: BorderRadius.only(
          bottomLeft: Radius.circular(8),
          bottomRight: Radius.circular(8),
          topRight: Radius.circular(0),
          topLeft: Radius.circular(0)),
    );
  }

  if (index == 0) {
    return rowBoxDecoration.copyWith(
      border: Border(bottom: BorderSide(width: 1, color: Color(0xFF444444))),
      borderRadius: BorderRadius.only(
          bottomLeft: Radius.circular(0),
          bottomRight: Radius.circular(0),
          topRight: Radius.circular(8),
          topLeft: Radius.circular(8)),
    );
  }
  return rowBoxDecoration.copyWith(
    borderRadius: BorderRadius.zero,
    border: Border(bottom: BorderSide(width: 1, color: Color(0xFF444444))),
  );
}
