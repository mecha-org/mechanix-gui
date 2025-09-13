import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:intl/intl.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_bloc.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_event.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_state.dart';

class DateSettings extends StatelessWidget {
  const DateSettings({super.key});

  void backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  void pickDate(BuildContext context, DateTime systemDateTime) async {
    final selectedDate = systemDateTime.toLocal();

    final dateTimeBloc = context.read<DateTimeBloc>();

    final picked = await showDatePicker(
      context: context,
      initialDate: selectedDate,
      firstDate: DateTime(2000),
      lastDate: DateTime(2100),
      // initialEntryMode: DatePickerEntryMode.inputOnly, // input field only
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

    if (picked != null && picked != selectedDate) {
      var formattedTime = dateTimeToMillisSinceEpoch(picked, systemDateTime);
      dateTimeBloc.add(SetTime(formattedTime));
    }
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<DateTimeBloc, DateTimeState>(
      builder: (context, state) {
        final currentDate = state.dateTime ?? DateTime.now();

        final formattedDate = DateFormat('dd / MM / yyyy').format(currentDate);

        return Scaffold(
          appBar: CustomAppBar(
            title: "Set Date",
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
                    title: const Text(
                      "Date format is dd/ mm/ yyyy",
                      style: TextStyle(color: Colors.white, fontSize: 20),
                    ),
                  ),
                ),
                const SizedBox(height: 20),
                GestureDetector(
                  onTap: () => pickDate(context, currentDate),
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
                        formattedDate,
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

int dateTimeToMillisSinceEpoch(
  DateTime dateTime,
  DateTime systemDateTime,
) {
  final combined = DateTime(
    dateTime.year,
    dateTime.month,
    dateTime.day,
    systemDateTime.hour,
    systemDateTime.minute,
    systemDateTime.second,
    systemDateTime.millisecond,
    systemDateTime.microsecond,
  );
  return combined.microsecondsSinceEpoch;
}
