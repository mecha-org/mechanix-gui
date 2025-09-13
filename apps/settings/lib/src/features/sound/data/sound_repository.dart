
import 'package:pulseaudio/pulseaudio.dart';

abstract class SoundRepository {

  Future<Stream<PulseAudioServerInfo>> streamSoundServerEvents();
  Future<Stream<PulseAudioSource>> streamSoundSourceEvents();
  Future<Stream<PulseAudioSink>> streamSoundSinkEvents();


  Future<String> getDefaultSourceName();
  Future<String> getDefaultSinkName();

  Future<List<PulseAudioSource>> getSourceList();
  Future<List<PulseAudioSink>> getSinkList();

  Future<void> setDefaultSource(String device);
  Future<void> setDefaultSink(String device);

  Future<void> setSinkMute(String sinkName, bool mute);
  Future<void> setSinkVolume(String sinkName, double volume);

  Future<void> setSourceMute(String sourceName, bool mute);
  Future<void> setSourceVolume(String sourceName, double volume);

}
