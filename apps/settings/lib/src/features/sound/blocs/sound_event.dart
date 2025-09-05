
import 'package:equatable/equatable.dart';

abstract class SoundEvent extends Equatable {
  @override
  List<Object?> get props => [];
}

class InitializeSound extends SoundEvent {}

class GetInputDeviceList extends SoundEvent {}
class GetOutputDeviceList extends SoundEvent {}


class SetInputDevice extends SoundEvent {
  final String device;

  SetInputDevice(this.device);
}

class SetInputDeviceMute extends SoundEvent {
  final String device;
  final bool mute;

  SetInputDeviceMute(this.device, this.mute);
}

class SetInputDeviceVolume extends SoundEvent {
  final String device;
  final double volume;

  SetInputDeviceVolume(this.device, this.volume);
}

class SetOutputDevice extends SoundEvent {
    final String device;

    SetOutputDevice(this.device);
}

class SetOutputDeviceVolume extends SoundEvent {
  final String device;
  final double volume;

  SetOutputDeviceVolume(this.device, this.volume);
}

class SetOutputDeviceMute extends SoundEvent {
  final String device;
  final bool mute;

  SetOutputDeviceMute(this.device, this.mute);
}