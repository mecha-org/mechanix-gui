import 'dart:io';

import 'package:dbus/dbus.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/app_config.dart';
import 'package:mechanix_files/app_route.dart';
import 'package:mechanix_files/load_settings.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/data/file_repository.dart';
import 'package:mechanix_files/src/features/files/data/file_repository_impl.dart';
import 'package:mechanix_files/src/features/files/data/recent_file_manager_repository.dart';
import 'package:mechanix_files/src/features/files/presentation/files.dart';
import 'package:mechanix_files/src/features/files/presentation/files_home.dart';
import 'package:watch_it/watch_it.dart';
import 'package:widgets/mechanix.dart';

Future<void> main(List<String> args) async {
  di.registerSingleton(ThemeToggle());
  WidgetsFlutterBinding.ensureInitialized();
  
  final configResult = await connectToMxconf();
  AppConfig().loadFromMap(configResult);

  final openPath = _parseOpenPath();

  runApp(
    MultiBlocProvider(
      providers: [
        RepositoryProvider<RecentFilesManager>(
          create: (_) => RecentFilesManager(),
        ),
        RepositoryProvider<FileRepository>(
          create: (_) => FileRepositoryImpl(),
        ),
      ],
      child: MechanixFilesApp(openPath: openPath),
    ),
  );
}

String _parseOpenPath() {
  const compileTimeOpenPath = String.fromEnvironment('MECHANIX_FILES_OPEN_PATH');
  final runtimeOpenPath = Platform.environment['MECHANIX_FILES_OPEN_PATH'];
  return compileTimeOpenPath.isNotEmpty ? compileTimeOpenPath : (runtimeOpenPath ?? '');
}

class MechanixFilesApp extends WatchingWidget {
  const MechanixFilesApp({super.key, required this.openPath});
  final String openPath;

  @override
  Widget build(BuildContext context) {
    final themeMode = watchPropertyValue((ThemeToggle t) => t.themeMode);

    return _MechanixFilesAppContent(
      openPath: openPath,
      themeMode: themeMode,
    );
  }
}

class _MechanixFilesAppContent extends StatefulWidget {
  const _MechanixFilesAppContent({
    required this.openPath,
    required this.themeMode,
  });

  final String openPath;
  final ThemeMode themeMode;

  @override
  State<_MechanixFilesAppContent> createState() =>
      _MechanixFilesAppContentState();
}

class _MechanixFilesAppContentState extends State<_MechanixFilesAppContent> {
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
      builder: (context, mechanix, child) => MainApp(
        darkTheme: mechanix.darkTheme,
        lightTheme: mechanix.lightTheme,
        themeMode: widget.themeMode,
        openPath: widget.openPath,
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
    required this.openPath,
  });

  final ThemeData lightTheme;
  final ThemeData darkTheme;
  final ThemeMode themeMode;
  final String openPath;

  @override
  Widget build(BuildContext context) {
    return MultiBlocProvider(
      providers: [
        BlocProvider(
          create: (_) => FilesBloc(
            fileRepository: context.read<FileRepository>(),
            recentFilesManager: context.read<RecentFilesManager>(),
          )..add(InitializeFiles()),
        ),
      ],
      child: MaterialApp(
        debugShowCheckedModeBanner: false,
        theme: _buildCustomTheme(),
        darkTheme: _buildDarkTheme(),
        themeMode: themeMode,
        home: _buildHomePage(),
        routes: {
          AppRoutes.files: (context) => const FileHomePage(),
        },
      ),
    );
  }

  ThemeData _buildCustomTheme() {
    return lightTheme.copyWith(
      scaffoldBackgroundColor: Colors.black,
    );
  }

  ThemeData _buildDarkTheme() {
    return darkTheme.copyWith(
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
      scaffoldBackgroundColor: Colors.black,
      pageTransitionsTheme: const PageTransitionsTheme(
        builders: {
          TargetPlatform.linux: CupertinoPageTransitionsBuilder(),
        },
      ),
    );
  }

  Widget _buildHomePage() {
    return openPath.isNotEmpty
        ? FileExplorerPage(startPath: openPath)
        : const FileHomePage();
  }
}