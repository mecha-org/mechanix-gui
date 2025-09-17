import 'package:file/file.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/app_config.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_button.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/blocs/file_state.dart';
import 'package:mechanix_files/src/features/files/models/file_location.dart';
import 'package:mechanix_files/src/features/files/presentation/dialogs.dart';
import 'package:path/path.dart' as p;

class MoveFileDialog extends StatefulWidget {
  final String currentPath;
  final List<String> selectedPaths;
  final FilesBloc filesBloc;

  const MoveFileDialog({
    super.key,
    required this.currentPath,
    required this.selectedPaths,
    required this.filesBloc,
  });

  @override
  State<MoveFileDialog> createState() => _MoveFileDialogState();
}

class _MoveFileDialogState extends State<MoveFileDialog> {
  late String selectedFolder;
  bool isExpanded = false;
  final downloadsDir = AppConfig().downloadsDir;
  final documentsDir = AppConfig().documentsDir;
  final homeDir = AppConfig().homeDir;

  @override
  void initState() {
    super.initState();
    selectedFolder = widget.currentPath;
  }

  @override
  Widget build(BuildContext context) {
    final itemCount = widget.selectedPaths.length;
    final isSingle = itemCount == 1;
    final subText = isSingle
        ? "Move '${widget.selectedPaths.first.split('/').last}' to"
        : "Move $itemCount items to";

    return Dialog(
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
      backgroundColor: Colors.grey[800],
      child: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            _buildTitle(),
            const SizedBox(height: 12),
            Text(subText, style: const TextStyle(color: Colors.white70)),
            const SizedBox(height: 6),
            _buildFolderSelector(),
            if (isExpanded)
              _buildFolderList(widget.filesBloc, selectedFolder, context),
            const SizedBox(height: 20),
            _buildActions(widget.selectedPaths, widget.filesBloc),
          ],
        ),
      ),
    );
  }

  Widget _buildTitle() {
    return Row(
      children: [
        Image.asset(Images.move, color: Colors.amber, height: 28, width: 28),
        const SizedBox(width: 8),
        const Text(
          "Move file",
          style: TextStyle(
            fontWeight: FontWeight.bold,
            fontSize: 18,
            color: Colors.white,
          ),
        ),
      ],
    );
  }

  Widget _buildFolderSelector() {
    return GestureDetector(
      onTap: () => setState(() => isExpanded = !isExpanded),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 10),
        decoration: BoxDecoration(
          color: Colors.grey[700],
          borderRadius: BorderRadius.circular(6),
        ),
        child: Row(
          children: [
            Expanded(
              child: Text(
                selectedFolder == homeDir ? "Files" : selectedFolder,
                style: const TextStyle(color: Colors.white),
                overflow: TextOverflow.ellipsis,
              ),
            ),
            Icon(
              isExpanded ? Icons.keyboard_arrow_up : Icons.unfold_more,
              color: Colors.white,
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildFolderList(
      FilesBloc filesBloc, String path, BuildContext buildContext) {
    if (path == homeDir) {
      final locations = [
        FileLocation(icon: Images.home, label: "Home", path: homeDir),
        FileLocation(
            icon: Images.downloads, label: "Downloads", path: downloadsDir),
        FileLocation(
            icon: Images.homeDocuments, label: "Documents", path: documentsDir),
        FileLocation(icon: Images.hardDrive, label: "Root(/)", path: "/"),
      ];

      return Container(
        margin: const EdgeInsets.only(top: 2),
        height: 200,
        decoration: BoxDecoration(
          color: Colors.grey[700],
          borderRadius: BorderRadius.circular(6),
        ),
        child: ListView(
          padding: const EdgeInsets.all(8),
          children: [
            ...locations.map(
                (item) => buildLocationTile(filesBloc, item, buildContext)),
          ],
        ),
      );
      //   ],
      // );
    } else {
      // Trigger the load when building (could also do this in initState)
      filesBloc.add(LoadFilesAtPath(path));

      return Container(
        margin: const EdgeInsets.only(top: 4),
        height: 200,
        decoration: BoxDecoration(
          color: Colors.grey[700],
          borderRadius: BorderRadius.circular(6),
        ),
        child: Column(
          children: [
            ListTile(
              title: Text(
                buildBreadcrumb(selectedFolder).join(" > "),
                style: const TextStyle(color: Colors.white),
              ),
              onTap: () {
                setState(() {
                  selectedFolder = goOneLevelBack(selectedFolder);
                });
              },
            ),

            ListTile(
              leading: const Icon(Icons.create_new_folder, color: Colors.blue),
              title: const Text(
                'New folder',
                style: TextStyle(color: Colors.white),
              ),
              onTap: () => showCreateFolderDialog(
                context: context,
                currentPath: widget.currentPath,
                filesBloc: widget.filesBloc, // pass the bloc directly
              ),
            ),
            const Divider(color: Colors.white24, height: 1),

            // Listen to bloc state for folder list
            Expanded(
              child: BlocBuilder<FilesBloc, FilesState>(
                bloc: filesBloc,
                builder: (context, state) {
                  final folders = state.fileSystemList
                      .where((entity) => entity is Directory)
                      .map((entity) => p.basename(entity.path))
                      .toList();

                  return ListView.builder(
                    itemCount: folders.length,
                    itemBuilder: (context, index) {
                      final folder = folders[index];
                      return ListTile(
                        leading: const Icon(Icons.folder, color: Colors.blue),
                        title: Text(
                          folder,
                          style: const TextStyle(color: Colors.white),
                        ),
                        onTap: () {
                          setState(() {
                            selectedFolder = folder;
                            isExpanded = false;
                          });
                        },
                      );
                    },
                  );
                },
              ),
            ),
          ],
        ),
      );
    }
  }

  Widget _buildActions(List<String> selectedPaths, FilesBloc filesBloc) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.end,
      children: [
        CustomButton(
          label: "Cancel",
          backgroundColor: Colors.grey[700]!,
          textColor: Colors.white,
          onPressed: () => Navigator.pop(context),
        ),
        const SizedBox(width: 12),
        CustomButton(
          label: "Save",
          backgroundColor: Colors.blue,
          textColor: Colors.white,
          onPressed: () {
            handleMove(
                filesBloc, selectedPaths, selectedFolder, widget.currentPath);
            Navigator.pop(context);
          },
        ),
      ],
    );
  }

  Widget buildLocationTile(
      FilesBloc filesBloc, FileLocation location, BuildContext context) {
    return Container(
      margin: const EdgeInsets.symmetric(vertical: 1),
      child: Builder(
        builder: (ctx) => ListTile(
          leading: Image.asset(location.icon, color: Colors.blueAccent),
          title:
              Text(location.label, style: const TextStyle(color: Colors.white)),
          trailing: const Icon(Icons.chevron_right, color: Colors.white70),
          onTap: () {
            final state = ctx.findAncestorStateOfType<_MoveFileDialogState>();
            state?.setState(() {
              state.selectedFolder = location.path;
            });
          },
        ),
      ),
    );
  }
}

String goOneLevelBack(String path) {
  if (path == "/") return "/";

  String normalized = path;
  if (normalized.endsWith("/") && normalized.length > 1) {
    normalized = normalized.substring(0, normalized.length - 1);
  }

  final parts = normalized.split("/")..removeWhere((e) => e.isEmpty);

  if (parts.isEmpty) return "/";

  parts.removeLast();
  return parts.isEmpty ? "/" : "/" + parts.join("/");
}

List<String> buildBreadcrumb(String currentPath) {
  // Normalize
  String path = currentPath;
  if (path.endsWith("/") && path.length > 1) {
    path = path.substring(0, path.length - 1);
  }

  final parts = path.split("/")..removeWhere((e) => e.isEmpty);

  if (path == "/") {
    return ["Files", "root"];
  }

  if (parts.length >= 2) {
    return [parts[parts.length - 2], parts.last];
  }

  return ["root", parts.last];
}

void handleMove(FilesBloc filesBloc, List<String> movedPaths,
    String selectedFolder, String currentPath) {
  final targetPath = '$currentPath/$selectedFolder';
  filesBloc.add(Move(
    sourcePaths: movedPaths,
    destinationPath: targetPath,
  ));
  filesBloc.add(CancelMoveMode());
}
