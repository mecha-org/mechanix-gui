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
import 'package:mechanix_files/src/features/files/presentation/files_home.dart';
import 'package:media_kit/media_kit.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  final configResult = await connectToMxconf(); // Await the Future properly
  AppConfig().loadFromMap(configResult); // Load into singleton instance
  MediaKit.ensureInitialized();

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
      child: const MainApp(),
    ),
  );
}

class MainApp extends StatelessWidget {
  const MainApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      theme: ThemeData.dark(),
      home: buildFileExplorerPage(context),
      routes: {
        AppRoutes.files: (context) => buildFileExplorerPage(context),
      },
    );
  }

  Widget buildFileExplorerPage(BuildContext context) {
    return BlocProvider(
      create: (_) => FilesBloc(
          fileRepository: context.read<FileRepository>(),
          recentFilesManager: context.read<RecentFilesManager>())
        ..add(InitializeFiles()),
      child: const FileHomePage(),
    );
  }
}
