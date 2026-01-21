import 'dart:async';

import 'package:dbus/dbus.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:intl/intl.dart';
import 'package:logger/web.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_event.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_state.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/dbus_service.dart';
import 'package:mechanix_settings/src/features/date_time/models/types.dart';
import 'package:mechanix_settings/src/features/display/models/types.dart';
import 'package:timezone/data/latest.dart' as tz;
import 'package:timezone/timezone.dart' as tz;
import 'package:widgets/widgets/wheel_scroll/wheel_scroll_options_type.dart';

class DateTimeBloc extends Bloc<DateTimeEvent, DateTimeState> {
  final logger = Logger();
  Timer? _timer; // Timer to update the time
  DateTime? _previousTime;
  StreamSubscription? _propertiesChangeSubscription;

  final DBusService _dBusService = DBusService();

  DateTimeBloc()
      : super(DateTimeState(
          autoDateTime: false,
          autoTimeZone: false,
          loading: false,
          error: null,
          selectedDate: DateTime.now().day,
          selectedMonth: DateTime.now().month,
          selectedYear: DateTime.now().year,
          selectedWeekDay: DateTime.now().weekday,
          selectedHour: hour24to12(DateTime.now().hour),
          selectedMinute: DateTime.now().minute,
        )) {
    on<GetDateTimeData>(_getDateTimeData);
    on<ToggleAutoDateTime>(_toggleAutoDateTime);
    on<ToggleAutoTimeZone>(_toggleAutoTimeZone);
    on<SetTimeZone>(_setTimeZone);
    on<SetTime>(_setTime);
    on<SetSelectedDateEvent>(_setSelectedDate);
    on<SetSelectedMonthEvent>(_setSelectedMonth);
    on<SetSelectedYearEvent>(_setSelectedYear);
    on<SetSelectedWeekDayEvent>(_setSelectedWeekDayEvent);
    on<UpdateSystemDateTimeEvent>(_onUpdateSystemDateTime);
    on<SetSelectedHourEvent>(_setSelectedHour);
    on<SetSelectedMinuteEvent>(_setSelectedMinute);
    on<SetSelectedMeridiemEvent>(_setSelectedMeridiem);
    on<SetSelectedTimezoneEvent>(_setSelectedTimezone);

    _initializeDBus();
    _streamPropertiesChange();
    _startUpdatingTime();
  }

  static const service = 'org.freedesktop.timedate1';
  static const path = '/org/freedesktop/timedate1';
  static const interface = 'org.freedesktop.timedate1';
  static const interactiveBoolean = false;

  Future<void> _initializeDBus() async {
    try {
      await _dBusService.initialize();
    } catch (e) {
      logger.e("Error initializing DBus client: $e");
    }
  }

  Future<void> _getDateTimeData(
    GetDateTimeData event,
    Emitter<DateTimeState> emit,
  ) async {
    try {
      final object = _dBusService.object!;

      final ntp = await object.getProperty(DBusService.interface, 'NTP');

      final ntpEnabled = (ntp as DBusBoolean).value;

      final timeUSec = await object.getProperty(interface, 'TimeUSec');
      final timeMillis = timeUSec.asUint64() ~/ 1000;
      final dateTimeUTC = DateTime.fromMillisecondsSinceEpoch(timeMillis);

      // logger.i("timeMillis $timeMillis ------> dateTimeUTC $dateTimeUTC");
      // logger.i("format ${DateFormat('hh:mm:ss a').format(dateTimeUTC)}");

      final currentTimeZone = await object.getProperty(interface, 'Timezone');
      // final listTimezonesMethodResponse =
      //     await object.callMethod(interface, 'ListTimezones', []);
      // final listTimezonesDBusValue =
      //     listTimezonesMethodResponse.returnValues.first as DBusArray;
      // final listTimezones =
      //     listTimezonesDBusValue.children.map((e) => e.asString()).toList();

      print("NTP Enabled: $ntpEnabled");
      print("currentTimeZone - ${currentTimeZone}");

      final meridiem = DateFormat('a').format(dateTimeUTC); // 'a' gives AM/PM
      final timezoneString = meridiem.toString();
      // final timezoneOptions = getAllTimezoneAbbreviations(listTimezones);

      // if (ntpEnabled) {
      //   final WheelScrollOption<String> ntpTimeZone = timezoneOptions
      //       .firstWhere((tz) => tz.label == dateTimeUTC.timeZoneName);
      //   add(SetTimeZone(ntpTimeZone.value));
      // }

      emit(state.copyWith(
        autoDateTime: ntpEnabled,
        systemDateTime: dateTimeUTC,
        // listTimezones: timezoneOptions,
        selectedMeridiem: timezoneString,
        selectedMinute: dateTimeUTC.minute,
        selectedHour: dateTimeUTC.hour - 12,
        selectedDate: dateTimeUTC.day,
        selectedMonth: dateTimeUTC.month,
        selectedYear: dateTimeUTC.year,
        selectedWeekDay: dateTimeUTC.weekday,
        // selectedTimezone: currentTimeZone,
      ));
    } catch (e) {
      logger.e("Error initializing date time data: $e");
      emit(state.copyWith(error: "Failed to fetch date/time data"));
    }
  }

