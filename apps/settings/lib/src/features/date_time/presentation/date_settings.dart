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
import 'package:widgets/widgets/wheel_scroll/wheel_scroll_options_type.dart';

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

        return Scaffold(
          body: SingleChildScrollView(
            child: ContainerWidget(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const CustomTitle(title: "Set Date"),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      SizedBox(
                        child: Row(
                          mainAxisAlignment: MainAxisAlignment.spaceBetween,
                          children: [
                            MechanixWheelScroll(
                              width: 74,
                              squeeze: 0.75,
                              height: 248,
                              selectionWidth: 74,
                              selectionHeight: 60,
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
                            ).padRight(24),
                            MechanixWheelScroll(
                              width: 74,
                              squeeze: 0.75,
                              height: 248,
                              selectionWidth: 74,
                              selectionHeight: 60,
                              value: state.selectedMonth,
                              options: shortMonths,
                              theme: MechanixWheelScrollThemeData(
                                  selectionBorderRadius: CircularRadius.all(0)),
                              onSelectedItemChanged: (value) {
                                context
                                    .read<DateTimeBloc>()
                                    .add(SetSelectedMonthEvent(value));
                              },
                            ).padRight(24),

                            // // Year wheel (1975..now+10)
                            MechanixWheelScroll(
                              width: 108,
                              height: 248,
                              squeeze: 0.75,
                              selectionWidth: 108,
                              selectionHeight: 60,
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
                        width: 74,
                        squeeze: 0.75,
                        height: 248,
                        selectionWidth: 74,
                        selectionHeight: 60,
                        value: state.selectedWeekDay,
                        options: shortWeekdays,
                        // scrollEnabled: false,
                        theme: MechanixWheelScrollThemeData(
                            selectionColor: Colors.transparent),
                        onSelectedItemChanged: (value) {
                          context
                              .read<DateTimeBloc>()
                              .add(SetSelectedWeekDayEvent(value));
                        },
                      ),
                    ],
                  ).padOnly(top: 100, right: 34, left: 34),
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
