import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:mechanix_notes/app_routes.dart';
import 'package:mechanix_notes/models/note_hive.dart';
import 'package:mechanix_notes/src/features/create_edit_note/create_edit_note.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/data/notes_repository.dart';
import 'package:mechanix_notes/src/features/home/data/notes_repository_impl.dart';
import 'package:mechanix_notes/src/features/home/home.dart';
import 'package:mechanix_notes/src/styles/constants.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:path_provider/path_provider.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await initializeHive();
  Hive.registerAdapter(NoteHiveAdapter());
  await Hive.openBox<NoteHive>(Constants.tableName);
  runApp(
    MultiRepositoryProvider(
      providers: [
        RepositoryProvider<NotesRepository>(
          create: (_) => NotesRepositoryImpl(),
        ),
      ],
      child: BlocProvider(
        create:
            (context) =>
                NotesBloc(notesRepository: context.read<NotesRepository>()),
        child: const MyApp(),
      ),
    ),
  );
}

Future<void> initializeHive() async {
  final appDir = await getApplicationSupportDirectory();

  await Hive.initFlutter(appDir.path);
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

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
      theme: ThemeData.dark(),
      home: HomePage(),
      routes: {AppRoutes.createEditNotes: (context) => CreateEditNote()},
    );
  }
}
