import 'dart:async';
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
import 'package:widgets/constants.dart';
import 'package:widgets/mechanix.dart';

Future<void> main(List<String> args) async {
  di.registerSingleton(ThemeToggle());

  WidgetsFlutterBinding.ensureInitialized();
  final configResult = await connectToMxconf();

  // Parse command-line arguments
  const compileTimeOpenPath =
      String.fromEnvironment('MECHANIX_FILES_OPEN_PATH');
  final runtimeOpenPath = Platform.environment['MECHANIX_FILES_OPEN_PATH'];
  final openPath =
      compileTimeOpenPath.isNotEmpty ? compileTimeOpenPath : runtimeOpenPath;

  AppConfig().loadFromMap(configResult);

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
      child: MechanixFilesApp(openPath: openPath ?? ''),
    ),
  );
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
  final _bus = DBusClient.session();
  StreamSubscription<DBusSignal>? _signalSubscription;

  MechanixThemeData _currentThemeData = MechanixThemeData(
    mechanixVariant: MechanixVariant.amber,
  );

  @override
  void initState() {
    super.initState();
    listenForSettingsChange();
    _fetchInitialTheme();
  }

  @override
  void dispose() {
    _signalSubscription?.cancel();
    _bus.close();
    super.dispose();
  }

  void listenForSettingsChange() {
    final remoteObj = DBusRemoteObject(
      _bus,
      name: 'org.mechanix.MxConf',
      path: DBusObjectPath('/org/mechanix/MxConf'),
    );

    _signalSubscription = DBusRemoteObjectSignalStream(
      object: remoteObj,
      interface: 'org.mechanix.MxConf',
      name: 'SchemaKeyChanged',
    ).listen((signal) {
      if (signal.values.length >= 3) {
        final schema = (signal.values[0] as DBusString).value;
        final key = (signal.values[1] as DBusString).value;
        final rawValue = (signal.values[2] as DBusString).value;

        if (schema == 'org.mechanix.desktop' &&
            key == 'settings.active_theme.theme_colors') {
          final colors = _parseThemeColors(rawValue);
          if (colors != null) {
            _applyThemeColors(colors);
          }
        }
      }
    });
  }

  Map<String, String>? _parseThemeColors(String description) {
    final accentMatch =
        RegExp(r'accent\s*=\s*"([^"]+)"').firstMatch(description);
    final backgroundMatch =
        RegExp(r'background\s*=\s*"([^"]+)"').firstMatch(description);
    final foregroundMatch =
        RegExp(r'foreground\s*=\s*"([^"]+)"').firstMatch(description);

    if (accentMatch == null ||
        backgroundMatch == null ||
        foregroundMatch == null) {
      print('Failed to parse all theme colors');
      return null;
    }

    return {
      'accent': accentMatch.group(1)!,
      'background': backgroundMatch.group(1)!,
      'foreground': foregroundMatch.group(1)!,
    };
  }

  Future<void> _fetchInitialTheme() async {
    print('Fetching initial theme from DBus...');
    try {
      final remoteObj = DBusRemoteObject(
        _bus,
        name: 'org.mechanix.MxConf',
        path: DBusObjectPath('/org/mechanix/MxConf'),
      );

      const key = "org.mechanix.desktop.settings.active_theme.theme_colors";

      final response = await remoteObj.callMethod(
        'org.mechanix.MxConf',
        'GetSetting',
        [const DBusString(key)],
      );

      if (response.returnValues.isNotEmpty &&
          response.returnValues[0] is DBusDict) {
        final dict = response.returnValues[0] as DBusDict;
        final dbusValue = dict.children[DBusString(key)];
        if (dbusValue is DBusString) {
          final colors = _parseThemeColors(dbusValue.value);
          if (colors != null) {
            _applyThemeColors(colors);
          }
        }
      }
    } catch (e) {
      print('Error fetching initial theme: $e');
    }
  }

  void _applyThemeColors(Map<String, String> colors) {
    final accent = colors['accent'];
    final background = colors['background'];
    final foreground = colors['foreground'];

    final updatedThemeData = MechanixThemeData(
      mechanixVariant: MechanixVariant.custom(
          accent?.toOKLCHStringToColor() ?? Colors.amber),
      mechanixBackgroundVariant: MechanixVariant.custom(
          background?.toOKLCHStringToColor() ?? defaultBackgroundColor),
      mechanixForegroundVariant: MechanixVariant.custom(
          foreground?.toOKLCHStringToColor() ??  defaultForegroundColor),
    );

    setState(() {
      _currentThemeData = updatedThemeData;
    });

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
              recentFilesManager: context.read<RecentFilesManager>())
            ..add(InitializeFiles()),
        ),
      ],
      child: MaterialApp(
        debugShowCheckedModeBanner: false,
        theme: darkTheme.copyWith(scaffoldBackgroundColor: Colors.black),
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
          scaffoldBackgroundColor: Colors.black,
          pageTransitionsTheme: const PageTransitionsTheme(
            builders: {TargetPlatform.linux: CupertinoPageTransitionsBuilder()},
          ),
        ),
        themeMode: themeMode,
        home: openPath.isNotEmpty
            ? FileExplorerPage(startPath: openPath)
            : const FileHomePage(),
        routes: {
          AppRoutes.files: (context) => const FileHomePage(),
        },
      ),
    );
  }
}
