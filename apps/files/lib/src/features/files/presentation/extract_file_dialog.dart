import 'dart:async';
import 'dart:io' as io;
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/app_config.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/commons/styles/file_theme_extenstions.dart';
import 'package:mechanix_files/src/controllers/file_manager_controller.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/blocs/file_state.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:mechanix_files/src/features/files/presentation/list_view.dart';
import 'package:path/path.dart' as p;
import 'package:widgets/extension.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottom_bar/mechanix_bottom_bar.dart';
import 'package:widgets/widgets/filled_button/mechanix_filled_button.dart';
import 'package:widgets/widgets/icon_widget.dart';
import 'package:widgets/widgets/section_list/mechanix_section_list.dart';
import 'package:widgets/widgets/section_list/mechanix_section_list_theme.dart';
import 'package:widgets/widgets/section_list/section_list_items_type.dart';
import 'package:widgets/widgets/text_input/mechanix_text_input.dart';

var totalExtractedCount = 0;

class ExtractBottomSheetContent extends StatefulWidget {
  final FilesBloc filesBloc;
  final int selectedCount;
  final VoidCallback reload;
  final String currentPath;
  final BuildContext rootContext;

  const ExtractBottomSheetContent({
    required this.filesBloc,
    required this.selectedCount,
    required this.reload,
    required this.currentPath,
    required this.rootContext,
    super.key,
  });

  @override
  State<ExtractBottomSheetContent> createState() =>
      ExtractBottomSheetContentState();
}

class ExtractBottomSheetContentState extends State<ExtractBottomSheetContent> {
  final FileManagerController controller = FileManagerController();
  final ScrollController _scrollController = ScrollController();
  String currentPath = "";
  final ValueNotifier<String> searchQuery = ValueNotifier("");
  final downloadsDir = AppConfig().downloadsDir;
  final documentsDir = AppConfig().documentsDir;
  final homeDir = AppConfig().homeDir;

  bool showHomeView = false;
  bool isSearching = false;
  bool isCreateFolder = false;

  bool showRenameBar = false;
  String renameText = "";
  String createdFolderPath = "";
  String originalFolderName = "";

  @override
  void initState() {
    super.initState();

    currentPath = widget.currentPath;

    _scrollController.addListener(_onScroll);

    controller.openDirectory(io.Directory(widget.currentPath));

    controller.getPathNotifier.addListener(() {
      setState(() {
        currentPath = controller.getPathNotifier.value;
      });
    });

    searchQuery.addListener(() {
      if (searchQuery.value.isEmpty) {
        controller.reload();
      }
    });
  }

  void _onScroll() {
    if (!_scrollController.hasClients) return;
    final position = _scrollController.position;
    if (!position.hasPixels || !position.hasContentDimensions) return;

    final maxScroll = position.maxScrollExtent;
    final currentScroll = position.pixels;

    // Trigger near bottom
    if (currentScroll >= 0.8 * maxScroll) {
      controller.loadNextChunk();
    }
  }

  @override
  void dispose() {
    searchQuery.dispose();
    _scrollController.removeListener(_onScroll);
    _scrollController.dispose();
    super.dispose();
  }

  void handleBack() {
    controller.goToParentDirectory();
  }

  void homeNavigation() {
    setState(() {
      showHomeView = true;
    });
  }

  TextStyle listItemTitleTextStyle(BuildContext context) {
    return TextStyle(
      fontSize: 18,
      color: const Color(0xFFD2D2D2),
      fontWeight: FontWeight.w500,
      fontFamily: Theme.of(context).extension<FilesTheme>()!.defaultFontFamily,
    );
  }

  @override
  Widget build(BuildContext context) {
    final isAtRoot = currentPath == '/' || currentPath.isEmpty;

    final isDocumentsDir = currentPath == documentsDir;
    final isDownloadsDir = currentPath == downloadsDir;
    final isHomeDir = currentPath == homeDir;
    final isHomePageDir =
        isHomeDir || isDownloadsDir || isDocumentsDir || isAtRoot;

    final selectedPaths = widget.filesBloc.state.zipFilePaths;

    final selectedCount = selectedPaths.length;
    final label = selectedCount > 1
        ? " $selectedCount items"
        : " '${selectedPaths.first.split('/').last}'";

    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const SizedBox(height: 18),

        // Title
        if (!showHomeView)
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
            child: Text(
              currentPath == '/' || currentPath.isEmpty
                  ? "Root"
                  : getCurrentFolderName(currentPath),
              style: TextStyle(
                color: const Color(0xFFD2D2D2),
                fontSize: 24,
                fontWeight: FontWeight.w600,
                fontFamily: Theme.of(context)
                    .extension<FilesTheme>()!
                    .defaultFontFamily,
              ),
            ),
          ),

