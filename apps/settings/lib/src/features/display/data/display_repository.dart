import 'package:mechanix_settings/src/features/display/models/types.dart';

abstract class DisplayRepository {
  Future<DBusDefaultSettings> getDisplaySettings();
  Future<void> setBrightness(double brightness);
  Future<void> setAutoBrightness(bool autoBrightness);
  Future<void> setScreenTimeout(int timeout);
  Future<void> setLockScreenTimeout(int timeout);
}
