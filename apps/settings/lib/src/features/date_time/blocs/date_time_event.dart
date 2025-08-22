// features/network/presentation/bloc/wireless_settings_event.dart

import 'package:equatable/equatable.dart';

abstract class DateTimeEvent extends Equatable {
  @override
  List<Object?> get props => [];
}

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

