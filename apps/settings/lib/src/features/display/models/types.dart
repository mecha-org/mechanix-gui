import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/select/select_type.dart';

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

enum DisplayScreenOffTime {
  tenSeconds,
  thirtySeconds,
  sixtySeconds,
  fiveMinutes,
  never,
}

final List<SelectOption<DisplayScreenOffTime>> screenOffOptions = [
  DisplayScreenOffTime.tenSeconds.toSelectOption('10s'),
  DisplayScreenOffTime.thirtySeconds.toSelectOption('30s'),
  DisplayScreenOffTime.sixtySeconds.toSelectOption('1 m'),
  DisplayScreenOffTime.fiveMinutes.toSelectOption('5 m'),
  DisplayScreenOffTime.never.toSelectOption('Never'),
];

double displayTime(DisplayScreenOffTime d) {
  switch (d) {
    case DisplayScreenOffTime.tenSeconds:
      return 10.0;
    case DisplayScreenOffTime.thirtySeconds:
      return 30.0;
    case DisplayScreenOffTime.sixtySeconds:
      return 60.0;
    case DisplayScreenOffTime.fiveMinutes:
      return 300.0;
    case DisplayScreenOffTime.never:
      return 1000.0;
  }
}

class DBusDefaultSettings {
  final int brightness;
  final bool autoBrightness;
  final int displayTimeout;
  final int lockScreenTimeout;

  DBusDefaultSettings(
      {required this.brightness,
      required this.autoBrightness,
      required this.displayTimeout,
      required this.lockScreenTimeout});
}

String displayLabel(double d) {
  switch (d) {
    case 10.0:
      return '10 s';
    case 30.0:
      return '30 s';
    case 60.0:
      return '60 s';
    case 300.0:
      return '5 m';
    default:
      return 'Never';
  }
}

DisplayScreenOffTime getValue(double d) {
  switch (d) {
    case 10.0:
      return DisplayScreenOffTime.tenSeconds;
    case 30.0:
      return DisplayScreenOffTime.thirtySeconds;
    case 60.0:
      return DisplayScreenOffTime.sixtySeconds;
    case 300.0:
      return DisplayScreenOffTime.fiveMinutes;
    default:
      return DisplayScreenOffTime.never;
  }
}
