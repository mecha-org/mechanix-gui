import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_bloc.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_event.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_state.dart';
import 'package:mechanix_settings/src/features/date_time/models/types.dart';
import 'package:mechanix_settings/src/features/date_time/presentation/widgets/apply_button.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/wheel_scroll/mechanix_wheel_scroll_theme.dart';

class TimeSettings extends StatefulWidget {
  const TimeSettings({super.key});

  @override
  State<TimeSettings> createState() => _TimeSettingsState();
}

class _TimeSettingsState extends State<TimeSettings> {
  void backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<DateTimeBloc, DateTimeState>(
      builder: (context, state) {
        return Scaffold(
          body: SingleChildScrollView(
            child: ContainerWidget(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const CustomTitle(title: "Set Time"),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    crossAxisAlignment: CrossAxisAlignment.center,
                    children: [
                      SizedBox(
                        child: Row(
                          children: [
                            MechanixWheelScroll(
                              width: 74,
                              squeeze: 0.75,
                              height: 248,
                              selectionWidth: 74,
                              selectionHeight: 60,
                              value: state.selectedHour,
                              options: hours12Options,
                              theme: MechanixWheelScrollThemeData(
                                  selectionBorderRadius:
                                      HorizontalRadius.leftAll(8)),
                              onSelectedItemChanged: (value) {
                                context
                                    .read<DateTimeBloc>()
                                    .add(SetSelectedHourEvent(value));
                              },
                            ),
                            // const CustomTrailingText(title: ":"),
                            const Text(":").padHorizontal(16),
                            MechanixWheelScroll(
                              width: 74,
                              squeeze: 0.75,
                              height: 248,
                              selectionWidth: 74,
                              selectionHeight: 60,
                              value: state.selectedMinute,
                              options: minutesOptions,
                              theme: MechanixWheelScrollThemeData(
                                  selectionBorderRadius: CircularRadius.all(0)),
                              onSelectedItemChanged: (value) {
                                context
                                    .read<DateTimeBloc>()
                                    .add(SetSelectedMinuteEvent(value));
                              },
                            ).padRight(36),
                            MechanixWheelScroll(
                              width: 74,
                              squeeze: 0.75,
                              height: 248,
                              selectionWidth: 74,
                              selectionHeight: 60,
                              value: state.selectedMeridiem,
                              options: timeMeridiemOptions,
                              theme: MechanixWheelScrollThemeData(
                                  selectionBorderRadius:
                                      HorizontalRadius.rightAll(8)),
                              offAxisFraction: 1,
                              isLoop: false,
                              onSelectedItemChanged: (value) {
                                context
                                    .read<DateTimeBloc>()
                                    .add(SetSelectedMeridiemEvent(value));
                              },
                            ),
                          ],
                        ),
                      ),
                    ],
                  ).padTop(100),
                ],
              ).padTop(8),
            ),
          ),
          bottomNavigationBar: MechanixBottomBar(
            leadingWidget: [context.backButton],
            anchorWidget: const [BottomBarButton.widget(widget: ApplyButton())],
          ),
        );
      },
    );
  }
}

int timeOfDayToMillisSinceEpoch(
  DateTime dateTime,
  TimeOfDay timeOfDay,
) {
  final combined = DateTime(
    dateTime.year,
    dateTime.month,
    dateTime.day,
    timeOfDay.hour,
    timeOfDay.minute,
  );
  return combined.microsecondsSinceEpoch;
}