  Future<void> _toggleAutoDateTime(
    ToggleAutoDateTime event,
    Emitter<DateTimeState> emit,
  ) async {
    try {
      final object = _dBusService.object!;

      await object.callMethod(
        interface,
        'SetNTP',
        [DBusBoolean(event.enabled), DBusBoolean(interactiveBoolean)],
      );

      emit(state.copyWith(
        autoDateTime: event.enabled,
      ));

      final ntp = await object.getProperty(DBusService.interface, 'NTP');

      final ntpEnabled = (ntp as DBusBoolean).value;

      if (ntpEnabled) {
        final ntpSynchronized =
            await object.getProperty(DBusService.interface, 'NTPSynchronized');

        final ntpSynchronizedEnabled = (ntpSynchronized as DBusBoolean).value;

        if (ntpSynchronizedEnabled) {
          add(GetDateTimeData());
        } else {
          _waitForNTPSynchronization(object, emit);
        }
      }
      logger.i("Auto Date Time set to: ${event.enabled}");
    } catch (e) {
      logger.e("Error toggling auto date time: $e");
      emit(state.copyWith(error: "Failed to toggle auto date time"));
    }
  }

  Future<void> _waitForNTPSynchronization(
      DBusRemoteObject object, Emitter<DateTimeState> emit) async {
    const maxRetries = 5;
    const retryDelay = Duration(seconds: 1);

    for (var i = 0; i < maxRetries; i++) {
      await Future.delayed(retryDelay);

      try {
        final ntpSynchronized =
            await object.getProperty(interface, 'NTPSynchronized');
        final ntpSynchronizedEnabled = (ntpSynchronized as DBusBoolean).value;

        if (ntpSynchronizedEnabled) {
          add(GetDateTimeData());
          return;
        }
      } catch (e) {
        logger.e('NTP synchronization error  $e');
      }
    }
  }

  Future<void> _toggleAutoTimeZone(
    ToggleAutoTimeZone event,
    Emitter<DateTimeState> emit,
  ) async {
    logger.w("Auto Time Zone not implemented yet");
  }

  Future<void> _setTimeZone(
    SetTimeZone event,
    Emitter<DateTimeState> emit,
  ) async {
    try {
      final object = _dBusService.object!;

      await object.callMethod(
        interface,
        'SetTimezone',
        [DBusString(event.timezone), DBusBoolean(interactiveBoolean)],
      );

      emit(state.copyWith(selectedTimezone: event.timezone));

      logger.i("Time Zone set to: ${event.timezone}");
    } catch (e) {
      logger.e("Error setting time zone: $e");
      emit(state.copyWith(error: "Failed to set time zone"));
    }
  }

  Future<void> _setTime(
    SetTime event,
    Emitter<DateTimeState> emit,
  ) async {
    try {
      final object = _dBusService.object!;

      await object.callMethod(
        interface,
        'SetTime',
        [
          DBusInt64(event.timeMicrosecondsSinceEpoch),
          DBusBoolean(false), // relative
          DBusBoolean(interactiveBoolean),
        ],
      );
      logger.i("Time set to: ${event.timeMicrosecondsSinceEpoch}");
    } catch (e) {
      logger.e("Error setting time: $e");
      emit(state.copyWith(error: "Failed to set time"));
    }
  }

  Future<void> _streamPropertiesChange() async {
    try {
      final object = _dBusService.object!;

      _propertiesChangeSubscription =
          object.propertiesChanged.listen((DBusPropertiesChangedSignal signal) {
        logger.w('Properties changed: ${signal.changedProperties}');
        if (!isClosed) {
          if (signal.changedProperties.containsKey('NTP')) {
            add(GetDateTimeData());
          } else if (signal.changedProperties.containsKey('Timezone')) {
            add(GetDateTimeData());
          }
        }
      });
    } catch (e) {
      logger.e("Error listening to properties change: $e");
    }
  }

  // TODO: discuss
  // Poll the system time using DateTime.now() every second
  Future<void> _startUpdatingTime() async {
    // Only start a timer if it is not already running
    if (_timer == null || !_timer!.isActive) {
      _timer = Timer.periodic(Duration(seconds: 1), (timer) {
        final currentTime = DateTime.now();
        if (_previousTime == null ||
            currentTime.minute != _previousTime!.minute) {
          _previousTime = currentTime;
          // Update the state with the new system time
          emit(state.copyWith(systemDateTime: currentTime));
        }
      });
    }
  }

