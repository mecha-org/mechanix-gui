import 'dart:io';
import 'package:dbus/dbus.dart';
import 'package:flutter/material.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:mechanix_notes/app_routes.dart';
import 'package:mechanix_notes/load_settings.dart';
import 'package:mechanix_notes/models/note_hive.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_bloc_provider.dart';
import 'package:mechanix_notes/src/features/editor/notes_editor.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/data/notes_repository.dart';
import 'package:mechanix_notes/src/features/home/data/notes_repository_impl.dart';
import 'package:mechanix_notes/src/features/home/home.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:path_provider/path_provider.dart';
import 'package:widgets/mechanix.dart';
import 'package:watch_it/watch_it.dart';
import 'package:widgets/widgets/floating_action_bar/mechanix_floating_action_bar_theme.dart';
import 'package:widgets/widgets/navigation_bar/mechanix_navigation_bar_theme.dart';
import 'package:widgets/widgets/pressable_list/mechanix_pressable_list_theme.dart';

void main() async {
  di.registerSingleton(ThemeToggle());
  WidgetsFlutterBinding.ensureInitialized();
  await initializeHive();
  Hive.registerAdapter(NoteHiveAdapter());
  runApp(
    MultiRepositoryProvider(
      providers: [
        RepositoryProvider<NotesRepository>(
          create: (_) => NotesRepositoryImpl(),
        ),
      ],
      child: MultiBlocProvider(
        providers: [
          BlocProvider(
            create:
                (context) =>
                    NotesBloc(notesRepository: context.read<NotesRepository>()),
          ),
        ],
        child: NotesApp(),
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
  final appDir = Directory('$baseDir/mechanix_notes');
  print(appDir.path);
  if (!await appDir.exists()) {
    await appDir.create(recursive: true);
  }

  Hive.init(appDir.path);
}

class NotesApp extends StatelessWidget with WatchItMixin {
  NotesApp({super.key});

  @override
  Widget build(BuildContext context) {
    final themeMode = watchPropertyValue((ThemeToggle t) => t.themeMode);

    return _MechanixNotesAppContent(themeMode: themeMode);
  }
}

class _MechanixNotesAppContent extends StatefulWidget {
  const _MechanixNotesAppContent({required this.themeMode});

  final ThemeMode themeMode;

  @override
  State<_MechanixNotesAppContent> createState() =>
      _MechanixNotesAppContentState();
}

class _MechanixNotesAppContentState extends State<_MechanixNotesAppContent> {
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
          (context, mechanix, child) => MyApp(
            darkTheme: mechanix.darkTheme,
            lightTheme: mechanix.lightTheme,
            themeMode: widget.themeMode,
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
      localizationsDelegates: const [
        GlobalMaterialLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate,
        GlobalWidgetsLocalizations.delegate,
        FlutterQuillLocalizations.delegate,
      ],
      title: 'Notes',
      theme: darkTheme.copyWith(scaffoldBackgroundColor: Colors.black),
      darkTheme: _buildDarkTheme(),
      themeMode: themeMode,
      home: const HomePage(),
      routes: {
        AppRoutes.createEditNotes:
            (context) => const EditorBlocProvider(child: NotesEditor()),
      },
    );
  }

  ThemeData _buildDarkTheme() {
    return darkTheme.copyWith(
      scaffoldBackgroundColor: Colors.black,
      pageTransitionsTheme: const PageTransitionsTheme(
        builders: {TargetPlatform.linux: CupertinoPageTransitionsBuilder()},
      ),
    );
  }
}
