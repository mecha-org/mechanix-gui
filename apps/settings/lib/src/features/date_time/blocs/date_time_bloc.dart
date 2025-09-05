import 'dart:async';

import 'package:dbus/dbus.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/web.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_event.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/date_time_state.dart';
import 'package:mechanix_settings/src/features/date_time/blocs/dbus_service.dart';

class DateTimeBloc extends Bloc<DateTimeEvent, DateTimeState> {
  final logger = Logger();
  Timer? _timer; // Timer to update the time
  DateTime? _previousTime;
  StreamSubscription? _propertiesChangeSubscription;

  final DbusService _dbusService = DbusService();

  DateTimeBloc()
      : super(DateTimeState(
          autoDateTime: false,
          autoTimeZone: false,
          loading: false,
          error: null,
        )) {
    on<GetDateTimeData>(_getDateTimeData);
    on<ToggleAutoDateTime>(_toggleAutoDateTime);
    on<ToggleAutoTimeZone>(_toggleAutoTimeZone);
    on<SetTimeZone>(_setTimeZone);
    on<SetTime>(_setTime);

    _initializeDBus();
    _streamPropertiesChange();
    // _startUpdatingTime();
  }

  static const service = 'org.freedesktop.timedate1';
  static const path = '/org/freedesktop/timedate1';
  static const interface = 'org.freedesktop.timedate1';
  static const interactiveBoolean = false;

  Future<void> _initializeDBus() async {
    try {
      await _dbusService.initialize();
    } catch (e) {
      logger.e("Error initializing DBus client: $e");
    }
  }

  Future<void> _getDateTimeData(
    GetDateTimeData event,
    Emitter<DateTimeState> emit,
  ) async {
    try {
      final object = _dbusService.object!;

      final ntp = await object.getProperty(DbusService.interface, 'NTP');

      final ntpEnabled = (ntp as DBusBoolean).value;

      final timeUSec = await object.getProperty(interface, 'TimeUSec');
      final timeMillis = timeUSec.asUint64() ~/ 1000;
      final dateTimeUTC = DateTime.fromMillisecondsSinceEpoch(timeMillis);

      // logger.i("timeMillis $timeMillis ------> dateTimeUTC $dateTimeUTC");
      // logger.i("format ${DateFormat('hh:mm:ss a').format(dateTimeUTC)}");

      final currentTimeZone = await object.getProperty(interface, 'Timezone');
      final listTimezonesMethodResponse =
          await object.callMethod(interface, 'ListTimezones', []);
      final listTimezonesDbusValue =
          listTimezonesMethodResponse.returnValues.first as DBusArray;
      final listTimezones =
          listTimezonesDbusValue.children.map((e) => e.asString()).toList();

      logger.i("NTP Enabled: $ntpEnabled");
      logger.i("Current Time Zone: $currentTimeZone");

      emit(state.copyWith(
          autoDateTime: ntpEnabled,
          dateTime: dateTimeUTC,
          currentTimeZone: currentTimeZone.asString(),
          listTimezones: listTimezones));
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
      final object = _dbusService.object!;

      await object.callMethod(
        interface,
        'SetNTP',
        [DBusBoolean(event.enabled), DBusBoolean(interactiveBoolean)],
      );
      logger.i("Auto Date Time set to: ${event.enabled}");
    } catch (e) {
      logger.e("Error toggling auto date time: $e");
      emit(state.copyWith(error: "Failed to toggle auto date time"));
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
      final object = _dbusService.object!;

      await object.callMethod(
        interface,
        'SetTimezone',
        [DBusString(event.timezone), DBusBoolean(interactiveBoolean)],
      );
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
      final object = _dbusService.object!;

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
      final object = _dbusService.object!;

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
            currentTime.second != _previousTime!.second) {
          _previousTime = currentTime;
          // Update the state with the new system time
          emit(state.copyWith(dateTime: currentTime));
        }
      });
    }
  }

  @override
  Future<void> close() {
    _propertiesChangeSubscription?.cancel();
    _timer?.cancel();
    _dbusService.dispose();
    return super.close();
  }
}
