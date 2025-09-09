import 'dart:async';

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/web.dart';
import 'package:mechanix_settings/src/features/battery/data/battery_repository.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_event.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_state.dart';
import 'package:upower/upower.dart';

class BatteryBloc extends Bloc<BatteryEvent, BatteryState> {
  final logger = Logger();
  final BatteryRepository batteryRepository;
  StreamSubscription? changeStream;

  BatteryBloc({required this.batteryRepository}) : super(BatteryState()) {
    on<SetBatteryMode>((event, emit) async {
      await batteryRepository.setBatteryMode(event.mode);
      add(BatteryInfoRequested());
    });
    on<BatteryInfoRequested>((event, emit) async {
      emit(state.copyWith(
        batteryPercentage: 0.0,
        status: UPowerDeviceState.unknown,
      ));
      try {
        final batteryInfo = await batteryRepository.getBatteryInfo();
        emit(state.copyWith(
            batteryPercentage: batteryInfo.batteryPercentage,
            status: batteryInfo.status,
            mode: batteryInfo.mode,
            batteryChargingTime: batteryInfo.batteryChargingTime,
            batteryRemainingTime: batteryInfo.batteryRemainingTime));
        _initializeBatteryStream();
      } catch (e) {
        emit(state.copyWith(
          status: UPowerDeviceState.unknown,
          error: e.toString(),
        ));
      }
    });
  }

  Future<void> _initializeBatteryStream() async {
    try {
      final stream = await batteryRepository.streamBatteryEvents();
      changeStream = stream.listen((prop) async {
        logger.i("Battery Property Update: $prop");
        // const relevantProps = [
        //   "Percentage",
        //   "State",
        //   "TimeToEmpty",
        //   "TimeToFull",
        //   "PowerSupply",
        // ];
        // // if (relevantProps.contains(prop))
        // if (prop.contains("UpdateTime")) add(BatteryInfoRequested());
      });
    } catch (e, stackTrace) {
      logger.e('Error initializing battery stream $e, $stackTrace ');
    }
  }

  @override
  Future<void> close() {
    changeStream?.cancel();
    return super.close();
  }
}
