import 'dart:async';

import 'package:logger/web.dart';
import 'package:mechanix_settings/src/features/sound/data/sound_repository.dart';
import 'package:pulseaudio/pulseaudio.dart';

class SoundRepositoryImpl implements SoundRepository {
  final client = PulseAudioClient();
  final logger = Logger();
  bool _connected = false;

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
      return sources;
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
      return sinks;
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

}
