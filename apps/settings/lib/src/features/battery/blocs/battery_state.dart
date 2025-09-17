import 'package:equatable/equatable.dart';
import 'package:upower/upower.dart';

class BatteryState extends Equatable {
  final double batteryPercentage;
  final UPowerDeviceState batteryStatus;
  final int? batteryChargingTime;
  final int? batteryRemainingTime;
  final String? error;
  final String? performanceMode;
  final List<String> availableBatteryModes;

  const BatteryState({
    this.batteryPercentage = 0.0,
    this.batteryStatus = UPowerDeviceState.unknown,
    this.error,
    this.batteryChargingTime = 0,
    this.batteryRemainingTime = 0,
    this.performanceMode,
    this.availableBatteryModes = const [],
  });

  BatteryState copyWith({
    double? batteryPercentage,
    UPowerDeviceState? batteryStatus,
    String? error,
    int? batteryRemainingTime,
    int? batteryChargingTime,
    String? performanceMode,
    List<String>? availableBatteryModes,
  }) {
    return BatteryState(
      batteryPercentage: batteryPercentage ?? this.batteryPercentage,
      batteryStatus: batteryStatus ?? this.batteryStatus,
      error: error ?? this.error,
      batteryChargingTime: batteryChargingTime ?? this.batteryChargingTime,
      batteryRemainingTime: batteryRemainingTime ?? this.batteryRemainingTime,
      availableBatteryModes:
          availableBatteryModes ?? this.availableBatteryModes,
      performanceMode: performanceMode ?? this.performanceMode,
    );
  }

  @override
  List<Object?> get props => [
        batteryPercentage,
        batteryStatus,
        batteryChargingTime,
        batteryRemainingTime,
        error,
        performanceMode,
        availableBatteryModes,
      ];
}
