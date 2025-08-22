import 'package:upower/upower.dart';
import 'package:equatable/equatable.dart';

class BatteryState extends Equatable {
  final double batteryPercentage;
  final UPowerDeviceState status;
  final String? mode;
  final int? batteryChargingTime;
  final int? batteryRemainingTime;
  final String? error;

  const BatteryState({
    this.batteryPercentage = 0.0,
    this.status = UPowerDeviceState.unknown,
    this.mode = '',
    this.error,
    this.batteryChargingTime = 0,
    this.batteryRemainingTime = 0,
  });

  BatteryState copyWith({
    double? batteryPercentage,
    UPowerDeviceState? status,
    String? mode,
    String? error,
    int? batteryRemainingTime,
    int? batteryChargingTime,
  }) {
    return BatteryState(
        batteryPercentage: batteryPercentage ?? this.batteryPercentage,
        status: status ?? this.status,
        mode: mode ?? this.mode,
        error: error ?? this.error,
        batteryChargingTime: batteryChargingTime,
        batteryRemainingTime: batteryRemainingTime);
  }

  @override
  List<Object?> get props => [batteryPercentage, status, mode, error];
}
