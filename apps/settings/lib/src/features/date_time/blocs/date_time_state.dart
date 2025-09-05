
import 'package:equatable/equatable.dart';

class DateTimeState extends Equatable {
  final bool autoDateTime;
  final bool autoTimeZone;
  final String? currentTimeZone;
  final DateTime? dateTime;
  final List<String>? listTimezones;
  final bool loading;
  final String? error;

  const DateTimeState({
    required this.autoDateTime,
    required this.autoTimeZone,
    this.currentTimeZone,
    this.dateTime,
    this.listTimezones,
    this.loading = false,
    this.error,
  });

  DateTimeState copyWith({
    bool? autoDateTime,
    bool? autoTimeZone,
    String? currentTimeZone,
    DateTime? dateTime,
    List<String>? listTimezones,
    bool? loading,
    String? error,
  }) {
    return DateTimeState(
      autoDateTime: autoDateTime ?? this.autoDateTime,
      autoTimeZone: autoTimeZone ?? this.autoTimeZone,
      currentTimeZone: currentTimeZone ?? this.currentTimeZone,
      dateTime: dateTime,
      listTimezones: listTimezones ?? this.listTimezones,
      loading: loading ?? this.loading,
      error: error,
    );
  }

  @override
  List<Object?> get props => [autoDateTime, autoTimeZone, currentTimeZone, dateTime, listTimezones, loading, error];
}

 