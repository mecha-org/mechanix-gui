import 'package:upower/upower.dart';

class BatteryInfo {
  double batteryPercentage;
  UPowerDeviceState status;
  String mode;
  int batteryRemainingTime;
  int batteryChargingTime;
  List<String> availableBatteryModes;

  BatteryInfo({
    required this.batteryPercentage,
    required this.status,
    required this.mode,
    required this.batteryChargingTime,
    required this.batteryRemainingTime,
    required this.availableBatteryModes,
  });
}
