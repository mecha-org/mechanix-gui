import 'dart:async';
import 'package:flutter/material.dart';
import 'package:hive_flutter/adapters.dart';
import 'package:media_kit/media_kit.dart';
import 'package:path_provider/path_provider.dart';

import 'src/features/player/presentation/music_player.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();

  // Initialize media_kit
  MediaKit.ensureInitialized();

  Hive.initFlutter();
  final dir = await getApplicationSupportDirectory();
  Hive.init(dir.path);

  // Open box for playlists
  await Hive.openBox<List>('playlistBox');
  await Hive.openBox<List>('metaBox');

  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'Music Player',
      theme: appTheme,
      home: const MusicPlayerPage(),
    );
  }
}

final ThemeData appTheme = ThemeData(
  brightness: Brightness.dark,
  scaffoldBackgroundColor: const Color.fromRGBO(26, 36, 50, 1), // background
  primaryColor: const Color(0xFF2C4C7B), // accent
  secondaryHeaderColor: const Color.fromARGB(
    255,
    145,
    177,
    226,
  ), // buttons, highlights
  cardColor: const Color(0xFF2D3E50), // playlist cards
  textTheme: const TextTheme(
    titleLarge: TextStyle(
      color: Colors.white,
      fontSize: 18,
      fontWeight: FontWeight.bold,
    ),
    titleMedium: TextStyle(color: Color(0xFFB0B6C1), fontSize: 12),
    bodyLarge: TextStyle(color: Color(0xFF7D8796), fontSize: 14),
  ),
  iconTheme: const IconThemeData(color: Colors.white),
  appBarTheme: const AppBarTheme(
    backgroundColor: Color(0xFF2D3E50),
    elevation: 0,
    titleTextStyle: TextStyle(
      color: Colors.white,
      fontSize: 20,
      fontWeight: FontWeight.bold,
    ),
    iconTheme: IconThemeData(color: Colors.white),
  ),

  tabBarTheme: const TabBarTheme(
    labelColor: Color.fromARGB(255, 145, 177, 226), // selected tab text/icon
    unselectedLabelColor: Colors.white, // unselected tab text/icon
    indicator: UnderlineTabIndicator(
      borderSide: BorderSide(
        color: Color.fromARGB(255, 145, 177, 226),
        width: 3,
      ), // underline color
    ),
  ),
);
