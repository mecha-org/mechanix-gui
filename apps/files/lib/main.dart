import 'dart:io';

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
  final configResult = await connectToMxconf(); // Await the Future properly

    // Parse command-line arguments
    // Support both compile-time (--dart-define) and runtime (env var)
  const compileTimeOpenPath = String.fromEnvironment('MECHANIX_FILES_OPEN_PATH');
  final runtimeOpenPath = Platform.environment['MECHANIX_FILES_OPEN_PATH'];
  final openPath = compileTimeOpenPath.isNotEmpty 
      ? compileTimeOpenPath 
      : runtimeOpenPath;
  
  print('Open path: $openPath');
  
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
      child: MechanixFilesApp(openPath: openPath ?? ''),
    ),
  );
}

class MechanixFilesApp extends StatelessWidget with WatchItMixin {
  MechanixFilesApp({super.key, required String this.openPath,});
  final String openPath;

  @override
  Widget build(BuildContext context) {
    final themeMode = watchPropertyValue((ThemeToggle t) => t.themeMode);

    return MechanixTheme(
      data: const MechanixThemeData(
        mechanixVariant: MechanixVariant.amber,
      ),
      builder: (context, mechanix, child) => MainApp(
        darkTheme: mechanix.darkTheme,
        lightTheme: mechanix.lightTheme,
        themeMode: themeMode,
        openPath: openPath,
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
