import 'package:equatable/equatable.dart';

class DateTimeState extends Equatable {
  final bool autoDateTime;
  final bool autoTimeZone;
  // final List<WheelScrollOption<String>> listTimezones;
  final bool loading;
  final String? error;
  final DateTime? systemDateTime;

  final int selectedDate;
  final int selectedMonth;
  final int selectedYear;
  final int selectedWeekDay;
  final int selectedHour;
  final int selectedMinute;
  final String selectedMeridiem;
  final String selectedTimezone;

  const DateTimeState({
    required this.autoDateTime,
    required this.autoTimeZone,
    // this.listTimezones = const [],
    this.loading = false,
    this.error,
    this.systemDateTime,
    required this.selectedYear,
    required this.selectedMonth,
    required this.selectedDate,
    required this.selectedHour,
    required this.selectedMinute,
    required this.selectedWeekDay,
    this.selectedMeridiem = '',
    this.selectedTimezone = "Asia/Kolkata",
  });

  DateTimeState copyWith({
    bool? autoDateTime,
    bool? autoTimeZone,
    // List<WheelScrollOption<String>>? listTimezones,
    bool? loading,
    String? error,
    DateTime? systemDateTime,
    int? selectedDate,
    int? selectedMonth,
    int? selectedYear,
    int? selectedWeekDay,
    int? selectedHour,
    int? selectedMinute,
    String? selectedMeridiem,
    String? selectedTimezone,
  }) {
    return DateTimeState(
      autoDateTime: autoDateTime ?? this.autoDateTime,
      autoTimeZone: autoTimeZone ?? this.autoTimeZone,
      // listTimezones: listTimezones ?? this.listTimezones,
      loading: loading ?? this.loading,
      error: error,
      systemDateTime: systemDateTime ?? this.systemDateTime,
      selectedDate: selectedDate ?? this.selectedDate,
      selectedMonth: selectedMonth ?? this.selectedMonth,
      selectedYear: selectedYear ?? this.selectedYear,
      selectedWeekDay: selectedWeekDay ?? this.selectedWeekDay,
      selectedHour: selectedHour ?? this.selectedHour,
      selectedMinute: selectedMinute ?? this.selectedMinute,
      selectedMeridiem: selectedMeridiem ?? this.selectedMeridiem,
      selectedTimezone: selectedTimezone ?? this.selectedTimezone,
    );
  }

  @override
  List<Object?> get props => [
        autoDateTime,
        autoTimeZone,
        // listTimezones,
        loading,
        error,
        systemDateTime,
        selectedDate,
        selectedMonth,
        selectedYear,
        selectedWeekDay,
        selectedHour,
        selectedMinute,
        selectedMeridiem,
        selectedTimezone,
      ];
}
