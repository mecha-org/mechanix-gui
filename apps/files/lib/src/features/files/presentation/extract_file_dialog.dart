import 'dart:async';
import 'dart:io' as io;

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/app_config.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/controllers/file_manager_controller.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/blocs/file_state.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:mechanix_files/src/features/files/presentation/list_view.dart';
import 'package:path/path.dart' as p;
import 'package:widgets/extension.dart';
import 'package:widgets/widgets/icon_widget.dart';
import 'package:widgets/widgets/navigation_bar/mechanix_navigation_bar.dart';
import 'package:widgets/widgets/searchbar/mechanix_search_bar.dart';
import 'package:widgets/widgets/sectionList/mechanix_section_list.dart';
import 'package:widgets/widgets/sectionList/mechanix_section_list_theme.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';

class ExtractBottomSheet extends StatefulWidget {
  final String path;
  final String title;
  final FilesBloc filesBloc;
  final FilesBloc filesBlocMainContext;
  final VoidCallback onExtractCompleted;

  const ExtractBottomSheet({
    super.key,
    required this.path,
    required this.title,
    required this.filesBloc,
    required this.filesBlocMainContext,
    required this.onExtractCompleted,
  });

  @override
  State<ExtractBottomSheet> createState() => _ExtractBottomSheetState();
}

class _ExtractBottomSheetState extends State<ExtractBottomSheet> {
  final FileManagerController controller = FileManagerController();
  late String currentPath;
  bool isSearching = false;
  final TextEditingController searchController = TextEditingController();
  final FocusNode searchFocusNode = FocusNode();

  final downloadsDir = AppConfig().downloadsDir;
  final documentsDir = AppConfig().documentsDir;
  final homeDir = AppConfig().homeDir;
  final recentDir = AppConfig().recentDir;

  Future<void> _loadFiles() async {
    await controller.openDirectory(io.Directory(widget.path));
  }

  @override
  void initState() {
    super.initState();
    currentPath = widget.path;
    _loadFiles();

    controller.getPathNotifier.addListener(() {
      setState(() {
        currentPath = controller.getPathNotifier.value;
      });
    });
  }

