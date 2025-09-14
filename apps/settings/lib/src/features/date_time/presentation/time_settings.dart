import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_bloc.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_event.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_state.dart';
import 'package:mechanix_settings/src/features/date_time/models/types.dart';
import 'package:mechanix_settings/src/features/date_time/presentation/widgets/apply_button.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/wheelScroll/mechanix_wheel_scroll_theme.dart';

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
        final currentTimeZoneAbbr = state.listTimezones.isNotEmpty
            ? state.listTimezones.firstWhere((t) {
                return t.value == state.selectedTimezone;
              }).label
            : '';

        final hour = hours12Options
            .firstWhere((e) => e.value == state.selectedHour)
            .label;

        final minute = minutesOptions
            .firstWhere((e) => e.value == state.selectedMinute)
            .label;

        return Scaffold(
          appBar: MechanixNavigationBar(title: "Set time"),
          body: SingleChildScrollView(
            child: ContainerWidget(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  CustomTrailingText(
                          title:
                              '$hour :$minute ${state.selectedMeridiem}, $currentTimeZoneAbbr')
                      .padOnly(top: 8, bottom: 40),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      SizedBox(
                        width: 348,
                        height: 284,
                        child: Row(
                          children: [
                            MechanixWheelScroll(
                              width: 116,
                              height: 220,
                              selectionWidth: 116,
                              selectionHeight: 56,
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
                            MechanixWheelScroll(
                              width: 116,
                              height: 220,
                              selectionWidth: 116,
                              selectionHeight: 56,
                              value: state.selectedMinute,
                              options: minutesOptions,
                              theme: MechanixWheelScrollThemeData(
                                  selectionBorderRadius: CircularRadius.all(0)),
                              onSelectedItemChanged: (value) {
                                context
                                    .read<DateTimeBloc>()
                                    .add(SetSelectedMinuteEvent(value));
                              },
                            ),
                            MechanixWheelScroll(
                              width: 116,
                              height: 220,
                              selectionWidth: 116,
                              selectionHeight: 56,
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
                      MechanixWheelScroll(
                        width: 124,
                        height: 220,
                        selectionWidth: 116,
                        selectionHeight: 56,
                        value: state.selectedTimezone,
                        options: state.listTimezones,
                        onSelectedItemChanged: (value) {
                          context
                              .read<DateTimeBloc>()
                              .add(SetSelectedTimezoneEvent(value));
                        },
                      ),
                    ],
                  ),
                  ApplyButton()
                ],
              ),
            ),
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