        // Folder list
        Expanded(
          child: showHomeView
              ? buildHomeView(context)
              : buildListViewMoveAndExtract(
                  context, _scrollController, controller),
        ),

        const SizedBox(height: 18),
        const Divider(),
        // Navigation bar
        SizedBox(
          height: 60,
          child: Row(
            children: [
              if (isSearching) ...[
                Expanded(
                  child: MechanixTextInput.search(
                    cursorColor:
                        Theme.of(context).extension<FilesTheme>()!.primaryColor,
                    prefixIcon: const IconWidget(
                      iconPath: Images.search,
                      iconColor: Color(0xFFD2D2D2),
                      iconHeight: 24,
                      iconWidth: 24,
                    ),
                    hintText: "Search here",
                    onChanged: (query) {
                      searchQuery.value = query;

                      if (query.trim().length > 2) {
                        controller.search(query.trim());
                      }
                    },
                    onClear: () {
                      setState(() {
                        isSearching = false;
                        searchQuery.value = "";
                      });
                      controller.search('');
                    },
                  ),
                ),
              ] else if (showRenameBar) ...[
                _buildRenameDialog()
              ] else ...[
                // Entire MechanixBottomBar must be inside Row children
                Expanded(
                  child: MechanixBottomBar(
                    leadingWidget: [
                      BottomBarButton(
                        iconPath: Images.back,
                        onPressed: () {
                          (isHomePageDir ? homeNavigation() : handleBack());
                        },
                      ),
                    ],
                    anchorWidget: [
                      BottomBarButton(
                        iconWidget: const IconWidget(
                          iconPath: Images.search,
                          iconColor: Color(0xFFD2D2D2),
                          iconHeight: 28,
                          iconWidth: 28,
                        ),
                        onPressed: () {
                          setState(() => isSearching = true);
                        },
                      ),
                      BottomBarButton(
                        iconWidget: const IconWidget(
                          iconPath: Images.home,
                          iconColor: Color(0xFFD2D2D2),
                          iconHeight: 24,
                          iconWidth: 24,
                        ),
                        onPressed: () {
                          setState(() => showHomeView = true);
                        },
                      ),
                      BottomBarButton(
                        iconWidget: const IconWidget(
                          iconPath: Images.createFolder,
                          iconColor: Color(0xFFD2D2D2),
                          iconHeight: 28,
                          iconWidth: 28,
                        ),
                        onPressed: () async {
                          await createFolderAndRename();
                        },
                      ),
                    ],
                  ).padLeft(16).padRight(16),
                ),
              ]
            ],
          ),
        ),

