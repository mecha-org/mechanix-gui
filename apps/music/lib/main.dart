import 'dart:io';

import 'package:dbus/dbus.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:mechanix_music/load_settings.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/models/recently_played.dart';
import 'package:mechanix_music/models/search_data.dart';
import 'package:mechanix_music/src/features/home/data/songs_repository.dart';
import 'package:mechanix_music/src/features/home/data/songs_repository_impl.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/features/home/home.dart';
import 'package:watch_it/watch_it.dart';
import 'package:widgets/theme/mechanix_theme.dart';
import 'package:widgets/theme/variants.dart';
import 'package:widgets/widgets/theme/theme_toggle.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
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
        child: MechanixMusicApp(),
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

class MechanixMusicApp extends StatelessWidget with WatchItMixin {
  MechanixMusicApp({super.key});

  @override
  Widget build(BuildContext context) {
    final themeMode = watchPropertyValue((ThemeToggle t) => t.themeMode);
    return _MechanixMusicAppContent(themeMode: themeMode);
  }
}

class _MechanixMusicAppContent extends StatefulWidget {
  const _MechanixMusicAppContent({required this.themeMode});

  final ThemeMode themeMode;

  @override
  State<_MechanixMusicAppContent> createState() =>
      _MechanixMusicAppContentState();
}

class _MechanixMusicAppContentState extends State<_MechanixMusicAppContent> {
  late final DBusClient _bus;
  late final ThemeSettingsService _themeService;

  MechanixThemeData _currentThemeData = MechanixThemeData(
    mechanixVariant: MechanixVariant.amber,
  );

  @override
  void initState() {
    super.initState();
    _initializeThemeService();
  }

  void _initializeThemeService() {
    _bus = DBusClient.session();
    _themeService = ThemeSettingsService(_bus);

    _themeService.listenForThemeChanges(_handleThemeChange);
    _fetchInitialTheme();
  }

  Future<void> _fetchInitialTheme() async {
    final colors = await _themeService.fetchCurrentTheme();
    if (colors != null) {
      _handleThemeChange(colors);
    }
  }

  void _handleThemeChange(Map<String, String> colors) {
    setState(() {
      _currentThemeData = _themeService.colorsToThemeData(colors);
    });
  }

  @override
  void dispose() {
    _themeService.dispose();
    _bus.close();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return MechanixTheme(
      data: _currentThemeData,
      builder:
          (context, mechanix, child) => MainApp(
            darkTheme: mechanix.darkTheme,
            lightTheme: mechanix.lightTheme,
            themeMode: widget.themeMode,
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

        pageTransitionsTheme: const PageTransitionsTheme(
          builders: {TargetPlatform.linux: CupertinoPageTransitionsBuilder()},
        ),
      ),
      darkTheme: darkTheme.copyWith(
        splashColor: Colors.transparent,
        highlightColor: Colors.transparent,
        hoverColor: Colors.transparent,
        splashFactory: NoSplash.splashFactory,

        pageTransitionsTheme: const PageTransitionsTheme(
          builders: {TargetPlatform.linux: CupertinoPageTransitionsBuilder()},
        ),
      ),
      themeMode: themeMode,
      home: HomePage(),
    );
  }
}
