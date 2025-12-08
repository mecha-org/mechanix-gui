import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/app_config.dart';
import 'package:mechanix_files/app_route.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/commons/styles/file_theme_extenstions.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:mechanix_files/src/features/files/presentation/files.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/sectionList/mechanix_section_list_theme.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';

class FileHomePage extends StatefulWidget {
  final String title;
  final List<FileItem> path;

  const FileHomePage({
    super.key,
    this.title = "Files",
    this.path = const [],
  });

  @override
  FileHomePageState createState() => FileHomePageState();
}

class FileHomePageState extends State<FileHomePage> {
  final downloadsDir = AppConfig().downloadsDir;
  final documentsDir = AppConfig().documentsDir;
  final homeDir = AppConfig().homeDir;
  final recentDir = AppConfig().recentDir;

  @override
  void dispose() {
    super.dispose();
  }

  void onSearch() {
    Navigator.pushNamed(context, AppRoutes.searchFiles);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        automaticallyImplyLeading: false,
        title: Text(
          "Files",
          style: TextStyle(
            fontSize: 24,
            color: const Color(0xFFD2D2D2),
            fontWeight: FontWeight.w600,
            fontFamily:
                Theme.of(context).extension<FilesTheme>()!.defaultFontFamily,
          ),
        ),
        // actionsPadding: const EdgeInsets.only(right: 0),
        // actions: [
        //   Padding(
        //     padding: const EdgeInsets.only(right: 8.0),
        //     child: IconButton(
        //       icon: Image.asset(Images.search, width: 24, height: 24),
        //       onPressed: () {
        //         onSearch();
        //       },
        //     ),
        //   ),
        // ],
      ),
      body: SingleChildScrollView(
        child: Container(
          margin: const EdgeInsets.all(12),
          child: Column(
            children: [
              MechanixSectionList(
                sectionListItems: [
                  SectionListItems(
                      title: "Home directory",
                      titleTextStyle: listItemTitleTextStyle(context),
                      onTap: () => onTap(context, homeDir, "Home"),
                      leading: Image.asset(Images.home, height: 20, width: 20)),
                  SectionListItems(
                      title: "Recents",
                      titleTextStyle: listItemTitleTextStyle(context),
                      onTap: () => onTap(context, recentDir, "Recents"),
                      leading:
                          Image.asset(Images.recent, height: 24, width: 24)),
                  SectionListItems(
                      title: "Downloads",
                      titleTextStyle: listItemTitleTextStyle(context),
                      onTap: () => onTap(context, downloadsDir, "Downloads"),
                      leading:
                          Image.asset(Images.downloads, height: 24, width: 24)),
                  SectionListItems(
                      title: "Documents",
                      titleTextStyle: listItemTitleTextStyle(context),
                      onTap: () => onTap(context, documentsDir, "Documents"),
                      leading: Image.asset(Images.homeDocuments,
                          height: 24, width: 24)),
                ],
              ),
              MechanixSectionList(
                title: 'Hard Drive',
                theme: MechanixSectionListThemeData(
                  titleTextStyle: TextStyle(
                    fontSize: 18,
                    color: const Color(0xFF8F8F8F),
                    fontWeight: FontWeight.w500,
                    fontFamily: Theme.of(context)
                        .extension<FilesTheme>()!
                        .defaultFontFamily,
                  ),
                ),
                sectionListItems: [
                  SectionListItems(
                      title: "Root (/)",
                      titleTextStyle: listItemTitleTextStyle(context),
                      onTap: () => onTap(context, "/", "Root"),
                      leading:
                          Image.asset(Images.hardDrive, height: 24, width: 24)),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }

  TextStyle listItemTitleTextStyle(BuildContext context) {
    return TextStyle(
      fontSize: 18,
      color: const Color(0xFFD2D2D2),
      fontWeight: FontWeight.w500,
      fontFamily: Theme.of(context).extension<FilesTheme>()!.defaultFontFamily,
    );
  }

  void onTap(BuildContext context, String path, String title) {
    if (path == recentDir) {
      final filesBloc = BlocProvider.of<FilesBloc>(context);
      filesBloc.add(LoadRecentFiles());

      Navigator.push(
        context,
        MaterialPageRoute(
          builder: (_) => BlocProvider.value(
            value: filesBloc,
            child: FileExplorerPage(
              title: 'Recent',
              path: pathToSegments(path),
            ),
          ),
        ),
      );
    } else {
      Navigator.push(
        context,
        MaterialPageRoute(
          builder: (_) => FileExplorerPage(
            startPath: path,
          ),
          //   ),
        ),
      );
    }
  }
}

List<FileItem> pathToSegments(String fullPath) {
  // Remove leading/trailing slashes, then split
  final segments =
      fullPath.split('/').where((segment) => segment.isNotEmpty).toList();

  return segments.map((name) {
    return FileItem(name: name, type: 'dir', children: null);
  }).toList();
}
