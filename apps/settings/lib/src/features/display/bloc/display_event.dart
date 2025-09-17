part of 'display_bloc.dart';

class DisplayEvent extends Equatable {
  const DisplayEvent();

  @override
  List<Object> get props => [];
}

class GetDefaultSettingsEvent extends DisplayEvent {}

class SetBrightnessEvent extends DisplayEvent {
  final double brightness;

  const SetBrightnessEvent(this.brightness);
}

class SetBrightnessChangeEvent extends DisplayEvent {
  final double brightness;

  const SetBrightnessChangeEvent(this.brightness);
}

class SetAutoBrightnessEvent extends DisplayEvent {
  final bool isAutoBrightness;

  const SetAutoBrightnessEvent(this.isAutoBrightness);

  @override
  List<Object> get props => [isAutoBrightness];
}

class SetDisplayTimeoutEvent extends DisplayEvent {
  final double displayTimeout;

  const SetDisplayTimeoutEvent(this.displayTimeout);

  @override
  List<Object> get props => [displayTimeout];
}

class SetLockScreenTimeoutEvent extends DisplayEvent {
  final double lockScreenTimeout;

  const SetLockScreenTimeoutEvent(this.lockScreenTimeout);

  @override
  List<Object> get props => [lockScreenTimeout];
}
