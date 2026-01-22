import 'package:widgets/widgets/select/select_type.dart';
import 'package:widgets/widgets/wheel_scroll/wheel_scroll_options_type.dart';

final List<WheelScrollOption<int>> hours12Options = List.generate(12, (index) {
  final hour = index + 1;
  return WheelScrollOption<int>(
    label: hour.toString().padLeft(2, '0'),
    value: hour,
  );
});

final List<WheelScrollOption<int>> hours24Options = List.generate(
    24,
    (index) => WheelScrollOption<int>(
          label: index.toString().padLeft(2, '0'),
          value: index,
        ));

final List<WheelScrollOption<int>> minutesOptions = List.generate(
    60,
    (index) => WheelScrollOption<int>(
          label: index.toString().padLeft(2, '0'),
          value: index,
        ));

final List<WheelScrollOption<String>> timeMeridiemOptions = [
  const WheelScrollOption(label: 'AM', value: 'AM'),
  const WheelScrollOption(label: 'PM', value: 'PM'),
];

final List<int> days = List.generate(31, (i) => i + 1);

final List<WheelScrollOption<int>> yearOptions = List.generate(
  (DateTime.now().year + 100) - (DateTime.now().year - 100) + 1,
  (index) {
    final year = DateTime.now().year - 100 + index;
    return WheelScrollOption(label: year.toString(), value: year);
  },
);

int hour24to12(int hour) {
  if (hour == 0 || hour == 12) return 12;
  if (hour < 12) return hour;
  return hour - 12;
}

List<WheelScrollOption<int>> getTotalDays({
  required int selectedYear,
  required int selectedMonth,
}) =>
    getDaysInMonth(
      selectedYear,
      selectedMonth, // because months list is 0-indexed
    ).map((d) => WheelScrollOption(label: d.toString(), value: d)).toList();

List<int> getDaysInMonth(int year, int month) {
  if (month == 2) {
    return List.generate(isLeapYear(year) ? 29 : 28, (i) => i + 1);
  } else if ([4, 6, 9, 11].contains(month)) {
    return List.generate(30, (i) => i + 1);
  } else {
    return List.generate(31, (i) => i + 1);
  }
}

bool isLeapYear(int year) {
  return (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
}

int getIndex<T>(List<WheelScrollOption<T>> list, T value) {
  return list.indexWhere((option) => option.value == value);
}

final List<WheelScrollOption<int>> shortWeekdays = [
  const WheelScrollOption(label: 'Mon', value: 1),
  const WheelScrollOption(label: 'Tue', value: 2),
  const WheelScrollOption(label: 'Wed', value: 3),
  const WheelScrollOption(label: 'Thu', value: 4),
  const WheelScrollOption(label: 'Fri', value: 5),
  const WheelScrollOption(label: 'Sat', value: 6),
  const WheelScrollOption(label: 'Sun', value: 7),
];

final List<WheelScrollOption<int>> longWeekdays = [
  const WheelScrollOption(label: 'Monday', value: 1),
  const WheelScrollOption(label: 'Tuesday', value: 2),
  const WheelScrollOption(label: 'Wednesday', value: 3),
  const WheelScrollOption(label: 'Thursday', value: 4),
  const WheelScrollOption(label: 'Friday', value: 5),
  const WheelScrollOption(label: 'Saturday', value: 6),
  const WheelScrollOption(label: 'Sunday', value: 7),
];

// Short form
final List<WheelScrollOption<int>> shortMonths = [
  const WheelScrollOption(label: 'Jan', value: 1),
  const WheelScrollOption(label: 'Feb', value: 2),
  const WheelScrollOption(label: 'Mar', value: 3),
  const WheelScrollOption(label: 'Apr', value: 4),
  const WheelScrollOption(label: 'May', value: 5),
  const WheelScrollOption(label: 'Jun', value: 6),
  const WheelScrollOption(label: 'Jul', value: 7),
  const WheelScrollOption(label: 'Aug', value: 8),
  const WheelScrollOption(label: 'Sep', value: 9),
  const WheelScrollOption(label: 'Oct', value: 10),
  const WheelScrollOption(label: 'Nov', value: 11),
  const WheelScrollOption(label: 'Dec', value: 12),
];

// Long form
final List<WheelScrollOption<int>> longMonths = [
  const WheelScrollOption(label: 'January', value: 1),
  const WheelScrollOption(label: 'February', value: 2),
  const WheelScrollOption(label: 'March', value: 3),
  const WheelScrollOption(label: 'April', value: 4),
  const WheelScrollOption(label: 'May', value: 5),
  const WheelScrollOption(label: 'June', value: 6),
  const WheelScrollOption(label: 'July', value: 7),
  const WheelScrollOption(label: 'August', value: 8),
  const WheelScrollOption(label: 'September', value: 9),
  const WheelScrollOption(label: 'October', value: 10),
  const WheelScrollOption(label: 'November', value: 11),
  const WheelScrollOption(label: 'December', value: 12),
];

final List<SelectOption<String>> selectTimeZones = [
  const SelectOption(label: 'IST', value: 'Asia/Kolkata'),
  const SelectOption(label: 'PST', value: 'Asia/Manila'),
  const SelectOption(label: 'CEST', value: 'Europe/Luxembourg'),
  const SelectOption(label: 'CST', value: 'Asia/Macau'),
  const SelectOption(label: 'AEST', value: 'Australia/Brisbane'),
  const SelectOption(label: 'ACST', value: 'Australia/Broken_Hill'),
  const SelectOption(label: 'CEST', value: 'Europe/Vatican'),
];

final SelectOption<String> defaultTimeZones =
    const SelectOption(label: 'IST', value: 'Asia/Kolkata');
