import 'dart:async';

import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:logger/web.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_event.dart';
import 'package:mechanix_settings/src/features/sound/blocs/sound_state.dart';
import 'package:mechanix_settings/src/features/sound/data/sound_repository.dart';

class SoundBloc extends Bloc<SoundEvent, SoundState> {
  final SoundRepository soundRepository;
  final logger = Logger();

  // Separate subscriptions for each stream
  StreamSubscription? _serverInfoSubscription;
  StreamSubscription? _sourceSubscription;
  StreamSubscription? _sinkSubscription;
  StreamSubscription? _sourceRemovedSubscription;
  StreamSubscription? _sinkRemovedSubscription;

  SoundBloc({required this.soundRepository})
      : super(const SoundState(
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

    on<RefreshOutputDevicesList>(_refreshOutputDevicesList);
    on<RefreshInputDevicesList>(_refreshInputDevicesList);

    on<UpdateAvailableDevices>(_updateAvailableDevices);

    on<SetInputDevice>(_setInputDevice);
    on<SetOutputDevice>(_setOutputDevice);

    on<SetInputDeviceVolume>(_setInputDeviceVolume);
    on<SetOutputDeviceVolume>(_setOutputDeviceVolume);

    on<SetInputDeviceMute>(_setInputDeviceMute);
    on<SetOutputDeviceMute>(_setOutputDeviceMute);

    on<SetEnableLauncherSoundsEvent>(_setEnableLauncherSounds);

    on<SetEnableVibrationEvent>(_setEnableVibration);

    on<SetVibrationLevelEvent>(_setVibrationLevel);

    on<SetNotificationSoundEvent>(_setNotificationSound);

    // Initialize all streams
    _initializeServerInfoStream();
    _initializeSourceStream();
    _initializeSinkStream();
    _initializeSinkRemoveStream();
    _initializeSourceRemoveStream();
  }

  @override
  Future<void> close() {
    print("sound bloc closing...");
    // Cancel all subscriptions
    _serverInfoSubscription?.cancel();
    _sourceSubscription?.cancel();
    _sinkSubscription?.cancel();
    _sourceRemovedSubscription?.cancel();
    _sinkRemovedSubscription?.cancel();

    soundRepository.close();
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
      final soundSettings = await soundRepository.getSoundSettings();

      logger.i(
          "Default Input Device: ${defaultSourceObject.volume} ---> ${defaultSourceObject.description}");
      logger.i(
          "Default Output Device: ${defaultSinkObject.volume} ---> ${defaultSinkObject.description}");

      add(GetInputDeviceList());
      add(GetOutputDeviceList());

      emit(state.copyWith(
        defaultInputDevice: defaultSourceObject,
        defaultOutputDevice: defaultSinkObject,
        enableLauncherSounds: soundSettings.enableSounds,
        enableVibration: soundSettings.enableVibration,
        vibrationLevel: soundSettings.vibrationLevel,
        notificationSound: soundSettings.notificationSound,
        inputSoundLevel: defaultSourceObject.volume,
        outputSoundLevel: defaultSinkObject.volume,
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
      _serverInfoSubscription = serverStream.listen((event) {
        logger.i("Sound event received: $event");
        if (event.defaultSourceName != state.defaultInputDevice?.name ||
            event.defaultSinkName != state.defaultOutputDevice?.name &&
                !isClosed) {
          add(InitializeSound());
        }
      });
    } catch (e) {
      print("Error initializing sound server stream: $e");
    }
  }

  // This method initializes the stream for sound source events change like mute/volume change
  Future<void> _initializeSourceStream() async {
    try {
      final sourceStream = await soundRepository.streamSoundSourceEvents();
      _sourceSubscription = sourceStream.listen((event) {
        logger.i("Sound SOURCE event received: $event");
        if (!isClosed) {
          // ignore: invalid_use_of_visible_for_testing_member
          emit(state.copyWith(defaultInputDevice: event));
        }
      });
    } catch (e) {
      print("Error initializing sound source stream: $e");
    }
  }

  Future<void> _initializeSinkStream() async {
    try {
      final sinkStream = await soundRepository.streamSoundSinkEvents();
      _sinkSubscription = sinkStream.listen((event) {
        print("Sound SINK event received: $event");
        if (!isClosed) {
          // ignore: invalid_use_of_visible_for_testing_member
          emit(state.copyWith(defaultOutputDevice: event));
        }
      });
    } catch (e) {
      print("Error initializing sound sink stream: $e");
    }
  }

  Future<void> _initializeSinkRemoveStream() async {
    try {
      final sinkRemovedStream =
          await soundRepository.streamSoundSinkRemovedEvents();
      _sinkRemovedSubscription = sinkRemovedStream.listen((event) {
        print("Sound SINK removed event received: $event");
        if (!isClosed) {
          add(UpdateAvailableDevices(index: event));
        }
      });
    } catch (e) {
      print("Error initializing sound sink removed stream: $e");
    }
  }

  Future<void> _initializeSourceRemoveStream() async {
    try {
      final sourceRemovedStream =
          await soundRepository.streamSoundSourceRemovedEvents();
      _sourceRemovedSubscription = sourceRemovedStream.listen((event) {
        print("Sound SOURCE removed event received: $event");
        if (!isClosed) {
          add(UpdateAvailableDevices(index: event, isSinkRemove: false));
        }
      });
    } catch (e) {
      print("Error initializing sound stream: $e");
    }
  }

  Future<void> _updateAvailableDevices(
      UpdateAvailableDevices event, Emitter<SoundState> emit) async {
    if (event.isSinkRemove) {
      final outputDevices = state.outputDevices;

      final updatedDevices =
          outputDevices.where((device) => device.index != event.index).toList();

      emit(state.copyWith(outputDevices: updatedDevices));
    } else {
      final inputDevices = state.inputDevices;

      final updatedDevices =
          inputDevices.where((device) => device.index != event.index).toList();

      emit(state.copyWith(inputDevices: updatedDevices));
    }
  }

  Future<void> _onGetInputDeviceList(
      GetInputDeviceList event, Emitter<SoundState> emit) async {
    try {
      final sources = await soundRepository.getSourceList();

      // For Refresh Device List only add new devices
      if (state.inputDevices.isNotEmpty) {
        final oldDevices =
            state.inputDevices.map((device) => device.name).toSet();

        final newDeviceList = sources
            .where((device) => !oldDevices.contains(device.name))
            .toList();

        if (newDeviceList.isNotEmpty) {
          emit(state.copyWith(
              inputDevices: [...state.inputDevices, ...newDeviceList]));
        }
      } else {
        emit(state.copyWith(inputDevices: sources));
      }
    } catch (e) {
      print("Error fetching source list: $e");
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _refreshInputDevicesList(
      RefreshInputDevicesList event, Emitter<SoundState> emit) async {
    try {
      print('Refreshing input device list');

      emit(state.copyWith(inputDevices: []));

      add(GetInputDeviceList());
    } catch (error) {
      print('Error refreshing Output Devices $error');
    }
  }

  Future<void> _onGetOutputDeviceList(
      GetOutputDeviceList event, Emitter<SoundState> emit) async {
    try {
      final sinks = await soundRepository.getSinkList();

      // For Refresh Device List only add new devices
      if (state.outputDevices.isNotEmpty) {
        final oldDevices =
            state.outputDevices.map((device) => device.name).toSet();

        final newDeviceList =
            sinks.where((device) => !oldDevices.contains(device.name)).toList();

        if (newDeviceList.isNotEmpty) {
          emit(state.copyWith(
              outputDevices: [...state.outputDevices, ...newDeviceList]));
        }
      } else {
        emit(state.copyWith(outputDevices: sinks));
      }
    } catch (e) {
      logger.i("Error fetching sink list: $e");
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _refreshOutputDevicesList(
      RefreshOutputDevicesList event, Emitter<SoundState> emit) async {
    try {
      print('Refreshing output device list');

      emit(state.copyWith(outputDevices: []));

      add(GetOutputDeviceList());
    } catch (error) {
      print('Error refreshing Output Devices $error');
    }
  }

  Future<void> _setInputDevice(
      SetInputDevice event, Emitter<SoundState> emit) async {
    try {
      await soundRepository.setDefaultSource(event.device);
      // check stream change & update state
      final inputDevice = state.inputDevices
          .firstWhere((device) => device.name == event.device);

      emit(state.copyWith(defaultInputDevice: inputDevice));
    } catch (e) {
      logger.i("Error setting input device: $e");
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _setOutputDevice(
      SetOutputDevice event, Emitter<SoundState> emit) async {
    try {
      await soundRepository.setDefaultSink(event.device);
      final outputDevice = state.outputDevices
          .firstWhere((device) => device.name == event.device);

      emit(state.copyWith(defaultOutputDevice: outputDevice));
    } catch (e) {
      logger.i("Error setting output device: $e");
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _setInputDeviceVolume(
      SetInputDeviceVolume event, Emitter<SoundState> emit) async {
    try {
      emit(state.copyWith(inputSoundLevel: event.volume));
      await soundRepository.setSourceVolume(event.device, event.volume);
    } catch (e) {
      logger.i("Error setting source volume: $e");
      emit(state.copyWith(error: e.toString()));
    }
  }

  Future<void> _setOutputDeviceVolume(
      SetOutputDeviceVolume event, Emitter<SoundState> emit) async {
    try {
      emit(state.copyWith(outputSoundLevel: event.volume));
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

  Future<void> _setEnableLauncherSounds(
      SetEnableLauncherSoundsEvent event, Emitter<SoundState> emit) async {
    try {
      logger.i('set enable sounds ${event.enableLauncherSounds}');
      await soundRepository.setEnableSounds(event.enableLauncherSounds);
      emit(state.copyWith(enableLauncherSounds: event.enableLauncherSounds));
    } catch (error) {
      logger.e('set enable sounds error $error');
    }
  }

  Future<void> _setEnableVibration(
      SetEnableVibrationEvent event, Emitter<SoundState> emit) async {
    try {
      logger.i('Setting enable vibration ${event.enableVibration}');
      await soundRepository.setEnableVibration(event.enableVibration);
      emit(state.copyWith(enableVibration: event.enableVibration));
    } catch (error) {
      logger.e('Error setting enable vibration $error');
    }
  }

  Future<void> _setVibrationLevel(
      SetVibrationLevelEvent event, Emitter<SoundState> emit) async {
    try {
      logger.i('Setting vibration level ${event.vibrationLevel}');
      await soundRepository.setVibrationLevel(event.vibrationLevel);
      emit(state.copyWith(vibrationLevel: event.vibrationLevel));
    } catch (error) {
      logger.e('Error setting vibration level $error');
    }
  }

  Future<void> _setNotificationSound(
      SetNotificationSoundEvent event, Emitter<SoundState> emit) async {
    try {
      logger.i('Setting notification sound ${event.notificationSound}');
      await soundRepository.setNotificationSound(event.notificationSound);
      emit(state.copyWith(notificationSound: event.notificationSound));
    } catch (error) {
      logger.e('Error setting notification sound $error');
    }
  }
}
