import 'dart:async';
import 'package:music_player/app_routes.dart';
import 'package:music_player/src/features/player/presentation/music_player_home.dart';
import 'package:flutter/material.dart';
import 'package:hive_flutter/adapters.dart';
import 'package:media_kit/media_kit.dart';
import 'package:path_provider/path_provider.dart';
import 'package:watch_it/watch_it.dart';
import 'package:widgets/mechanix.dart';


Future<void> main() async {
  di.registerSingleton(ThemeToggle());
  WidgetsFlutterBinding.ensureInitialized();

  // Initialize media_kit
  MediaKit.ensureInitialized();

  Hive.initFlutter();
  final dir = await getApplicationSupportDirectory();
  Hive.init(dir.path);

  // Open box for playlists
  await Hive.openBox<List>('recentBox'); 
  await Hive.openBox<List>('playlistBox');
  await Hive.openBox<List>('metaBox');

  runApp(MechanixSettingsApp());
}

class MechanixSettingsApp extends StatelessWidget with WatchItMixin {
  MechanixSettingsApp({super.key});

  @override
  Widget build(BuildContext context) {
    final themeMode = watchPropertyValue((ThemeToggle t) => t.themeMode);
    final mechanixVariant =
        watchPropertyValue((ThemeToggle t) => t.mechanixVariant);

    return MechanixTheme(
      data: MechanixThemeData(
        mechanixVariant: mechanixVariant,
      ),
      builder: (context, mechanix, child) => MainApp(
        darkTheme: mechanix.darkTheme,
        lightTheme: mechanix.lightTheme,
        themeMode: themeMode,
      ),
    );
  }
}

class MainApp extends StatelessWidget {
   const MainApp({
    super.key,
    required this.lightTheme,
    required this.darkTheme,
    required this.themeMode,
  });

    final ThemeData lightTheme;
  final ThemeData darkTheme;
  final ThemeMode themeMode;


  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'Music',
      theme: lightTheme,
      darkTheme: darkTheme,
      themeMode: themeMode,
      home: const MusicPlayerPage(),
      // routes: AppRoutes.player: 
              

    );
  }
}