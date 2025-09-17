import 'package:mechanix_settings/src/features/about/data/about_repository.dart';
import 'package:mechanix_settings/src/features/about/data/dbus_about_service.dart';
import 'package:mechanix_settings/src/features/about/models/types.dart';

class AboutRepositoryImpl extends AboutRepository {
  final DBusAboutService dBusAboutService = DBusAboutService();

  @override
  Future<AboutDetailsType> getDeviceDetails() {
    return dBusAboutService.getDeviceDetails();
  }
}
