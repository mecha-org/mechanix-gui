import 'dart:io';

import 'package:dbus/dbus.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:hive/hive.dart';
import 'package:mechanix_camera/data/camera_repository.dart';
import 'package:mechanix_camera/data/camera_repository_impl.dart';
import 'package:mechanix_camera/db/camera_config.dart';
import 'package:mechanix_camera/src/app_routes.dart';
import 'package:mechanix_camera/src/bloc/camera_bloc.dart';
import 'package:mechanix_camera/src/media/media_view.dart';
import 'package:mechanix_camera/src/home/home.dart';
import 'package:mechanix_camera/load_settings.dart';
import 'package:watch_it/watch_it.dart';
import 'package:widgets/theme/mechanix_theme.dart';
import 'package:widgets/theme/variants.dart';
import 'package:widgets/widgets/theme/theme_toggle.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  di.registerSingleton(ThemeToggle());
  Hive.registerAdapter(CameraConfigAdapter());
  Hive.registerAdapter(CaptureModeAdapter());
  Hive.registerAdapter(SoundModeAdapter());
  Hive.registerAdapter(CameraTimerAdapter());
  Hive.registerAdapter(CameraGridAdapter());
  Hive.registerAdapter(CameraResolutionAdapter());
  Hive.registerAdapter(CameraAspectRatioAdapter());
  await initializeHive();

  runApp(
    MultiRepositoryProvider(
      providers: [
        RepositoryProvider<CameraRepository>(
          create: (_) => CameraRepositoryImpl(),
        ),
      ],
      child: MultiBlocProvider(
        providers: [
          BlocProvider(
            create:
                (context) =>
                    CameraBloc(repository: context.read<CameraRepository>()),
          ),
        ],
        child: MechanixCameraApp(),
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
  final appDir = Directory('$baseDir/mechanix_camera');
  if (!await appDir.exists()) {
    await appDir.create(recursive: true);
  }
  Hive.init(appDir.path);
}

class MechanixCameraApp extends StatelessWidget with WatchItMixin {
  MechanixCameraApp({super.key});

  @override
  Widget build(BuildContext context) {
    final themeMode = watchPropertyValue((ThemeToggle t) => t.themeMode);
    return _MechanixCameraAppContent(themeMode: themeMode);
  }
}

class _MechanixCameraAppContent extends StatefulWidget {
  const _MechanixCameraAppContent({required this.themeMode});

  final ThemeMode themeMode;

  @override
  State<_MechanixCameraAppContent> createState() =>
      _MechanixCameraAppContentState();
}

class _MechanixCameraAppContentState extends State<_MechanixCameraAppContent> {
  late final DBusClient _bus;
  late final ThemeSettingsService _themeService;

  MechanixThemeData _currentThemeData = const MechanixThemeData(
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
      title: 'Camera UI',
      debugShowCheckedModeBanner: false,
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
      home: const CameraScreen(),
      routes: {
        AppRoutes.home: (context) => const CameraScreen(),
        AppRoutes.media: (context) => const MediaView(),
      },
    );
  }
}
