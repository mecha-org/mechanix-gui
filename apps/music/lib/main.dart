import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:media_kit/media_kit.dart';
import 'package:mechanix_music/app_routes.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/features/home/home.dart';
import 'package:mechanix_music/src/features/search_tab/search.dart';
import 'package:watch_it/watch_it.dart';
import 'package:widgets/theme/mechanix_theme.dart';
import 'package:widgets/widgets/theme/theme_toggle.dart';
import 'package:path_provider/path_provider.dart';

void main() async {
  di.registerSingleton(ThemeToggle());
  Hive.registerAdapter(SongInfoAdapter());
  await initializeHive();
  MediaKit.ensureInitialized();

  runApp(BlocProvider(create: (context) => SongsBloc(), child: MainApp()));
}

Future<void> initializeHive() async {
  final appDir = await getApplicationSupportDirectory();
  await Hive.initFlutter(appDir.path);
}

class MainApp extends StatelessWidget with WatchItMixin {
  MainApp({super.key});

  @override
  Widget build(BuildContext context) {
    final themeMode = watchPropertyValue((ThemeToggle t) => t.themeMode);
    final mechanixVariant = watchPropertyValue(
      (ThemeToggle t) => t.mechanixVariant,
    );

    return MechanixTheme(
      data: MechanixThemeData(mechanixVariant: mechanixVariant),
      builder:
          (context, mechanix, child) => MusicApp(
            darkTheme: mechanix.darkTheme,
            lightTheme: mechanix.lightTheme,
            themeMode: themeMode,
          ),
    );
  }
}

class MusicApp extends StatelessWidget {
  const MusicApp({
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
        splashColor: Colors.transparent,
        highlightColor: Colors.transparent,
        hoverColor: Colors.transparent,
        splashFactory: NoSplash.splashFactory,
        iconButtonTheme: const IconButtonThemeData(
          style: ButtonStyle(
            splashFactory: NoSplash.splashFactory,
            overlayColor: WidgetStatePropertyAll(Colors.transparent),
          ),
        ),

        scrollbarTheme: const ScrollbarThemeData(
          radius: Radius.circular(4),
          thickness: WidgetStatePropertyAll(6),
          thumbColor: WidgetStatePropertyAll(MusicColors.scrollBarColor),
        ),

        pageTransitionsTheme: const PageTransitionsTheme(
          builders: {TargetPlatform.linux: CupertinoPageTransitionsBuilder()},
        ),
      ),
      themeMode: themeMode,
      home: HomePage(),
      routes: {AppRoutes.searchPage: (context) => SearchPage()},
    );
  }
}
