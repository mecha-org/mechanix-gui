import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:hive/hive.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:mechanix_music/src/features/home/custom_slider.dart';
import 'package:media_kit/media_kit.dart';
import 'package:mechanix_music/app_routes.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/features/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/features/home/home.dart';
import 'package:mechanix_music/src/features/search/search.dart';
import 'package:watch_it/watch_it.dart';
import 'package:widgets/theme/mechanix_theme.dart';
import 'package:widgets/widgets/theme/theme_toggle.dart';
import 'package:path_provider/path_provider.dart';

void main() async {
  di.registerSingleton(ThemeToggle());
  Hive.registerAdapter(SongInfoAdapter());
  await initializeHive();
  MediaKit.ensureInitialized();

  runApp(BlocProvider(create: (context) => SongsBloc(), child: MusicApp()));
}

Future<void> initializeHive() async {
  final appDir = await getApplicationSupportDirectory();
  await Hive.initFlutter(appDir.path);
}

class MusicApp extends StatelessWidget with WatchItMixin {
  MusicApp({super.key});

  @override
  Widget build(BuildContext context) {
    final themeMode = watchPropertyValue((ThemeToggle t) => t.themeMode);
    final mechanixVariant = watchPropertyValue(
      (ThemeToggle t) => t.mechanixVariant,
    );

    return MechanixTheme(
      data: MechanixThemeData(mechanixVariant: mechanixVariant, extensions: [
        ],
      ),
      builder:
          (context, mechanix, child) => MyApp(
            darkTheme: mechanix.darkTheme,
            lightTheme: mechanix.lightTheme,
            themeMode: themeMode,
          ),
    );
  }
}

class MyApp extends StatelessWidget {
  const MyApp({
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
      title: 'Flutter Demo',
      theme: lightTheme,
      darkTheme: darkTheme.copyWith(
        pageTransitionsTheme: PageTransitionsTheme(
          builders: {TargetPlatform.linux: SlideLeftTransitionsBuilder()},
        ),
      ),
      themeMode: themeMode,
      home: HomePage(),
      routes: {AppRoutes.searchPage: (context) => SearchPage()},
    );
  }
}
