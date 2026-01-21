import 'dart:async';

import 'package:logger/web.dart';
import 'package:mechanix_settings/src/features/sound/data/dbus_sound_service.dart';
import 'package:mechanix_settings/src/features/sound/data/sound_repository.dart';
import 'package:pulseaudio/pulseaudio.dart';

class SoundRepositoryImpl implements SoundRepository {
  final client = PulseAudioClient();
  final logger = Logger();
  bool _connected = false;
  final DBusSoundService _dBusSoundService = DBusSoundService();

  SoundRepositoryImpl() {
    _init();
  }

  Future<void> _init() async {
    if (!_connected) {
      try {
        await client.initialize();

        _connected = true;
      } catch (e) {
        logger.e("Error connecting to PulseAudio: $e");
      }
    }
  }

  Future<void> _ensureConnected() async {
    if (!_connected) {
      await client.initialize();
      _connected = true;
    }
  }

  @override
  Future<Stream<PulseAudioServerInfo>> streamSoundServerEvents() async {
    await _ensureConnected();
    return client.onServerInfoChanged;
  }

  @override
  Future<Stream<PulseAudioSource>> streamSoundSourceEvents() async {
    await _ensureConnected();
    return client.onSourceChanged;
  }

  @override
  Future<Stream<PulseAudioSink>> streamSoundSinkEvents() async {
    await _ensureConnected();
    return client.onSinkChanged;
  }

  @override
  Future<Stream<int>> streamSoundSourceRemovedEvents() async {
    await _ensureConnected();
    return client.onSourceRemoved;
  }

  @override
  Future<Stream<int>> streamSoundSinkRemovedEvents() async {
    await _ensureConnected();
    return client.onSinkRemoved;
  }

  @override
  Future<String> getDefaultSinkName() async {
    await _ensureConnected();
    try {
      final serverInfo = await client.getServerInfo();
      final sink = serverInfo.defaultSinkName;
      return sink;
    } catch (e) {
      logger.e("Error getting default audio sink: $e");
      return '';
    }
  }

  @override
  Future<String> getDefaultSourceName() async {
    await _ensureConnected();
    try {
      final serverInfo = await client.getServerInfo();
      final source = serverInfo.defaultSourceName;
      return source;
    } catch (e) {
      logger.e("Error getting default audio source: $e");
      return '';
    }
  }

  @override
  Future<List<PulseAudioSource>> getSourceList() async {
    await _ensureConnected();
    try {
      final sources = await client.getSourceList();
      final filteredSources = _PulseAudioHelper.getSourceDevices(sources);
      return filteredSources;
    } catch (e) {
      logger.e("Error getting audio sources: $e");
      return [];
    }
  }

  @override
  Future<List<PulseAudioSink>> getSinkList() async {
    await _ensureConnected();
    try {
      final sinks = await client.getSinkList();
      final filteredSinks = _PulseAudioHelper.getSinkDevices(sinks);
      return filteredSinks;
    } catch (e) {
      logger.e("Error getting audio sinks: $e");
      return [];
    }
  }

  @override
  Future<void> setDefaultSource(String device) async {
    await _ensureConnected();
    try {
      await client.setDefaultSource(device);
    } catch (e) {
      logger.e("Error setting audio source device: $e");
    }
  }

  @override
  Future<void> setDefaultSink(String device) async {
    await _ensureConnected();
    try {
      await client.setDefaultSink(device);
    } catch (e) {
      logger.e("Error setting audio sink device: $e");
    }
  }

  @override
  Future<void> setSourceMute(String sourceName, bool mute) async {
    await _ensureConnected();
    try {
      await client.setSourceMute(sourceName, mute);
    } catch (e) {
      logger.e("Error setting audio source volume: $e");
    }
  }

  @override
  Future<void> setSinkMute(String sinkName, bool mute) async {
    await _ensureConnected();
    try {
      await client.setSinkMute(sinkName, mute);
    } catch (e) {
      logger.e("Error setting audio source volume: $e");
    }
  }

  @override
  Future<void> setSourceVolume(String sourceName, double volume) async {
    await _ensureConnected();
    try {
      await client.setSourceVolume(sourceName, volume);
    } catch (e) {
      logger.e("Error setting audio source volume: $e");
    }
  }

  @override
  Future<void> setSinkVolume(String sinkName, double volume) async {
    await _ensureConnected();
    try {
      await client.setSinkVolume(sinkName, volume);
    } catch (e) {
      logger.e("Error setting audio sink volume: $e");
    }
  }

  @override
  getSoundSettings() async {
    return await _dBusSoundService.getSoundSettings();
  }

  @override
  setEnableSounds(bool enableSounds) async {
    return await _dBusSoundService.setEnableSounds(enableSounds);
  }

  @override
  setEnableVibration(bool enableVibration) async {
    return await _dBusSoundService.setEnableVibration(enableVibration);
  }

  @override
  setVibrationLevel(String vibrationLevel) async {
    return await _dBusSoundService.setVibrationLevel(vibrationLevel);
  }

  @override
  setNotificationSound(String notificationSound) async {
    return await _dBusSoundService.setNotificationSound(notificationSound);
  }
}

class _PulseAudioHelper {
  static List<PulseAudioSource> getSourceDevices(
      List<PulseAudioSource> sources) {
    return sources
        .where((source) => !source.name.contains('.monitor'))
        .toList();
  }

  static List<PulseAudioSink> getSinkDevices(List<PulseAudioSink> sinks) {
    return sinks.where((source) => !source.name.contains('.monitor')).toList();
  }
}
