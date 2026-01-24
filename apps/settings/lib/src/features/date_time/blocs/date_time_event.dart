// features/network/presentation/bloc/wireless_settings_event.dart

import 'package:equatable/equatable.dart';

abstract class DateTimeEvent extends Equatable {
  @override
  List<Object?> get props => [];
}

class InitializeDateTime extends DateTimeEvent {}

class GetDateTimeData extends DateTimeEvent {}

class ToggleAutoDateTime extends DateTimeEvent {
  final bool enabled;

  ToggleAutoDateTime(this.enabled);

  @override
  List<Object?> get props => [enabled];
}

class ToggleAutoTimeZone extends DateTimeEvent {
  final bool enabled;

  ToggleAutoTimeZone(this.enabled);

  @override
  List<Object?> get props => [enabled];
}

class SetTimeZone extends DateTimeEvent {
  final String timezone;

  SetTimeZone(this.timezone);

  @override
  List<Object?> get props => [timezone];
}

class SetTime extends DateTimeEvent {
  final int timeMicrosecondsSinceEpoch;
  SetTime(this.timeMicrosecondsSinceEpoch);

  @override
  List<Object?> get props => [timeMicrosecondsSinceEpoch];
}

class UpdateSystemDateTimeEvent extends DateTimeEvent {
  final DateTime dateTime;
  final String timeZone;

  UpdateSystemDateTimeEvent({required this.dateTime, required this.timeZone});

  @override
  List<Object?> get props => [
        dateTime,
        timeZone,
      ];
}

class SetSelectedDateEvent extends DateTimeEvent {
  final int date;
  SetSelectedDateEvent(this.date);
}

class SetSelectedMonthEvent extends DateTimeEvent {
  final int month;
  SetSelectedMonthEvent(this.month);
}

class SetSelectedYearEvent extends DateTimeEvent {
  final int year;
  SetSelectedYearEvent(this.year);
}

class SetSelectedWeekDayEvent extends DateTimeEvent {
  final int day;
  SetSelectedWeekDayEvent(this.day);
}

class SetSelectedHourEvent extends DateTimeEvent {
  final int selectedHour;
  SetSelectedHourEvent(this.selectedHour);
}

class SetSelectedMinuteEvent extends DateTimeEvent {
  final int selectedMinute;
  SetSelectedMinuteEvent(this.selectedMinute);
}

class SetSelectedMeridiemEvent extends DateTimeEvent {
  final String selectedMeridiem; // 0 for AM, 1 for PM
  SetSelectedMeridiemEvent(this.selectedMeridiem);
}

class SetSelectedTimezoneEvent extends DateTimeEvent {
  final String selectedTimezone; // Index of the timezone in your list
  SetSelectedTimezoneEvent(this.selectedTimezone);
}
