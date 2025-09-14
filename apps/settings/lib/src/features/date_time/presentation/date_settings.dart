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
import 'package:widgets/widgets/wheelScroll/wheel_scroll_options_type.dart';

class DateSettings extends StatefulWidget {
  const DateSettings({super.key});

  @override
  State<DateSettings> createState() => _DateSettingsState();
}

class _DateSettingsState extends State<DateSettings> {
  @override
  Widget build(BuildContext context) {
    return BlocBuilder<DateTimeBloc, DateTimeState>(
      builder: (context, state) {
        List<WheelScrollOption<int>> totalDays = getTotalDays(
          selectedMonth: state.selectedMonth,
          selectedYear: state.selectedYear,
        );

        final weekDay = longWeekdays
            .firstWhere((e) => e.value == state.selectedWeekDay)
            .label;

        final month =
            longMonths.firstWhere((e) => e.value == state.selectedMonth).label;

        return Scaffold(
          appBar: MechanixNavigationBar(title: "Set Date"),
          body: SingleChildScrollView(
            child: ContainerWidget(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  CustomTrailingText(
                          title:
                              '${state.selectedDate} $month ${state.selectedYear}, $weekDay')
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
                              value: state.selectedDate,
                              options: totalDays,
                              theme: MechanixWheelScrollThemeData(
                                  selectionBorderRadius:
                                      HorizontalRadius.leftAll(8)),
                              onSelectedItemChanged: (value) {
                                context
                                    .read<DateTimeBloc>()
                                    .add(SetSelectedDateEvent(value));
                              },
                            ),
                            MechanixWheelScroll(
                              width: 116,
                              height: 220,
                              selectionWidth: 116,
                              selectionHeight: 56,
                              value: state.selectedMonth,
                              options: shortMonths,
                              theme: MechanixWheelScrollThemeData(
                                  selectionBorderRadius: CircularRadius.all(0)),
                              onSelectedItemChanged: (value) {
                                context
                                    .read<DateTimeBloc>()
                                    .add(SetSelectedMonthEvent(value));
                              },
                            ),

                            // // Year wheel (1975..now+10)
                            MechanixWheelScroll(
                              width: 116,
                              height: 220,
                              selectionWidth: 116,
                              selectionHeight: 56,
                              value: state.selectedYear,
                              theme: MechanixWheelScrollThemeData(
                                  selectionBorderRadius:
                                      HorizontalRadius.rightAll(8)),
                              options: yearOptions,
                              onSelectedItemChanged: (value) {
                                setState(() {
                                  context
                                      .read<DateTimeBloc>()
                                      .add(SetSelectedYearEvent(value));
                                });
                              },
                            ),
                          ],
                        ),
                      ),

                      // Weekday wheel (Mon..Sun)
                      MechanixWheelScroll(
                        width: 116,
                        height: 220,
                        selectionWidth: 116,
                        selectionHeight: 56,
                        value: state.selectedWeekDay,
                        options: shortWeekdays,
                        scrollEnabled: false,
                        theme: MechanixWheelScrollThemeData(
                            selectionColor: Colors.transparent),
                        onSelectedItemChanged: (value) {
                          context
                              .read<DateTimeBloc>()
                              .add(SetSelectedWeekDayEvent(value));
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
