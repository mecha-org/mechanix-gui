import 'dart:io';
import 'package:record/record.dart';
import 'package:path_provider/path_provider.dart';
import 'package:path/path.dart' as path;

class LinuxAudioRecorder {
  static final AudioRecorder _recorder = AudioRecorder();
  static String? _currentRecordingPath;

  /// Check if audio recording is available (permissions + required tools)
  // static Future<bool> isRecordingAvailable() async {
  //   try {
  //     return await _recorder.hasPermission();
  //   } catch (_) {
  //     return false;
  //   }
  // }

  /// Start audio recording
  static Future<String?> startRecording() async {
    try {
      // if (!await isRecordingAvailable()) {
      //   throw Exception(
      //     'Audio recording not available. Check microphone permissions or required tools (parecord/ffmpeg).',
      //   );
      // }

      final appDir = await getApplicationSupportDirectory();
      final fileName = 'audio_${DateTime.now().millisecondsSinceEpoch}.m4a';
      final filePath = path.join(appDir.path, fileName);

      // Ensure directory exists
      await Directory(path.dirname(filePath)).create(recursive: true);
      // Start recording
      await _recorder.start(
        const RecordConfig(encoder: AudioEncoder.aacLc),
        path: filePath,
      );

      _currentRecordingPath = filePath;
      return filePath;
    } catch (e) {
      print("error $e");
      return null;
    }
  }

  /// Stop recording and return file path
  static Future<String?> stopRecording() async {
    try {
      final recordedPath = await _recorder.stop();
      final pathToCheck = recordedPath ?? _currentRecordingPath;
      _currentRecordingPath = null;

      if (pathToCheck != null) {
        final file = File(pathToCheck);
        if (await file.exists() && await file.length() > 0) {
          return pathToCheck;
        }
      }
      return null;
    } catch (_) {
      return null;
    }
  }

  /// Cancel recording (delete file)
  static Future<void> cancelRecording() async {
    try {
      await _recorder.cancel();
      _currentRecordingPath = null;
    } catch (_) {}
  }

  /// Check if currently recording
  static Future<bool> get isRecording async {
    try {
      return await _recorder.isRecording();
    } catch (_) {
      return false;
    }
  }

  /// Clean up resources
  static Future<void> dispose() async {
    await _recorder.dispose();
  }
}
