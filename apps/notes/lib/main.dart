import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:mechanix_notes/app_routes.dart';
import 'package:mechanix_notes/models/note_hive.dart';
import 'package:mechanix_notes/src/features/editor/bloc/editor_bloc_provider.dart';
import 'package:mechanix_notes/src/features/editor/notes_editor.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/data/notes_repository.dart';
import 'package:mechanix_notes/src/features/home/data/notes_repository_impl.dart';
import 'package:mechanix_notes/src/features/home/home.dart';
import 'package:mechanix_notes/src/constants/constants.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/features/search_notes/presentation/search_notes.dart';
// import 'package:media_kit/media_kit.dart';
import 'package:path_provider/path_provider.dart';
import 'package:widgets/mechanix.dart';
import 'package:watch_it/watch_it.dart';

void main() async {
  di.registerSingleton(ThemeToggle());
  WidgetsFlutterBinding.ensureInitialized();
  await initializeHive();
  Hive.registerAdapter(NoteHiveAdapter());
  // MediaKit.ensureInitialized();
  await Hive.openBox<NoteHive>(Constants.tableName);
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
  final appDir = await getApplicationSupportDirectory();
  await Hive.initFlutter(appDir.path);
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
      data: MechanixThemeData(mechanixVariant: mechanixVariant),
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
        scaffoldBackgroundColor: Colors.black,
        textSelectionTheme: const TextSelectionThemeData(
          cursorColor: Colors.white,
        ),
        pageTransitionsTheme: const PageTransitionsTheme(
          builders: {TargetPlatform.linux: CupertinoPageTransitionsBuilder()},
        ),
      ),
      themeMode: themeMode,

      home: const HomePage(),
      routes: {
        AppRoutes.createEditNotes:
            (context) => const EditorBlocProvider(child: NotesEditor()),
        AppRoutes.searchNotes: (context) => const SearchNotes(),
      },
    );
  }
}
