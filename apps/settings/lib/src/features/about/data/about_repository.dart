import 'package:mechanix_settings/src/features/about/models/types.dart';

abstract class AboutRepository {
  Future<AboutDetailsType> getDeviceDetails();
}
