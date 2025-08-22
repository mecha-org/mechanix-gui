abstract class BatteryEvent {}

class BatteryInfoRequested extends BatteryEvent {
  BatteryInfoRequested();
}

class BatteryStreamListen extends BatteryEvent {}

class SetBatteryMode extends BatteryEvent {
  final String mode;
  SetBatteryMode(this.mode);
}
