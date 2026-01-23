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

  BatteryBloc({required this.batteryRepository}) : super(const BatteryState()) {
    on<BatteryInit>(_onInit);
    on<SetBatteryMode>(_setBatteryMode);
    on<BatteryInfoRequested>(_getBatteryInfo);
  }

  Future<void> _onInit(BatteryInit event, Emitter<BatteryState> emit) async {
    try {
      print('BLOC:: Init Battery Repo');
      await batteryRepository.init();
      add(BatteryInfoRequested());
    } catch (e) {
      print('Error  Init Battery Repo: $e');
    }
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
      // print('BLOC -Battery mode: ${batteryInfo.mode}');
      // print('BLOC -Battery percentage: ${batteryInfo.batteryPercentage}');
      // print('BLOC -Battery time To Full: ${batteryInfo.batteryChargingTime}');
      // print('BLOC -Battery time To Empty: ${batteryInfo.batteryRemainingTime}');
      // print(
      //     'BLOC -available battery modes: ${batteryInfo.availableBatteryModes}');
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
      if (stream != null) {
        changeStream = stream.listen((prop) async {
          print("Battery Property Update: $prop");

          if (prop.contains("TimeToFull") ||
              prop.contains("Percentage") ||
              prop.contains("TimeToEmpty")) add(BatteryInfoRequested());
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
      }
    } catch (e, stackTrace) {
      logger.e('Error initializing battery stream $e, $stackTrace ');
    }
  }

  @override
  Future<void> close() {
    print("battery bloc closing...");
    batteryRepository.close();
    changeStream?.cancel();
    return super.close();
  }
}