  @override
  Widget build(BuildContext context) {
    final isAtRoot = currentPath == '/' || currentPath.isEmpty;

    final isDocumentsDir = currentPath == documentsDir;
    final isDownloadsDir = currentPath == downloadsDir;
    final isHomeDir = currentPath == homeDir;
    final isRecentDir = currentPath == recentDir;
    final isHomePageDir = isHomeDir ||
        isDownloadsDir ||
        isDocumentsDir ||
        isAtRoot ||
        isRecentDir;

    return ValueListenableBuilder<List<io.FileSystemEntity>>(
      valueListenable: controller.paginatedEntities,
      builder: (context, entities, _) {
        final foldersList = entities
            .where((file) =>
                file is io.Directory &&
                !p
                    .basename(file.path)
                    .startsWith('.')) // exclude hidden folders
            .toList();

        if (foldersList.length < pageSize) {
          controller.loadNextChunk();
        }

        final itemCount = foldersList.length;

        // Base height logic
        double minChildSize;
        double initialChildSize;
        double maxChildSize = 0.7;

        if (itemCount <= 2) {
          minChildSize = 0.35;
          initialChildSize = 0.4;
        } else if (itemCount <= 5) {
          minChildSize = 0.4;
          initialChildSize = 0.5;
        } else if (itemCount <= 10) {
          minChildSize = 0.5;
          initialChildSize = 0.6;
        } else {
          minChildSize = 0.6;
          initialChildSize = 0.7;
        }

        return DraggableScrollableSheet(
          expand: false,
          initialChildSize: initialChildSize,
          minChildSize: minChildSize,
          maxChildSize: maxChildSize,
          builder: (context, sheetController) {
            // Scroll controller listener
            sheetController.addListener(() {
              final maxScroll = sheetController.position.maxScrollExtent;
              final currentScroll = sheetController.position.pixels;

              if (currentScroll >= 0.8 * maxScroll) {
                controller.loadNextChunk();
              }
            });

            return Container(
              decoration: BoxDecoration(
                color: Colors.grey[850],
                borderRadius:
                    const BorderRadius.vertical(top: Radius.circular(16)),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  MechanixNavigationBar(
                    backgroundColor: Colors.grey[850],
                    leadingWidget: IconButton(
                      icon: const Icon(
                        Icons.arrow_back_ios,
                        size: 20,
                        color: Colors.blue,
                      ),
                      onPressed: () {
                        if (isHomePageDir) {
                          Navigator.pop(context);
                          extractMainBottomSheet(
                              widget.onExtractCompleted, controller);
                        } else {
                          controller.goToParentDirectory();
                        }
                      },
                    ),
                    title:
                        isAtRoot ? "Root" : getCurrentFolderName(currentPath),
                    titleStyle: const TextStyle(
                      color: Colors.white,
                      fontSize: 18,
                    ),
                    actionWidgets: [
                      if (!isSearching)
                        IconButton(
                          icon: const Icon(Icons.search, color: Colors.white),
                          onPressed: () {
                            setState(() {
                              isSearching = true;
                              searchFocusNode.requestFocus();
                            });
                          },
                        ),
                    ],
                  ).padTop(8),
                  Expanded(
                    child: buildListViewExtract(
                      foldersList,
                      context,
                      currentPath,
                      widget.filesBloc,
                      widget.onExtractCompleted,
                      sheetController,
                    ),
                  ),
                  if (isSearching)
                    Padding(
                      padding: const EdgeInsets.symmetric(
                          horizontal: 12, vertical: 12),
                      child: SizedBox(
                        height: 48,
                        child: MechanixSearchBar(
                          controller: searchController,
                          autoFocus: true,
                          hintText: "Type here",
                          onChanged: (query) {
                            controller.search(query);
                          },
                          onCloseIconPress: () {
                            setState(() {
                              searchController.clear();
                              isSearching = false;
                            });

                            // Reload directory content when clearing search
                            controller.reload();
                          },
                        ),
                      ),
                    )
                  else
                    Container(
                      padding: const EdgeInsets.symmetric(
                          horizontal: 12, vertical: 12),
                      decoration: BoxDecoration(
                        color: Colors.grey[850],
                        borderRadius: const BorderRadius.vertical(
                            bottom: Radius.circular(16)),
                      ),
                      child: Row(
                        children: [
                          Expanded(
                            child: OutlinedButton(
                              style: OutlinedButton.styleFrom(
                                side: const BorderSide(
                                    color: Colors.white70, width: 0.4),
                                backgroundColor: Colors.transparent,
                              ),
                              onPressed: () {
                                Navigator.pop(context);
                              },
                              child: const Icon(Icons.close,
                                  color: Colors.white70),
                            ),
                          ),
                          const SizedBox(width: 12),
                          Expanded(
                            child: ElevatedButton(
                              style: ElevatedButton.styleFrom(
                                backgroundColor: Colors.grey[800],
                              ),
                              onPressed: () {
                                handleExtract(context, widget.filesBloc.state);

                                Navigator.pop(context, true);
                              },
                              child: const Icon(Icons.check,
                                  color: Colors.white70),
                            ),
                          ),
                        ],
                      ),
                    ),
                ],
              ),
            );
          },
        );
      },
    );
  }

  void extractMainBottomSheet(
      onExtractCompleted, FileManagerController controller) {
    final filesBloc = BlocProvider.of<FilesBloc>(context); // get bloc

    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.grey[850],
      isScrollControlled: true,
      builder: (context) {
        return SingleChildScrollView(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const Text(
                'Extract to',
                style: TextStyle(
                  fontSize: 16,
                  color: Colors.grey,
                ),
              ).padLeft(18).padBottom(4).padTop(14),
              MechanixSectionListTheme(
                style: MechanixSectionListThemeData(
                  height: 42,
                  dividerPadding: EdgeInsets.zero,
                  widgetPadding: EdgeInsets.zero,
                  backgroundColor: WidgetStateProperty.all(Colors.grey[850]),
                ),
                child: MechanixSectionList(
                  sectionListItems: [
                    SectionListItems(
                        title: "Home directory",
                        titleTextStyle: const TextStyle(fontSize: 14),
                        onTap: () => onItemTap(context, homeDir, "Home",
                            filesBloc, onExtractCompleted),
                        leading: const IconWidget(
                          iconWidth: 20,
                          iconHeight: 20,
                          iconPath: Images.home,
                          iconColor: Colors.blueAccent,
                        ),
                        defaultTrailingIcon: false,
                        trailing: SizedBox(
                          child: const Icon(
                            size: 16,
                            Icons.arrow_forward_ios,
                            color: Colors.grey,
                          ).padAll(4),
                        )),
                    SectionListItems(
                        title: "Downloads",
                        titleTextStyle: const TextStyle(fontSize: 14),
                        onTap: () => onItemTap(context, downloadsDir,
                            "Downloads", filesBloc, onExtractCompleted),
                        leading: const IconWidget(
                          iconWidth: 20,
                          iconHeight: 20,
                          iconPath: Images.downloads,
                          iconColor: Colors.deepPurpleAccent,
                        ),
                        defaultTrailingIcon: false,
                        trailing: SizedBox(
                          child: const Icon(
                            size: 16,
                            Icons.arrow_forward_ios,
                            color: Colors.grey,
                          ).padAll(4),
                        )),
                    SectionListItems(
                        title: "Documents",
                        titleTextStyle: const TextStyle(fontSize: 14),
                        onTap: () => onItemTap(context, documentsDir,
                            "Documents", filesBloc, onExtractCompleted),
                        leading: const IconWidget(
                          iconWidth: 20,
                          iconHeight: 20,
                          iconPath: Images.homeDocuments,
                          iconColor: Colors.orangeAccent,
                        ),
                        defaultTrailingIcon: false,
                        trailing: SizedBox(
                          child: const Icon(
                            size: 16,
                            Icons.arrow_forward_ios,
                            color: Colors.grey,
                          ).padAll(4),
                        )),
                    SectionListItems(
                        title: "Root (/)",
                        titleTextStyle: const TextStyle(fontSize: 14),
                        onTap: () => onItemTap(context, "/", "Root", filesBloc,
                            onExtractCompleted),
                        leading: const IconWidget(
                          iconWidth: 20,
                          iconHeight: 20,
                          iconPath: Images.hardDrive,
                        ),
                        defaultTrailingIcon: false,
                        trailing: SizedBox(
                          child: const Icon(
                            size: 16,
                            Icons.arrow_forward_ios,
                            color: Colors.grey,
                          ).padAll(4),
                        )),
                  ],
                ),
              ),
            ],
          ),
        );
      },
    );
  }

  Future<void> handleExtract(BuildContext context, FilesState state) async {
    String targetPath = currentPath;
    final bloc = BlocProvider.of<FilesBloc>(context);
    final completer = Completer<void>();

    // Validate zip file
    if (!isZipFileValid(state.zipFilePath)) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text(
            "ZIP file is corrupted or invalid",
            style: TextStyle(color: Colors.red),
          ),
          backgroundColor: Colors.white,
        ),
      );
      return;
    }

    // Create unique destination folder if one already exists
    final zipName = p.basenameWithoutExtension(state.zipFilePath);
    final baseExtractPath = p.join(targetPath, zipName);
    final uniqueExtractPath = await getUniqueExtractPath(baseExtractPath);

    bloc.add(ExtractZipTo(
      state.zipFilePath,
      uniqueExtractPath,
      completer,
    ));

    await completer.future;
    bloc.add(CancelExtractMode());

    // widget.onExtractCompleted();

    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: const Text("Finished extracting",
            style: TextStyle(color: Colors.white)),
        duration: const Duration(seconds: 2),
        backgroundColor: Colors.grey[800],
      ),
    );
  }
}

void onItemTap(
  BuildContext context,
  String path,
  String title,
  FilesBloc filesBlocMain,
  VoidCallback onExtractCompleted,
) {
  final localBloc = FilesBloc(
    fileRepository: filesBlocMain.fileRepository,
    recentFilesManager: filesBlocMain.recentFilesManager,
  );

// Clone the state
  localBloc.emit(
    localBloc.state.copyWith(
      zipFilePath: filesBlocMain.state.zipFilePath,
    ),
  );
  // Close any existing bottom sheet
  Navigator.pop(context, true);

  showModalBottomSheet(
    context: context,
    isScrollControlled: true,
    backgroundColor: Colors.transparent,
    shape: const RoundedRectangleBorder(
      borderRadius: BorderRadius.vertical(top: Radius.circular(16)),
    ),
    builder: (_) => BlocProvider.value(
      value: localBloc,
      child: ExtractBottomSheet(
        path: path,
        title: title,
        filesBloc: localBloc,
        filesBlocMainContext: filesBlocMain,
        onExtractCompleted: onExtractCompleted,
      ),
    ),
  );
}
