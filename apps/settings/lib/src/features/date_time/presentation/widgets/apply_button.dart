import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_bloc.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_event.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_state.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/filled_button/mechanix_filled_button_theme.dart';

class ApplyButton extends StatefulWidget {
  const ApplyButton({
    super.key,
  });

  @override
  State<ApplyButton> createState() => _ApplyButtonState();
}

class _ApplyButtonState extends State<ApplyButton> {
  void onPressed(DateTimeState state) {
    DateTime newDateTime = DateTime(
      state.selectedYear,
      state.selectedMonth,
      state.selectedDate,
      state.selectedMeridiem == 'PM'
          ? state.selectedHour + 12
          : state.selectedHour,
      state.selectedMinute,
    );

    context.read<DateTimeBloc>().add(UpdateSystemDateTimeEvent(
        dateTime: newDateTime, timeZone: state.selectedTimezone));
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<DateTimeBloc, DateTimeState>(
      builder: (context, state) {
        return MechanixFilledButton(
          label: "Save",
          theme: const MechanixFilledButtonThemeData(buttonSize: Size(64, 44)),
          onPressed: () => onPressed(state),
        ).padOnly(right: 20);
      },
    );
  }
}
