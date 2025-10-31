import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/app_config.dart';
import 'package:mechanix_files/app_route.dart';
import 'package:mechanix_files/load_settings.dart';
import 'package:mechanix_files/slider_left_transition_builder.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/data/file_repository.dart';
import 'package:mechanix_files/src/features/files/data/file_repository_impl.dart';
import 'package:mechanix_files/src/features/files/data/recent_file_manager_repository.dart';
import 'package:mechanix_files/src/features/files/presentation/file_search.dart';
import 'package:mechanix_files/src/features/files/presentation/files_home.dart';
import 'package:watch_it/watch_it.dart';
import 'package:widgets/mechanix.dart';

Future<void> main() async {
  di.registerSingleton(ThemeToggle());

  WidgetsFlutterBinding.ensureInitialized();
  final configResult = await connectToMxconf(); // Await the Future properly
  AppConfig().loadFromMap(configResult); // Load into singleton instance

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
      child: MechanixFilesApp(),
    ),
  );
}

class MechanixFilesApp extends StatelessWidget with WatchItMixin {
  MechanixFilesApp({super.key});

  @override
  Widget build(BuildContext context) {
    final themeMode = watchPropertyValue((ThemeToggle t) => t.themeMode);
    final mechanixVariant =
        watchPropertyValue((ThemeToggle t) => t.mechanixVariant);

    return MechanixTheme(
      data: MechanixThemeData(
        mechanixVariant: mechanixVariant,
      ),
      builder: (context, mechanix, child) => MainApp(
        darkTheme: mechanix.darkTheme,
        lightTheme: mechanix.lightTheme,
        themeMode: themeMode,
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
        theme: lightTheme,
        darkTheme: darkTheme.copyWith(
            pageTransitionsTheme: const PageTransitionsTheme(builders: {
          TargetPlatform.linux: CupertinoPageTransitionsBuilder()
          // SlideLeftTransitionsBuilder()
        })),
        themeMode: themeMode,
        home: const FileHomePage(),
        routes: {
          AppRoutes.files: (context) => const FileHomePage(),
          AppRoutes.searchFiles: (context) => const FileSearchPage(),
        },
      ),
    );
  }
}
