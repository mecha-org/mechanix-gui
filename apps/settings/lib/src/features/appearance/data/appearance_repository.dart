import 'package:widgets/mechanix.dart';

abstract class AppearanceRepository {
  Future<Map<String, String>?> onInit();

  Future<void> applyTheme(MechanixVariant variant);

  Future<void> applyWallpaper(String wallpaper);
}
