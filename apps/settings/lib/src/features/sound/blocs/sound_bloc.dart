import 'dart:async';

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/web.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_event.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_state.dart';
import 'package:mechanix_settings/src/features/sound/data/sound_repository.dart';

class SoundBloc extends Bloc<SoundEvent, SoundState> {
  final SoundRepository soundRepository;
  final logger = Logger();

  StreamSubscription? soundEventsSubscription;

  SoundBloc({required this.soundRepository})
      : super(SoundState(
          inputDevices: [],
          outputDevices: [],
          loading: false,
          error: null,
          defaultInputDevice: null,
          defaultOutputDevice: null,
        )) {
    on<InitializeSound>(_onInitializeSound);
    on<GetInputDeviceList>(_onGetInputDeviceList);
    on<GetOutputDeviceList>(_onGetOutputDeviceList);

    on<SetInputDevice>(_setInputDevice);
    on<SetOutputDevice>(_setOutputDevice);

    on<SetInputDeviceVolume>(_setInputDeviceVolume);
    on<SetOutputDeviceVolume>(_setOutputDeviceVolume);

    on<SetInputDeviceMute>(_setInputDeviceMute);
    on<SetOutputDeviceMute>(_setOutputDeviceMute);

    _initializeServerInfoStream();
    _initializeSourceStream();
    _initializeSinkStream();
  }

  @override
  Future<void> close() {
    soundEventsSubscription?.cancel();
    return super.close();
  }

  Future<void> _onInitializeSound(
      InitializeSound event, Emitter<SoundState> emit) async {
    try {
      String defaultSource = await soundRepository.getDefaultSourceName();
      String defaultSink = await soundRepository.getDefaultSinkName();

      var sources = await soundRepository.getSourceList();
      var sinks = await soundRepository.getSinkList();

      logger.i("All Output Devices: $sinks");
      logger.i("All Input Devices: $sources");

      final defaultSourceObject = sources.firstWhere(
        (source) => source.name == defaultSource,
        orElse: () => sources.first,
      );

      final defaultSinkObject = sinks.firstWhere(
        (sink) => sink.name == defaultSink,
        orElse: () => sinks.first,
      );

      logger.i(
          "Default Input Device: ${defaultSourceObject.volume} ---> ${defaultSourceObject.description}");
      logger.i(
          "Default Output Device: ${defaultSinkObject.volume} ---> ${defaultSinkObject.description}");

      emit(state.copyWith(
        defaultInputDevice: defaultSourceObject,
        defaultOutputDevice: defaultSinkObject,
      ));
    } catch (e) {
      logger.i("Error initializing sound: $e");
      emit(state.copyWith(error: e.toString()));
    }
  }

// This method initializes the stream for sound server events like default source/sink name change
  Future<void> _initializeServerInfoStream() async {
    try {
      final serverStream = await soundRepository.streamSoundServerEvents();
      soundEventsSubscription = serverStream.listen((event) {
        logger.i("Sound event received: $event");
        if (event.defaultSourceName != state.defaultInputDevice?.name ||
            event.defaultSinkName != state.defaultOutputDevice?.name &&
                !isClosed) {
          add(InitializeSound());
        }
      });
    } catch (e) {
      logger.i("Error initializing sound stream: $e");
    }
  }

// This method initializes the stream for sound source events change like mute/volume change
  Future<void> _initializeSourceStream() async {
    try {
      final serverStream = await soundRepository.streamSoundSourceEvents();
      soundEventsSubscription = serverStream.listen((event) {
        logger.i("Sound SOURCE event received: $event");
        if (!isClosed) {
          // ignore: invalid_use_of_visible_for_testing_member
          emit(state.copyWith(defaultInputDevice: event));
        }
      });
    } catch (e) {
      logger.i("Error initializing sound stream: $e");
    }
  }

  Future<void> _initializeSinkStream() async {
    try {
      final serverStream = await soundRepository.streamSoundSinkEvents();
      soundEventsSubscription = serverStream.listen((event) {
        logger.i("Sound SINK event received: $event");
        if (!isClosed) {
          // ignore: invalid_use_of_visible_for_testing_member
          emit(state.copyWith(defaultOutputDevice: event));
        }
      });
    } catch (e) {
      logger.i("Error initializing sound stream: $e");
    }
  }

  Future<void> _onGetInputDeviceList(
      GetInputDeviceList event, Emitter<SoundState> emit) async {
    try {
      final sources = await soundRepository.getSourceList();
      emit(state.copyWith(inputDevices: sources));
    } catch (e) {
      logger.i("Error fetching source list: $e");
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _onGetOutputDeviceList(
      GetOutputDeviceList event, Emitter<SoundState> emit) async {
    try {
      final sinks = await soundRepository.getSinkList();
      emit(state.copyWith(outputDevices: sinks));
    } catch (e) {
      logger.i("Error fetching sink list: $e");
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _setInputDevice(
      SetInputDevice event, Emitter<SoundState> emit) async {
    try {
      await soundRepository.setDefaultSource(event.device);
      // check stream change & update state
    } catch (e) {
      logger.i("Error setting input device: $e");
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _setOutputDevice(
      SetOutputDevice event, Emitter<SoundState> emit) async {
    try {
      await soundRepository.setDefaultSink(event.device);
    } catch (e) {
      logger.i("Error setting output device: $e");
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _setInputDeviceVolume(
      SetInputDeviceVolume event, Emitter<SoundState> emit) async {
    try {
      await soundRepository.setSourceVolume(event.device, event.volume);
    } catch (e) {
      logger.i("Error setting source volume: $e");
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _setOutputDeviceVolume(
      SetOutputDeviceVolume event, Emitter<SoundState> emit) async {
    try {
      await soundRepository.setSinkVolume(event.device, event.volume);
    } catch (e) {
      logger.i("Error setting sink volume: $e");
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _setInputDeviceMute(
      SetInputDeviceMute event, Emitter<SoundState> emit) async {
    try {
      await soundRepository.setSourceMute(event.device, event.mute);
    } catch (e) {
      logger.i("Error setting source mute: $e");
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _setOutputDeviceMute(
      SetOutputDeviceMute event, Emitter<SoundState> emit) async {
    try {
      await soundRepository.setSinkMute(event.device, event.mute);
    } catch (e) {
      logger.i("Error setting sink mute: $e");
      emit(state.copyWith(error: e.toString()));
    }
  }
}
