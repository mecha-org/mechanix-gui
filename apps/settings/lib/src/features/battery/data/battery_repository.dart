import 'package:mechanix_settings/src/features/battery/models/battery_info.dart';

abstract class BatteryRepository {
  Future<BatteryInfo> getBatteryInfo();
  Future<String> setBatteryMode(String mode);
  Future<Stream<List<String>>?> streamBatteryEvents();
}
