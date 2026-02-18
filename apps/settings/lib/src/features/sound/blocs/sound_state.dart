import 'package:equatable/equatable.dart';
import 'package:pulseaudio/pulseaudio.dart';

class SoundState extends Equatable {
  // sink - output
  // source - input

  final bool loading;

  final List<PulseAudioSource> inputDevices;

  final PulseAudioSource? defaultInputDevice;

  final List<PulseAudioSink> outputDevices;

  final PulseAudioSink? defaultOutputDevice;

  final String? error;

  final bool enableLauncherSounds;

  final bool enableVibration;

  final String vibrationLevel;

  final String notificationSound;

  final double inputSoundLevel;

  final double outputSoundLevel;

  final bool inputDeviceLoading;

  final bool outputDeviceLoading;

  const SoundState({
    this.loading = false,
    this.inputDevices = const [],
    this.defaultInputDevice,
    this.outputDevices = const [],
    this.defaultOutputDevice,
    this.error,
    this.enableLauncherSounds = true,
    this.enableVibration = true,
    this.vibrationLevel = 'medium',
    this.notificationSound = 'space',
    this.inputSoundLevel = 0.4,
    this.outputSoundLevel = 0.4,
    this.inputDeviceLoading = false,
    this.outputDeviceLoading = false,
  });

  SoundState copyWith({
    bool? loading,
    List<PulseAudioSource>? inputDevices,
    PulseAudioSource? defaultInputDevice,
    List<PulseAudioSink>? outputDevices,
    PulseAudioSink? defaultOutputDevice,
    String? error,
    bool? enableLauncherSounds,
    bool? enableVibration,
    String? vibrationLevel,
    String? notificationSound,
    double? inputSoundLevel,
    double? outputSoundLevel,
    bool? inputDeviceLoading,
    bool? outputDeviceLoading,
  }) {
    return SoundState(
      loading: loading ?? this.loading,
      inputDevices: inputDevices ?? this.inputDevices,
      defaultInputDevice: defaultInputDevice ?? this.defaultInputDevice,
      outputDevices: outputDevices ?? this.outputDevices,
      defaultOutputDevice: defaultOutputDevice ?? this.defaultOutputDevice,
      error: error ?? this.error,
      enableLauncherSounds: enableLauncherSounds ?? this.enableLauncherSounds,
      enableVibration: enableVibration ?? this.enableVibration,
      vibrationLevel: vibrationLevel ?? this.vibrationLevel,
      notificationSound: notificationSound ?? this.notificationSound,
      inputSoundLevel: inputSoundLevel ?? this.inputSoundLevel,
      outputSoundLevel: outputSoundLevel ?? this.outputSoundLevel,
      inputDeviceLoading: inputDeviceLoading ?? this.inputDeviceLoading,
      outputDeviceLoading: outputDeviceLoading ?? this.outputDeviceLoading,
    );
  }

  @override
  List<Object?> get props => [
        loading,
        inputDevices,
        defaultInputDevice,
        outputDevices,
        defaultOutputDevice,
        error,
        enableLauncherSounds,
        enableVibration,
        vibrationLevel,
        notificationSound,
        inputSoundLevel,
        outputSoundLevel,
        inputDeviceLoading,
        outputDeviceLoading,
      ];
}
