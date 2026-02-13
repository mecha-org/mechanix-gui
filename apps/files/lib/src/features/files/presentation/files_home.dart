import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/app_config.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:mechanix_files/src/features/files/presentation/files.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/section_list/mechanix_section_list_theme.dart';
import 'package:widgets/widgets/section_list/section_list_items_type.dart';

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
  void initState() {
    super.initState();

    if (widget.path.isNotEmpty) {
      WidgetsBinding.instance.addPostFrameCallback((_) {
        Navigator.push(
          context,
          MaterialPageRoute(
            builder: (_) => FileExplorerPage(
              startPath: "/${widget.path.map((e) => e.name).join("/")}",
              path: widget.path,
            ),
          ),
        );
      });
    }
  }

  @override
  void dispose() {
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        scrolledUnderElevation: 0,
        elevation: 0,
        backgroundColor: Colors.transparent,
        automaticallyImplyLeading: false,
        title: Text(
          "Files",
          style: TextStyle(
            fontSize: 32,
            color: context.colorScheme.primary,
            fontWeight: FontWeight.w600,
          ),
        ),
      ),
      body: SingleChildScrollView(
        child: Container(
          margin: const EdgeInsets.all(12),
          child: Column(
            children: [
              MechanixSectionList(
                sectionListItems: [
                  SectionListItems.leadingIcon(
                    title: "Home directory",
                    titleTextStyle: listItemTitleTextStyle(context),
                    onTap: () => onTap(context, homeDir, "Home"),
                    iconColor: context.colorScheme.primaryContainer,
                    iconPath: Images.home,
                  ),
                  SectionListItems.leadingIcon(
                    title: "Recents",
                    titleTextStyle: listItemTitleTextStyle(context),
                    onTap: () => onTap(context, recentDir, "Recents"),
                    iconColor: context.colorScheme.primaryContainer,
                    iconPath: Images.recent,
                  ),
                  SectionListItems.leadingIcon(
                    title: "Downloads",
                    titleTextStyle: listItemTitleTextStyle(context),
                    onTap: () => onTap(context, downloadsDir, "Downloads"),
                    iconColor: context.colorScheme.primaryContainer,
                    iconPath: Images.downloads,
                  ),
                  SectionListItems.leadingIcon(
                    title: "Documents",
                    titleTextStyle: listItemTitleTextStyle(context),
                    onTap: () => onTap(context, documentsDir, "Documents"),
                    iconColor: context.colorScheme.primaryContainer,
                    iconPath: Images.homeDocuments,
                  ),
                ],
              ),
              MechanixSectionList(
                title: 'Hard Drive',
                theme: MechanixSectionListThemeData(
                  titleTextStyle: TextStyle(
                    fontSize: 20,
                    color: context.colorScheme.onSecondaryFixed,
                    fontWeight: FontWeight.w500,
                  ),
                ),
                sectionListItems: [
                  SectionListItems.leadingIcon(
                    title: "Root (/)",
                    titleTextStyle: listItemTitleTextStyle(context),
                    onTap: () => onTap(context, "/", "Root"),
                    iconColor: context.colorScheme.primaryContainer,
                    iconPath: Images.hardDrive,
                  ),
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
      fontSize: 20,
      color: context.colorScheme.onSurface,
      fontWeight: FontWeight.w500,
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
