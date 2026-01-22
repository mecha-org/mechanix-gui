import 'package:dbus/dbus.dart';
import 'package:mechanix_settings/load_settings.dart';
import 'package:mechanix_settings/src/features/appearance/data/appearance_repository.dart';
import 'package:widgets/theme/variants.dart';

class AppearanceRepositoryImpl extends AppearanceRepository {
  @override
  Future<({Map<String, String>? currentTheme, String? currentWallPaper})?>
      onInit() async {
    final dBus = DBusClient.session();
    try {
      final themeService = ThemeSettingsService(dBus);

      final currentTheme = await themeService.fetchCurrentTheme();
      final currentWallPaper = await themeService.getWallpaper();

      return (currentTheme: currentTheme, currentWallPaper: currentWallPaper);
    } catch (e) {
      print("Error on initialize Appearance repository: $e");
      return null;
    } finally {
      await dBus.close();
    }
  }

  @override
  Future<void> applyTheme(MechanixVariant variant) async {
    final dBus = DBusClient.session();
    try {
      final themeService = ThemeSettingsService(dBus);

      await themeService.setCurrentTheme(variant);
    } catch (e) {
      print("Error in Updating theme");
    } finally {
      dBus.close();
    }
  }

  @override
  Future<void> applyWallpaper(String wallpaper) async {
    final dBus = DBusClient.session();
    try {
      final themeService = ThemeSettingsService(dBus);

      await themeService.setWallpaper(wallpaper);
    } catch (e) {
      print("Error in Updating theme");
    } finally {
      dBus.close();
    }
  }
}
