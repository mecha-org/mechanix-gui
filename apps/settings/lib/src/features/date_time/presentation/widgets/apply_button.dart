import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_bloc.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_event.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_state.dart';
import 'package:widgets/mechanix.dart';

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
      state.selectedHour,
      state.selectedMinute,
    );
    context.read<DateTimeBloc>().add(UpdateSystemDateTimeEvent(newDateTime));
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<DateTimeBloc, DateTimeState>(
      builder: (context, state) {
        return SizedBox(
          width: double.infinity,
          child: FilledButton(
            onPressed: () => onPressed(state),
            style: ButtonStyle(
              backgroundColor: WidgetStateProperty.all<Color>(
                context.colorScheme.secondary,
              ),
            ),
            child: Text(
              'Apply',
              style: TextStyle(color: context.colorScheme.onSurface),
            ).padVertical(16),
          ).padOnly(top: 87, left: 84, right: 84),
        );
      },
    );
  }
}
