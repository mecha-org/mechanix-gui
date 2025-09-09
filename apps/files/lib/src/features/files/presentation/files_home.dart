import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/app_config.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/blocs/file_state.dart';
import 'package:mechanix_files/src/features/files/models/file_location.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:mechanix_files/src/features/files/presentation/files.dart';

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
  final recentFilesCount = AppConfig().recentFilesCount;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.black,
      appBar: AppBar(
        automaticallyImplyLeading: false,
        title: const Text("Files"),
        backgroundColor: Colors.black,
        actions: [
          Padding(
              padding: const EdgeInsets.only(right: 8.0),
              child: IconButton(
                icon: Image.asset(Images.search, width: 24, height: 24),
                onPressed: () {}, // TODO: Implement search functionality
              )),
        ],
      ),
      body: BlocBuilder<FilesBloc, FilesState>(
        builder: (context, state) {
          // Default UI (Locations list)
          final locations = [
            FileLocation(icon: Images.home, label: "Home", path: homeDir),
            FileLocation(
                icon: Images.recent, label: "Recents", path: "/recent"),
            FileLocation(
                icon: Images.downloads, label: "Downloads", path: downloadsDir),
            FileLocation(
                icon: Images.homeDocuments,
                label: "Documents",
                path: documentsDir),
          ];

          //TODO: Refactor this to use a more dynamic approach to get drives
          final drives = [
            FileLocation(icon: Images.hardDrive, label: "Root (/)", path: "/"),
            // FileLocation(
            //     icon: Images.hardDrive, label: "/nvme/part 1", path: "/nvme"),
          ];

          return ListView(
            padding: const EdgeInsets.all(12),
            children: [
              ...locations.map((item) => _buildLocationTile(context, item)),
              const SizedBox(height: 20),
              const Text(
                "Hard-drive",
                style: TextStyle(color: Colors.white70, fontSize: 14),
              ),
              const SizedBox(height: 8),
              ...drives.map((item) => _buildLocationTile(context, item)),
            ],
          );
        },
      ),
    );
  }

  Widget _buildLocationTile(BuildContext context, FileLocation location) {
    return Container(
      margin: const EdgeInsets.symmetric(vertical: 6),
      decoration: BoxDecoration(
        color: const Color(0xFF1E1E1E),
        borderRadius: BorderRadius.circular(8),
      ),
      child: ListTile(
        leading: Image.asset(location.icon, color: Colors.blueAccent),
        title:
            Text(location.label, style: const TextStyle(color: Colors.white)),
        trailing: const Icon(Icons.chevron_right, color: Colors.white70),
        onTap: () async {
          final filesBloc = BlocProvider.of<FilesBloc>(context);

          if (location.path == "/recent") {
            filesBloc.add(LoadRecentFiles());
            Navigator.push(
              context,
              MaterialPageRoute(
                builder: (_) => BlocProvider.value(
                  value: filesBloc,
                  child: FileExplorerPage(
                    title: 'recent',
                    path: pathToSegments(location.path),
                  ),
                ),
              ),
            );
          } else {
            filesBloc.add(LoadFilesAtPath(location.path));
            Navigator.push(
              context,
              MaterialPageRoute(
                builder: (_) => BlocProvider.value(
                  value: filesBloc,
                  child: FileExplorerPage(
                    path: pathToSegments(location.path),
                  ),
                ),
              ),
            );
          }
        },
      ),
    );
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