  @override
  Future<void> close() {
    _propertiesChangeSubscription?.cancel();
    _timer?.cancel();
    _dBusService.dispose();
    return super.close();
  }

  List<WheelScrollOption<String>> getAllTimezoneAbbreviations(
      List<String> listTimezones) {
    tz.initializeTimeZones();

    final locations = tz.timeZoneDatabase.locations;

    final List<WheelScrollOption<String>> timezoneOptions = [];
    locations.forEach((key, location) {
      final now = tz.TZDateTime.now(location);
      final abbreviation = now.timeZoneName;
      timezoneOptions
          .add(WheelScrollOption(value: location.name, label: abbreviation));
    });

    return timezoneOptions;
  }

  void _onUpdateSystemDateTime(
    UpdateSystemDateTimeEvent event,
    Emitter<DateTimeState> emit,
  ) {
    if (state.systemDateTime != null && state.systemDateTime is DateTime) {
      add(SetTime(
          dateTimeToMillisSinceEpoch(event.dateTime, state.systemDateTime!)));
      add(SetTimeZone(event.timeZone));
    }

    emit(state.copyWith(
      systemDateTime: event.dateTime,
    ));
  }

  Future<void> _setSelectedDate(
      SetSelectedDateEvent event, Emitter<DateTimeState> emit) async {
    emit(state.copyWith(selectedDate: event.date));
    updateWeekday(
        year: state.selectedYear,
        month: state.selectedMonth,
        date: event.date,
        emit: emit);
  }

  Future<void> _setSelectedMonth(
      SetSelectedMonthEvent event, Emitter<DateTimeState> emit) async {
    final daysInNewMonth = getDaysInMonth(state.selectedYear, event.month);

    int newDate = state.selectedDate;
    if (state.selectedDate > daysInNewMonth.length) {
      newDate = daysInNewMonth.length;
    }

    emit(state.copyWith(selectedMonth: event.month, selectedDate: newDate));
    updateWeekday(
      year: state.selectedYear,
      month: event.month,
      date: newDate,
      emit: emit,
    );
  }

  Future<void> _setSelectedYear(
      SetSelectedYearEvent event, Emitter<DateTimeState> emit) async {
    final daysInNewMonth = getDaysInMonth(event.year, state.selectedMonth);

    int newDate = state.selectedDate;

    if (state.selectedDate >= daysInNewMonth.length) {
      newDate = daysInNewMonth.length;
    }
    emit(state.copyWith(
      selectedYear: event.year,
      selectedDate: newDate,
    ));

    updateWeekday(
      year: event.year,
      month: state.selectedMonth,
      date: newDate,
      emit: emit,
    );
  }

  Future<void> _setSelectedWeekDayEvent(
      SetSelectedWeekDayEvent event, Emitter<DateTimeState> emit) async {
    emit(state.copyWith(selectedWeekDay: event.day));
  }

  Future<void> _setSelectedHour(
      SetSelectedHourEvent event, Emitter<DateTimeState> emit) async {
    emit(state.copyWith(selectedHour: event.selectedHour));
  }

  Future<void> _setSelectedMinute(
      SetSelectedMinuteEvent event, Emitter<DateTimeState> emit) async {
    emit(state.copyWith(selectedMinute: event.selectedMinute));
  }

  Future<void> _setSelectedMeridiem(
      SetSelectedMeridiemEvent event, Emitter<DateTimeState> emit) async {
    emit(state.copyWith(selectedMeridiem: event.selectedMeridiem));
  }

  Future<void> _setSelectedTimezone(
      SetSelectedTimezoneEvent event, Emitter<DateTimeState> emit) async {
    add(SetTimeZone(event.selectedTimezone));
    emit(state.copyWith(selectedTimezone: event.selectedTimezone));
  }

  int getDaysInSelectedMonth({required int year, required int month}) {
    return getDaysInMonth(year, month + 1).length;
  }

  void adjustDayIfNeeded(
      {required int year, required int month, required int date}) {
    final daysInMonth = getDaysInSelectedMonth(year: year, month: month);
    if (date >= daysInMonth) {
      add(SetSelectedDateEvent(daysInMonth - 1));
      date = daysInMonth - 1;
    }
  }

  void updateWeekday({
    required int year,
    required int month,
    required int date,
    required Emitter<DateTimeState> emit,
  }) {
    final selectedDate = DateTime(year, month, date);
    final int selectedWeekDay = selectedDate.weekday;

    emit(state.copyWith(selectedWeekDay: selectedWeekDay));
  }
}
