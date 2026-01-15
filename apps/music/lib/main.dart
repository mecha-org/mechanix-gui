import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/models/recently_played.dart';
import 'package:mechanix_music/models/search_data.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/features/home/data/songs_repository.dart';
import 'package:mechanix_music/src/features/home/data/songs_repository_impl.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/features/home/home.dart';
import 'package:watch_it/watch_it.dart';
import 'package:widgets/theme/mechanix_theme.dart';
import 'package:widgets/theme/variants.dart';
import 'package:widgets/widgets/bottom_sheet_modals/mechanix_bottom_sheet_theme.dart';
import 'package:widgets/widgets/theme/theme_toggle.dart';

void main() async {
  di.registerSingleton(ThemeToggle());
  Hive.registerAdapter(SongInfoAdapter());
  Hive.registerAdapter(RecentlyPlayedAdapter());
  Hive.registerAdapter(PlaylistInfoAdapter());
  Hive.registerAdapter(SearchDataAdapter());
  await initializeHive();
  runApp(
    MultiRepositoryProvider(
      providers: [
        RepositoryProvider<SongsRepository>(
          create: (_) => SongsRepositoryImpl(),
        ),
      ],
      child: MultiBlocProvider(
        providers: [
          BlocProvider(
            create:
                (context) =>
                    SongsBloc(songsRepository: context.read<SongsRepository>()),
          ),
        ],
        child: MainApp(),
      ),
    ),
  );
}

Future<void> initializeHive() async {
  final home = Platform.environment['HOME'];
  final xdgConfig = Platform.environment['XDG_CONFIG_HOME'];

  if (home == null && xdgConfig == null) {
    throw Exception('Cannot determine home directory');
  }

  final baseDir = xdgConfig ?? '$home/.config';
  final appDir = Directory('$baseDir/mechanix_music');
  if (!await appDir.exists()) {
    await appDir.create(recursive: true);
  }
  Hive.init(appDir.path);
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
      data: MechanixThemeData(
        mechanixVariant: MechanixVariant.amber,
        extensions: [
          MechanixBottomSheetThemeData(
            decoration: BoxDecoration(
              color: Color(0xFF2E2E2E),
              borderRadius: BorderRadius.only(
                topLeft: Radius.circular(16),
                topRight: Radius.circular(16),
              ),
            ),
          ),
          // MechanixMenuThemeData(
          //   decoration: BoxDecoration(color: context.colorScheme.tertiary),
          //   itemBackgroundColor: context.colorScheme.tertiary,
          // ),
        ],
      ),
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
      theme: darkTheme.copyWith(
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
          thumbColor: WidgetStatePropertyAll(MusicColors.primaryTextColor),
        ),

        pageTransitionsTheme: const PageTransitionsTheme(
          builders: {TargetPlatform.linux: CupertinoPageTransitionsBuilder()},
        ),
      ),
      darkTheme: darkTheme.copyWith(
        splashColor: Colors.transparent,
        highlightColor: Colors.transparent,
        hoverColor: Colors.transparent,
        splashFactory: NoSplash.splashFactory,

        // iconButtonTheme: const IconButtonThemeData(
        //   style: ButtonStyle(
        //     splashFactory: NoSplash.splashFactory,

        //     overlayColor: WidgetStatePropertyAll(Colors.transparent),
        //   ),
        // ),
        scrollbarTheme: const ScrollbarThemeData(
          radius: Radius.circular(4),
          thickness: WidgetStatePropertyAll(6),
          thumbColor: WidgetStatePropertyAll(MusicColors.primaryTextColor),
        ),

        pageTransitionsTheme: const PageTransitionsTheme(
          builders: {TargetPlatform.linux: CupertinoPageTransitionsBuilder()},
        ),
      ),
      themeMode: themeMode,
      home: HomePage(),
    );
  }
}
