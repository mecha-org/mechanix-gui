import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:mechanix_notes/app_routes.dart';
import 'package:mechanix_notes/models/note_hive.dart';
import 'package:mechanix_notes/src/commons/styles/colors.dart';
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
    final mechanixVariant = watchPropertyValue(
      (ThemeToggle t) => t.mechanixVariant,
    );

    return MechanixTheme(
      data: MechanixThemeData(
        mechanixVariant: mechanixVariant,
        extensions: const [
          // TODO: FIX THEME
          MechanixFloatingActionBarThemeData(
            padding: EdgeInsets.all(0),
            width: double.infinity,
            // decoration: BoxDecoration(
            //   color: Colors.pink,
            // borderRadius: BorderRadius.only(
            //   topLeft: Radius.circular(12),
            //   topRight: Radius.circular(12),
            // ),
            // ),
          ),
          MechanixSelectableListThemeData(
            // backgroundColor: NotesColors.backgroundColor,
            checkboxSpacing: EdgeInsets.only(right: 16, left: 6),
            leadingIconPadding: EdgeInsets.zero,
            itemPadding: EdgeInsets.only(
              left: 16,
              right: 12,
              top: 10,
              bottom: 10,
            ),
            titleTextStyle: TextStyle(
              fontSize: 16,
              color: NotesColors.titleTextColor,
            ),
          ),
          MechanixNavigationBarThemeData(
            scrolledUnderElevation: 0,
            titleStyle: TextStyle(
              fontSize: 32,
              height: 1.3,
              letterSpacing: -1.1,
              fontWeight: FontWeight.w600,
              color: NotesColors.highlightTextColor,
            ),
            titleSpacing: 16,
            backgroundColor: Colors.transparent,
            elevation: 0,
          ),
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
      localizationsDelegates: const [
        GlobalMaterialLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate,
        GlobalWidgetsLocalizations.delegate,
        FlutterQuillLocalizations.delegate,
      ],
      title: 'Notes',
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
        scrollbarTheme: const ScrollbarThemeData(
          radius: Radius.circular(4),
          thickness: WidgetStatePropertyAll(6),
          thumbColor: WidgetStatePropertyAll(NotesColors.titleTextColor),
        ),

        scaffoldBackgroundColor: Colors.black,
        textSelectionTheme: TextSelectionThemeData(
          cursorColor: NotesColors.secondaryCardColor,
          selectionColor: NotesColors.secondaryCardColor.withValues(alpha: 0.4),
        ),
        // this is temporary fix
        pageTransitionsTheme: const PageTransitionsTheme(
          builders: {TargetPlatform.linux: CupertinoPageTransitionsBuilder()},
        ),
      ),
      themeMode: themeMode,

      home: const HomePage(),
      routes: {
        AppRoutes.createEditNotes:
            (context) => const EditorBlocProvider(child: NotesEditor()),
      },
    );
  }
}
