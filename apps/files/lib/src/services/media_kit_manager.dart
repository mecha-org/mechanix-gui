import 'package:media_kit/media_kit.dart';

class MediaKitManager {
  static bool _initialized = false;

  static Future<void> init() async {
    if (!_initialized) {
      MediaKit.ensureInitialized();
      _initialized = true;
    }
  }
}
