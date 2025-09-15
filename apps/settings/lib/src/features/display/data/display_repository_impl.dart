import 'package:logger/web.dart';
import 'package:mechanix_settings/src/features/display/data/dbus_display_service.dart';
import 'package:mechanix_settings/src/features/display/data/display_repository.dart';
import 'package:mechanix_settings/src/features/display/models/types.dart';

class DisplayRepositoryImpl extends DisplayRepository {
  final logger = Logger();
  final DBusDisplayService _dBusDisplayService = DBusDisplayService();

  @override
  Future<DBusDefaultSettings> getDisplaySettings() {
    return _dBusDisplayService.getDisplaySettings();
  }

  @override
  setBrightness(int brightness) {
    return _dBusDisplayService.setBrightness(brightness);
  }

  @override
  Future<void> setAutoBrightness(bool isAutoBrightness) {
    return _dBusDisplayService.setAutoBrightness(isAutoBrightness);
  }

  @override
  Future<void> setScreenTimeout(int timeoutMinutes) {
    return _dBusDisplayService.setScreenTimeout(timeoutMinutes);
  }

  @override
  Future<void> setLockScreenTimeout(int timeoutMinutes) {
    return _dBusDisplayService.setLockScreenTimeout(timeoutMinutes);
  }
}
