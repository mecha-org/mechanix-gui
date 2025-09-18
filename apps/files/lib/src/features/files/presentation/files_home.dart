import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/app_config.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/blocs/file_state.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:mechanix_files/src/features/files/presentation/file_search.dart';
import 'package:mechanix_files/src/features/files/presentation/files.dart';
import 'package:widgets/mechanix.dart';
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

  ValueNotifier<String> searchQuery = ValueNotifier('');
  OverlayEntry? _searchOverlayEntry;
  bool _isSearching = false;

  @override
  void dispose() {
    searchQuery.dispose();
    _searchOverlayEntry?.remove();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        automaticallyImplyLeading: false,
        title: const Text("Files"),
        actionsPadding: const EdgeInsets.only(right: 0),
        actions: [
          Padding(
            padding: const EdgeInsets.only(right: 8.0),
            child: IconButton(
              icon: Image.asset(Images.search, width: 24, height: 24),
              onPressed: () {
                if (!_isSearching) {
                  _showSearchOverlay(context);
                } else {
                  _clearSearch();
                }
              },
            ),
          ),
        ],
      ),
      body: BlocBuilder<FilesBloc, FilesState>(
        builder: (context, state) {
          void onTap(BuildContext context, String path, String title) {
            final filesBloc = BlocProvider.of<FilesBloc>(context);

            if (path == "/recent") {
              filesBloc.add(LoadRecentFiles());
              Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (_) => BlocProvider.value(
                    value: filesBloc,
                    child: FileExplorerPage(
                      title: 'recent',
                      path: pathToSegments(path),
                    ),
                  ),
                ),
              );
            } else {
              filesBloc.add(LoadFilesAtPath(path));
              Navigator.push(
                context,
                MaterialPageRoute(
                  builder: (_) => BlocProvider.value(
                    value: filesBloc,
                    child: FileExplorerPage(
                      path: pathToSegments(path),
                    ),
                  ),
                ),
              );
            }
          }

          return SingleChildScrollView(
            child: Container(
              margin: EdgeInsets.all(12),
              child: Column(
                children: [
                  MechanixSectionList(
                    sectionListItems: [
                      SectionListItems(
                        title: "Home directory",
                        titleTextStyle: const TextStyle(),
                        onTap: () => onTap(context, homeDir, "Home"),
                        leading: IconWidget(
                          iconWidth: 24,
                          iconHeight: 24,
                          iconPath: Images.home,
                          iconColor: Colors.blueAccent,
                        ),
                      ),
                      SectionListItems(
                        title: "Recents",
                        onTap: () => onTap(context, recentDir, "Recents"),
                        leading: IconWidget(
                          iconWidth: 24,
                          iconHeight: 24,
                          iconPath: Images.recent,
                        ),
                      ),
                      SectionListItems(
                        title: "Downloads",
                        onTap: () => onTap(context, downloadsDir, "Downloads"),
                        leading: IconWidget(
                          iconWidth: 24,
                          iconHeight: 24,
                          iconPath: Images.downloads,
                          iconColor: Colors.deepPurpleAccent,
                        ),
                      ),
                      SectionListItems(
                        title: "Documents",
                        onTap: () => onTap(context, documentsDir, "Documents"),
                        leading: IconWidget(
                          iconWidth: 24,
                          iconHeight: 24,
                          iconPath: Images.homeDocuments,
                          iconColor: Colors.orangeAccent,
                        ),
                      ),
                    ],
                  ),
                  MechanixSectionList(
                    title: 'Hard drive',
                    sectionListItems: [
                      SectionListItems(
                        title: "Root (/)",
                        onTap: () => onTap(context, "/", "Root"),
                        leading: IconWidget(
                          iconWidth: 24,
                          iconHeight: 24,
                          iconPath: Images.hardDrive,
                        ),
                      ),
                    ],
                  ),
                ],
              ),
            ),
          );
        },
      ),
    );
  }

  void _showSearchOverlay(BuildContext context) {
    final overlay = Overlay.of(context);

    _searchOverlayEntry = OverlayEntry(
      builder: (ctx) => Positioned(
        left: 0,
        right: 0,
        bottom: 0,
        child: Material(
          color: Colors.transparent,
          child: Container(
            margin: const EdgeInsets.symmetric(horizontal: 16),
            padding: const EdgeInsets.symmetric(horizontal: 16),
            decoration: BoxDecoration(
              color: Colors.grey[900],
              borderRadius: BorderRadius.circular(50),
            ),
            child: Row(
              children: [
                const Icon(Icons.search, color: Colors.white70, size: 24),
                const SizedBox(width: 12),
                Expanded(
                  child: TextField(
                    autofocus: true,
                    onSubmitted: (value) => _performSearch(context, value),
                    onChanged: (value) => searchQuery.value = value,
                    style: const TextStyle(color: Colors.white),
                    decoration: const InputDecoration(
                      border: InputBorder.none,
                      hintText: 'Type here',
                      hintStyle: TextStyle(color: Colors.white54),
                    ),
                  ),
                ),
                OutlinedButton(
                  style: OutlinedButton.styleFrom(
                          // padding: const EdgeInsets.symmetric(vertical: 0),
                          )
                      .copyWith(
                    splashFactory: NoSplash.splashFactory,
                  ),
                  onPressed: () {
                    _clearSearch();
                  },
                  child: const Icon(Icons.close, color: Colors.white),
                ),
              ],
            ),
          ).padBottom(8),
        ),
      ),
    );

    overlay.insert(_searchOverlayEntry!);
    setState(() => _isSearching = true);
  }

  void _clearSearch() {
    setState(() {
      _isSearching = false;
      searchQuery.value = '';
      _searchOverlayEntry?.remove();
      _searchOverlayEntry = null;
    });
  }

  void _performSearch(BuildContext context, String query) {
    if (query.isEmpty) return;

    final filesBloc = BlocProvider.of<FilesBloc>(context);
    filesBloc.add(SearchFilesInDirectory(homeDir, query));

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (_) => BlocProvider.value(
          value: filesBloc,
          child: SearchResultsPage(query: query),
        ),
      ),
    );

    _clearSearch();
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
