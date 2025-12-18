import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/app_config.dart';
import 'package:mechanix_files/app_route.dart';
import 'package:mechanix_files/load_settings.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/commons/styles/file_theme_extenstions.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/data/file_repository.dart';
import 'package:mechanix_files/src/features/files/data/file_repository_impl.dart';
import 'package:mechanix_files/src/features/files/data/recent_file_manager_repository.dart';
import 'package:mechanix_files/src/features/files/presentation/files_home.dart';
import 'package:watch_it/watch_it.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/section_list/mechanix_section_list_theme.dart';
import 'package:widgets/widgets/text_input/mechanix_text_input_theme.dart';

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
        extensions: [
          const FilesTheme(),
          const MechanixSectionListThemeData(
            backgroundColor: WidgetStatePropertyAll(
              FilesThemeConstants.sectionListBackgroundColor,
            ),
            titleTextStyle: TextStyle(
              fontSize: 18,
              color: FilesThemeConstants.titleTextColor,
              fontWeight: FontWeight.w500,
              fontFamily: FilesThemeConstants.fontFamily,
            ),
          ),
          MechanixTextInputThemeData(
            fillColor: const Color(0xFF151515),
            borderSide: const BorderSide(color: Color(0xFF151515)),
            focusedBorderSide: const BorderSide(color: Color(0xFF151515)),
            borderRadius: BorderRadius.circular(8),
          ),
        ],
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
          textSelectionTheme: TextSelectionThemeData(
            cursorColor: FilesThemeConstants.primaryColor,
            selectionColor:
                FilesThemeConstants.primaryColor.withValues(alpha: 0.4),
            selectionHandleColor: FilesThemeConstants.primaryColor,
          ),
          scrollbarTheme: const ScrollbarThemeData(
            radius: Radius.circular(4),
            thickness: WidgetStatePropertyAll(6),
            thumbColor: WidgetStatePropertyAll(FilesThemeConstants.thumbColor),
          ),
          pageTransitionsTheme: const PageTransitionsTheme(
            builders: {TargetPlatform.linux: CupertinoPageTransitionsBuilder()},
          ),
        ),
        themeMode: themeMode,
        home: const FileHomePage(),
        routes: {
          AppRoutes.files: (context) => const FileHomePage(),
        },
      ),
    );
  }
}
