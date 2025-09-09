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

  const SoundState({
    this.loading = false,
    this.inputDevices = const [],
    this.defaultInputDevice,
    this.outputDevices = const [],
    this.defaultOutputDevice,
    this.error,

  });

  SoundState copyWith({
    bool? loading,
    List<PulseAudioSource>? inputDevices,
    PulseAudioSource? defaultInputDevice,
    List<PulseAudioSink>? outputDevices,
    PulseAudioSink? defaultOutputDevice,
    String? error,
  }) {
    return SoundState(
      loading: loading ?? this.loading,
      inputDevices: inputDevices ?? this.inputDevices,
      defaultInputDevice: defaultInputDevice ?? this.defaultInputDevice,
      outputDevices: outputDevices ?? this.outputDevices,
      defaultOutputDevice: defaultOutputDevice ?? this.defaultOutputDevice,
      error: error ?? this.error,
    );
  }

  @override
  List<Object?> get props => [loading, inputDevices, defaultInputDevice, outputDevices, defaultOutputDevice, error];

}