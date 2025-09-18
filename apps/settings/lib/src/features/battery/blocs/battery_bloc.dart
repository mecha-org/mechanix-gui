import 'dart:async';

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/web.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_event.dart';
import 'package:mechanix_settings/src/features/battery/blocs/battery_state.dart';
import 'package:mechanix_settings/src/features/battery/data/battery_repository.dart';
import 'package:upower/upower.dart';

class BatteryBloc extends Bloc<BatteryEvent, BatteryState> {
  final logger = Logger();
  final BatteryRepository batteryRepository;
  StreamSubscription? changeStream;

  BatteryBloc({required this.batteryRepository}) : super(BatteryState()) {
    on<SetBatteryMode>(_setBatteryMode);
    on<BatteryInfoRequested>(_getBatteryInfo);
  }

  Future<void> _setBatteryMode(
      SetBatteryMode event, Emitter<BatteryState> emit) async {
    await batteryRepository.setBatteryMode(event.mode);
    add(BatteryInfoRequested());
  }

  Future<void> _getBatteryInfo(
      BatteryInfoRequested event, Emitter<BatteryState> emit) async {
    try {
      final batteryInfo = await batteryRepository.getBatteryInfo();

      emit(state.copyWith(
        batteryPercentage: batteryInfo.batteryPercentage,
        batteryStatus: batteryInfo.status,
        performanceMode: batteryInfo.mode,
        batteryChargingTime: batteryInfo.batteryChargingTime,
        batteryRemainingTime: batteryInfo.batteryRemainingTime,
        availableBatteryModes: batteryInfo.availableBatteryModes,
      ));
      _initializeBatteryStream();
    } catch (e) {
      emit(state.copyWith(
        batteryStatus: UPowerDeviceState.unknown,
        error: e.toString(),
      ));
    }
  }

  Future<void> _initializeBatteryStream() async {
    try {
      final stream = await batteryRepository.streamBatteryEvents();
      changeStream = stream.listen((prop) async {
        logger.i("Battery Property Update: $prop");

        add(BatteryInfoRequested());
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
