import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:intl/intl.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_bloc.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_event.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_state.dart';

class TimeSettings extends StatelessWidget {
  const TimeSettings({super.key});

  void backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  void pickTime(BuildContext context, DateTime selectedDateTime) async {
    final localTime = selectedDateTime.toLocal();

    // Convert local time to TimeOfDay for the time picker
    final selectedTime =
        TimeOfDay(hour: localTime.hour, minute: localTime.minute);

    final dateTimeBloc = context.read<DateTimeBloc>();

    final picked = await showTimePicker(
      context: context,
      initialTime: selectedTime,
      initialEntryMode: TimePickerEntryMode.inputOnly,
      builder: (context, child) {
        return Theme(
          data: Theme.of(context).copyWith(
            colorScheme: const ColorScheme.dark(
              primary: Colors.blueAccent,
              onSurface: Colors.white,
              surface: Color(0xFF0A0A1A),
            ),
            textButtonTheme: TextButtonThemeData(
              style: TextButton.styleFrom(foregroundColor: Colors.blueAccent),
            ),
          ),
          child: child!,
        );
      },
    );

    if (picked != null && picked != selectedTime) {
      var formattedTime = timeOfDayToMillisSinceEpoch(localTime, picked);
      dateTimeBloc.add(SetTime(formattedTime));
    }
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<DateTimeBloc, DateTimeState>(
      builder: (context, state) {
        final currentTime = state.dateTime ?? DateTime.now();
        final formattedTime =
            DateFormat('hh:mm a').format(currentTime.toLocal());

        return Scaffold(
          appBar: CustomAppBar(
            title: "Set time",
            leftIcon: Image.asset(Images.back),
            leftIconOnTap: () => backNavigation(context),
          ),
          body: ContainerWidget(
            child: Column(
              children: [
                FixedHeightRow(
                  showTopBorder: false,
                  showBottomBorder: false,
                  child: ListTile(
                    title: const Text("Time set according to the standard time",
                        style: TextStyle(color: Colors.white, fontSize: 20)),
                  ),
                ),
                const SizedBox(height: 20),
                GestureDetector(
                  onTap: () => pickTime(context, currentTime),
                  child: FractionallySizedBox(
                    widthFactor: 1.0,
                    child: Container(
                      padding: const EdgeInsets.symmetric(vertical: 16),
                      decoration: BoxDecoration(
                        border: Border.all(color: Colors.blueAccent),
                        borderRadius: BorderRadius.circular(8),
                      ),
                      alignment: Alignment.center,
                      child: Text(
                        formattedTime,
                        style: const TextStyle(
                          fontSize: 28,
                          letterSpacing: 8,
                          color: Colors.white,
                        ),
                      ),
                    ),
                  ),
                ),
              ],
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