        // Bottom bar
        Container(
          height: 60,
          padding: const EdgeInsets.symmetric(horizontal: 16),
          decoration: const BoxDecoration(
            color: Color(0xFF222222),
            borderRadius: BorderRadius.vertical(bottom: Radius.circular(12)),
          ),
          child: Row(
            children: [
              Expanded(
                child: RichText(
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  text: TextSpan(
                    children: [
                      TextSpan(
                        text: "Extracting ",
                        style: regularStyle(context),
                      ),
                      TextSpan(
                        text: label,
                        style: boldStyle(context),
                      ),
                    ],
                  ),
                ),
              ),
              MechanixFilledButton(
                theme: buttonThemeData(context,
                    type: MechanixButtonType.cancel, size: const Size(94, 40)),
                label: "Cancel",
                onPressed: () {
                  widget.filesBloc.add(CancelExtractMode());
                  Navigator.pop(context);
                },
              ),
              const SizedBox(width: 10),
              MechanixFilledButton(
                theme: buttonThemeData(context,
                    type: showHomeView
                        ? MechanixButtonType.disable
                        : MechanixButtonType.action,
                    size: const Size(94, 40)),
                label: "Extract",
                onPressed: showHomeView
                    ? null
                    : () {
                        handleExtract(context, widget.filesBloc.state);
                        Navigator.pop(context, true);
                      },
              )
            ],
          ),
        ),
      ],
    );
  }

  Future<void> createFolderAndRename() async {
    final path = controller.getCurrentPath;
    final bloc = context.read<FilesBloc>();

    // Generate safe name
    final folderName = await generateUniqueFolderName(path);
    final newPath = p.join(path, folderName);

    bloc.add(CreateFolder(
      path: path,
      folderName: folderName,
      controller: controller,
    ));

    // Wait for folder to appear in UI (optional small delay)
    await Future.delayed(const Duration(milliseconds: 200));

    // Store rename target + default rename text
    setState(() {
      createdFolderPath = newPath;
      renameText = folderName;
      originalFolderName = folderName;
      showRenameBar = true;
      isCreateFolder = false;
    });
  }

  Widget _buildRenameDialog() {
    final bool isEmpty = renameText.trim().isEmpty;
    final bool isSame = renameText.trim() == originalFolderName.trim();
    final bool showCheck = !isEmpty && !isSame; // valid new name

    return Expanded(
      child: MechanixTextInput.textInput(
        cursorColor: Theme.of(context).extension<FilesTheme>()!.primaryColor,
        autofocus: true,
        initialValue: renameText,
        onChanged: (v) => setState(() => renameText = v),
        anchorWidget: showCheck
            ? IconButton(
                icon: const Icon(Icons.check, color: Colors.white),
                onPressed: () {
                  final filesBloc = context.read<FilesBloc>();
                  filesBloc.add(
                    Rename(
                      oldPath: createdFolderPath,
                      newName: renameText,
                      controller: controller,
                    ),
                  );
                  setState(() => showRenameBar = false);
                  controller.clearNewFolder();
                },
              )
            : IconButton(
                icon: const Icon(Icons.close, color: Colors.white),
                onPressed: () {
                  setState(() => showRenameBar = false);
                  controller.clearNewFolder();
                },
              ),
      ),
    );
  }

  Widget buildHomeView(BuildContext context) {
    return SingleChildScrollView(
      child: Container(
        margin: const EdgeInsets.all(12),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Title
            Padding(
              padding: const EdgeInsets.symmetric(vertical: 8),
              child: Text(
                "Files",
                style: TextStyle(
                  fontSize: 20,
                  fontWeight: FontWeight.w600,
                  color: const Color(0xFFD2D2D2),
                  fontFamily: Theme.of(context)
                      .extension<FilesTheme>()!
                      .defaultFontFamily,
                ),
              ),
            ),

            // Home directory
            MechanixSectionList(
                theme: MechanixSectionListThemeData(
                  backgroundColor: WidgetStateProperty.all(Colors.transparent),
                ),
                sectionListItems: [
                  SectionListItems(
                    title: "Home directory",
                    titleTextStyle: listItemTitleTextStyle(context),
                    onTap: () {
                      setState(() => showHomeView = false);
                      controller.openDirectory(Directory(homeDir));
                    },
                    leading: Image.asset(Images.home, height: 20, width: 20),
                  ),

                  // Downloads
                  SectionListItems(
                    title: "Downloads",
                    titleTextStyle: listItemTitleTextStyle(context),
                    onTap: () {
                      setState(() => showHomeView = false);
                      controller.openDirectory(Directory(downloadsDir));
                    },
                    leading:
                        Image.asset(Images.downloads, height: 24, width: 24),
                  ),

                  // Documents
                  SectionListItems(
                    title: "Documents",
                    titleTextStyle: listItemTitleTextStyle(context),
                    onTap: () {
                      setState(() => showHomeView = false);
                      controller.openDirectory(Directory(documentsDir));
                    },
                    leading: Image.asset(Images.homeDocuments,
                        height: 24, width: 24),
                  ),
                ]),

            // Root dir
            MechanixSectionList(
                title: 'Hard Drive',
                theme: MechanixSectionListThemeData(
                  backgroundColor: WidgetStateProperty.all(Colors.transparent),
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
                    onTap: () {
                      setState(() => showHomeView = false);
                      controller.openDirectory(Directory("/"));
                    },
                    leading:
                        Image.asset(Images.hardDrive, height: 24, width: 24),
                  ),
                ]),
          ],
        ),
      ),
    );
  }

  Future<void> handleExtract(BuildContext context, FilesState state) async {
    final bloc = context.read<FilesBloc>();
    final zipPaths = state.zipFilePaths;

    int success = 0;
    int failed = 0;

    for (int i = 0; i < zipPaths.length; i++) {
      final completer = Completer<String>();

      final zipPath = zipPaths[i];
      final zipName = p.basenameWithoutExtension(zipPath);

      final uniquePath =
          await getUniqueExtractPath(p.join(currentPath, zipName));

      bloc.add(ExtractZipTo(
        zipPath,
        uniquePath,
        completer,
        index: i,
        total: zipPaths.length,
      ));

      String status = await completer.future;

      if (status == 'success') {
        success++;
      } else {
        failed++;
      }
    }

    bloc.add(ExtractZipBatchCompleted(
      successCount: success,
      failureCount: failed,
    ));
  }
}
