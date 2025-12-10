import 'dart:async';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/app_config.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_loading_dialog.dart';
import 'package:mechanix_files/src/commons/customWidgets/tab_clipper.dart';
import 'package:mechanix_files/src/commons/styles/file_theme_extenstions.dart';
import 'package:mechanix_files/src/controllers/file_manager_controller.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/blocs/file_state.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:mechanix_files/src/features/files/presentation/conflict_resolution_bottomsheet.dart';
import 'package:mechanix_files/src/features/files/presentation/extract_file_dialog.dart';
import 'package:mechanix_files/src/features/files/presentation/file_details_dialog.dart';
import 'package:mechanix_files/src/features/files/presentation/move_file_dialog.dart';
import 'package:widgets/constants.dart';
import 'package:widgets/widgets/bottomBar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottomBar/mechanix_bottom_bar_theme.dart';
import 'package:widgets/widgets/floating_action_bar/mechanix_floating_action_bar_theme.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';
import 'package:widgets/widgets/navigation_bar/mechanix_navigation_bar_theme.dart';
import 'package:widgets/widgets/textInput/mechanix_text_input_theme.dart';
import 'view_mode_notifier.dart';
import 'grid_view.dart';
import 'list_view.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:path/path.dart' as p;
import 'package:widgets/mechanix.dart';

class FileExplorerPage extends StatefulWidget {
  final String title;
  final List<FileItem> path;
  final String? startPath;

  const FileExplorerPage({
    super.key,
    this.title = "Files",
    this.path = const [],
    this.startPath,
  });

  @override
  State<FileExplorerPage> createState() => FileExplorerPageState();
}

class FileExplorerPageState extends State<FileExplorerPage> {
  final FileManagerController controller = FileManagerController();
  List<FileItem> displayedFiles = [];
  bool selectionMode = false;
  Set<String> selectedPaths = {};
  bool showHiddenFiles = false;
  bool isLoadingDialogShown = false;
  bool isCopyMode = false;
  bool isMoveMode = false;
  bool isExtractMode = false;
  bool isHomePageDir = false;
  String zipFilePath = '';
  List<String> copiedPaths = [];
  List<String> movedPaths = [];

  bool isFolderActionMenuOpen = false;
  bool isSelectionActionMenuOpen = false;
  bool isSortMenuOpen = false;

  final downloadsDir = AppConfig().downloadsDir;
  final documentsDir = AppConfig().documentsDir;
  final homeDir = AppConfig().homeDir;
  final recentDir = AppConfig().recentDir;

  final ValueNotifier<String> searchQuery = ValueNotifier('');
  bool isSearching = false;

  /// For list/grid view
  final ScrollController _scrollController = ScrollController();
  String currentPath = '';

  final ValueNotifier<bool> allSelectedNotifier = ValueNotifier(false);

  @override
  void initState() {
    super.initState();

    _scrollController.addListener(_onScroll);

    // Default to home directory if no startPath is provided
    final initialPath = widget.startPath ?? homeDir;
    controller.openDirectory(Directory(initialPath));

    controller.getPathNotifier.addListener(() {
      final newPath = controller.getPathNotifier.value;
      setState(() {
        currentPath = newPath;
      });
    });

    searchQuery.addListener(() {
      if (searchQuery.value.isEmpty) {
        controller.reload(); // Reload when user clears search
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
    _fabController.dispose();
    super.dispose();
  }

  final FloatingActionBarController _fabController =
      FloatingActionBarController();

  void _openFabMenuProgrammatically() {
    if (!mounted) return;
    _fabController.open();
  }

  void _closeFabMenuProgrammatically() {
    if (!mounted) return;
    _fabController.close();
  }

  void _toggleFabMenuProgrammatically() {
    _fabController.toggle();
  }

  final ValueNotifier<bool> hasSelectionNotifier = ValueNotifier(false);

  @override
  Widget build(BuildContext context) {
    final isAtRoot = currentPath == '/' || currentPath.isEmpty;

    final isDocumentsDir = currentPath == documentsDir;
    final isDownloadsDir = currentPath == downloadsDir;
    final isHomeDir = currentPath == homeDir;
    final isRecentDir = currentPath == recentDir;
    isHomePageDir = isHomeDir ||
        isDownloadsDir ||
        isDocumentsDir ||
        isAtRoot ||
        isRecentDir;

    return MultiBlocListener(
      listeners: [
        // Compression dialog handler
        BlocListener<FilesBloc, FilesState>(
          listenWhen: (previous, current) =>
              previous.compressionStatus != current.compressionStatus,
          listener: (context, state) {
            if (state.compressionStatus == FileCompressionStatus.inProgress) {
              showDialog(
                context: context,
                barrierColor: Colors.black.withOpacity(0.2),
                barrierDismissible: false,
                builder: (_) => buildLoadingDialog("Compressing..."),
              );
            } else if (state.compressionStatus ==
                    FileCompressionStatus.success ||
                state.compressionStatus == FileCompressionStatus.failure) {
              Navigator.of(context, rootNavigator: true).pop();

              if (state.compressionStatus == FileCompressionStatus.failure) {
                ScaffoldMessenger.of(context).showSnackBar(
                  SnackBar(
                    content: Text(
                      'Compression failed: ${state.compressionError}',
                      style: const TextStyle(color: Colors.red),
                    ),
                  ),
                );
              }

              if (state.compressionStatus == FileCompressionStatus.success) {
                ScaffoldMessenger.of(context).showSnackBar(
                  SnackBar(
                    content: const Text('Compression complete',
                        style: TextStyle(color: Colors.white)),
                    duration: const Duration(seconds: 2),
                    backgroundColor: Colors.grey[800],
                  ),
                );
              }
            }
          },
        ),
        // Global error message handler
        BlocListener<FilesBloc, FilesState>(
          listenWhen: (previous, current) =>
              previous.error != current.error && current.error != null,
          listener: (context, state) {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: Text(
                  'Error: ${state.error}',
                  style: const TextStyle(color: Colors.red),
                ),
              ),
            );
          },
        ),
        // Loading indicator for general loading states
        // BlocListener<FilesBloc, FilesState>(
        //   listenWhen: (previous, current) =>
        //       previous.loading != current.loading,
        //   listener: (context, state) async {
        //     if (state.loading && !isLoadingDialogShown) {
        //       isLoadingDialogShown = true;
        //       await showDialog(
        //         context: context,
        //         barrierColor: Colors.black.withOpacity(0.2),
        //         barrierDismissible: false,
        //         builder: (_) => buildLoadingDialog("Loading..."),
        //       );
        //       isLoadingDialogShown = false;
        //     } else if (!state.loading && isLoadingDialogShown) {
        //       Navigator.of(context, rootNavigator: true).pop();
        //     }
        //   },
        // ),
        // // Copy: Show conflict resolution dialog
        BlocListener<FilesBloc, FilesState>(
          listenWhen: (prev, curr) =>
              prev.conflictingPaths != curr.conflictingPaths,
          listener: (context, state) {
            if (!state.loading &&
                state.conflictingPaths.isNotEmpty &&
                state.isCopyMode) {
              showModalBottomSheet<ConflictResolutionStrategy>(
                context: context,
                useRootNavigator: true,
                isScrollControlled: true,
                backgroundColor: Colors.transparent,
                builder: (sheetContext) {
                  return ConflictResolutionBottomSheet(
                    conflictingPaths: state.conflictingPaths,
                    destinationPath: state.conflictDestinationPath,
                    controller: controller,
                  );
                },
              );
            }
          },
        ),
        // Controller settings sync
        BlocListener<FilesBloc, FilesState>(
          listenWhen: (prev, curr) =>
              prev.showHiddenFiles != curr.showHiddenFiles ||
              prev.currentSortBy != curr.currentSortBy,
          listener: (context, state) {
            controller.syncSettings(
              showHidden: state.showHiddenFiles,
              sortMode: state.currentSortBy,
            );
          },
        ),
      ],
      child: Scaffold(
        appBar: PreferredSize(
          preferredSize: const Size.fromHeight(48),
          child: Padding(
            padding: const EdgeInsets.only(top: 18, left: 16, right: 16),
            child: MechanixNavigationBar(
              automaticallyImplyLeading: false,
              theme: const MechanixNavigationBarThemeData(
                scrolledUnderElevation: 0,
              ),
              titleWidget: selectionMode
                  ? Text(
                      "${selectedPaths.length} Selected",
                      style: TextStyle(
                        fontSize: 20,
                        color: const Color(0xFFD2D2D2),
                        fontWeight: FontWeight.w600,
                        fontFamily: Theme.of(context)
                            .extension<FilesTheme>()!
                            .defaultFontFamily,
                      ),
                    ).padRight(24)
                  : ValueListenableBuilder<String>(
                      valueListenable: controller.getPathNotifier,
                      builder: (context, path, _) {
                        final title = widget.title == "Recent"
                            ? "Recents"
                            : (path == '/'
                                ? "Root"
                                : getCurrentFolderName(path));
                        return Text(
                          title,
                          style: TextStyle(
                            fontSize: 24,
                            color: const Color(0xFFD2D2D2),
                            fontWeight: FontWeight.w600,
                            fontFamily: Theme.of(context)
                                .extension<FilesTheme>()!
                                .defaultFontFamily,
                          ),
                        );
                      },
                    ),
            ),
          ),
        ),
        body: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const SizedBox(height: 6),
            Expanded(
              child: ContainerWidget(
                child: ValueListenableBuilder<String>(
                  valueListenable: searchQuery,
                  builder: (context, query, _) {
                    List<FileSystemEntity> filteredFilesRecent = [];
                    List<FileItem> filteredFiles = [];

                    if (widget.title == 'Recent') {
                      // Read the current file list from the bloc state synchronously
                      final fileSystemList = BlocProvider.of<FilesBloc>(context)
                          .state
                          .fileSystemList;
                      filteredFilesRecent = query.isEmpty
                          ? fileSystemList
                          : fileSystemList
                              .where((file) => p
                                  .basename(file.path)
                                  .toLowerCase()
                                  .contains(query.toLowerCase()))
                              .toList();
                    } else {
                      // Normal directory
                      filteredFiles = query.isEmpty
                          ? displayedFiles
                          : displayedFiles
                              .where((file) => file.name
                                  .toLowerCase()
                                  .contains(query.toLowerCase()))
                              .toList();
                    }

                    return ValueListenableBuilder<bool>(
                      valueListenable: viewModeNotifier,
                      builder: (context, isGrid, _) {
                        return isGrid
                            ? widget.title == 'recent'
                                ? buildGridViewForRecentFiles(
                                    context, filteredFilesRecent)
                                : buildGridView(
                                    context, _scrollController, controller)
                            : widget.title == 'Recent'
                                ? buildListViewForRecentFiles(
                                    context, filteredFilesRecent)
                                : buildListView(
                                    context, _scrollController, controller);
                      },
                    );
                  },
                ),
              ),
            ),
          ],
        ),
        bottomSheet: isSearching ? null : _buildBottomActionMenuBar(context),
      ),
    );
  }

  void handleBack() {
    controller.goToParentDirectory();
  }

  void homeNavigation() {
    Navigator.pop(context);
  }

  OverlayEntry? _searchOverlayEntry;

  void showSearchBottomSheet(
      BuildContext context, ValueNotifier<String> searchQuery) {
    final overlay = Overlay.of(context);

    _searchOverlayEntry = OverlayEntry(
      builder: (ctx) => Positioned(
        left: 0,
        right: 0,
        bottom: 0,
        child: Material(
          child: SizedBox(
            height: 60,
            child: MechanixTextInput.search(
              theme: MechanixTextInputThemeData(
                fillColor: const Color(0xFF151515),
                borderSide: const BorderSide(color: Color(0xFF151515)),
                focusedBorderSide: const BorderSide(color: Color(0xFF151515)),
                borderRadius: BorderRadius.circular(8),
              ),
              cursorColor:
                  Theme.of(context).extension<FilesTheme>()!.primaryColor,
              autofocus: false,
              prefixIcon: const IconWidget(
                iconPath: Images.search,
                iconColor: Color(0xFFD2D2D2),
                iconHeight: 24,
                iconWidth: 24,
              ),
              hintText: "Search here",
              onChanged: (query) => controller.search(query),
              onClear: () {
                clearSearch();
                _buildBottomActionMenuBar(context);
              },
            ),
          ),
        ),
      ),
    );

    overlay.insert(_searchOverlayEntry!);
  }

  void clearSearch() {
    setState(() {
      isSearching = false;
      searchQuery.value = '';
    });

    // safely remove overlay if still mounted
    _searchOverlayEntry?.remove();
    _searchOverlayEntry = null;

    // Reload directory content when clearing search
    controller.reload();
  }

  /// Shows bottom sheet for selecting sort mode
  Widget showSortMenu(
    BuildContext context,
    String currentSortByFromBloc,
    bool ascendingFromBloc,
  ) {
    // Convert sort key → SortBy enum
    final currentSort = sortByFromKey(currentSortByFromBloc);

    // Decide which ascending value to use:
    // 1. If controller already has value → use it
    // 2. Otherwise → use selector (bloc) value
    final effectiveAscending =
        controller.hasSortApplied ? controller.isAscending : ascendingFromBloc;

    final offset = const Offset(-8, -14);

    return MechanixMenu(
      offset: offset,
      dropdownPosition: DropdownPosition.topRight,
      animationDuration: const Duration(milliseconds: 300),
      buttonIcon: IconWidget(
        iconPath: Images.sortAscending,
        iconColor: isSortMenuOpen
            ? Theme.of(context).extension<FilesTheme>()!.primaryColor
            : const Color(0xFFD2D2D2),
      ),
      openMenu: () => setState(() => isSortMenuOpen = true),
      closeMenu: () => setState(() => isSortMenuOpen = false),
      items: [
        _buildSortMenuItem(context,
            key: 'name',
            label: 'Name',
            currentSort: currentSort,
            isAscending: effectiveAscending),
        _buildSortMenuItem(context,
            key: 'type',
            label: 'Date created',
            currentSort: currentSort,
            isAscending: effectiveAscending,
            isDisabled: true),
        _buildSortMenuItem(context,
            key: 'accessed_time',
            label: 'Date last opened',
            currentSort: currentSort,
            isAscending: effectiveAscending),
        _buildSortMenuItem(context,
            key: 'mod_time',
            label: 'Date modified',
            currentSort: currentSort,
            isAscending: effectiveAscending),
        _buildSortMenuItem(context,
            key: 'size',
            label: 'Size',
            currentSort: currentSort,
            isAscending: effectiveAscending),
      ],
    );
  }

  /// Builds a selectable menu item
  MechanixMenuItemsType _buildSortMenuItem(
    BuildContext context, {
    required String key,
    required String label,
    required SortBy currentSort,
    required bool isAscending,
    bool isDisabled = false,
  }) {
    final isSelected = sortByFromKey(key) == currentSort;

    // If selected → choose correct icon
    Widget? trailing;
    if (isSelected) {
      final icon = isAscending ? Images.sortAscending : Images.sortDescending;
      trailing = Padding(
        padding: const EdgeInsets.only(right: 14),
        child: Image.asset(
          icon,
          width: 20,
          height: 20,
          color: Theme.of(context).extension<FilesTheme>()!.primaryColor,
        ),
      );
    }

    return MechanixMenuItemsType(
      title: label,
      trailing: trailing,
      isSelected: isSelected,
      disabled: isDisabled,
      onTap: () {
        final sortBy = sortByFromKey(key);

        bool nextAscending = isAscending;

        // Toggle if same key tapped again
        if (currentSort == sortBy) {
          nextAscending = !isAscending;
        } else {
          nextAscending = true; // default ASC on first selection
        }

        controller.sortBy(sortBy, isAscending: nextAscending);
        controller.reload();

        handleSortMode(key, nextAscending);
      },
    );
  }

  bool get hasSelection => selectedPaths.isNotEmpty;

  Widget _buildBottomActionMenuBar(BuildContext context) {
    return MechanixBottomBar(
      leadingWidget: [
        BottomBarButton(
          iconPath: Images.back,
          onPressed: () {
            selectionMode
                ? clearSelection()
                : (isHomePageDir ? homeNavigation() : handleBack());
          },
        ),
      ],
      centerWidgetSpacing: 28,
      centerWidget: [
        BottomBarButton(
          iconWidget: IconWidget(
            iconPath: Images.search,
            iconColor: selectionMode ? Colors.grey.shade600 : Colors.white,
          ),
          onPressed: () {
            setState(() => isSearching = true);
            showSearchBottomSheet(context, searchQuery);
          },
          isDisabled: selectionMode,
        ),
        BottomBarButton.widget(
          widget: ValueListenableBuilder<bool>(
            valueListenable: viewModeNotifier,
            builder: (context, isList, _) {
              return IconButton(
                icon: Image.asset(
                  isList ? Images.list : Images.grid,
                  height: 24,
                ),
                onPressed: () {
                  viewModeNotifier.value = !viewModeNotifier.value;
                },
                highlightColor: Colors.transparent,
              );
            },
          ),
        ),
        BottomBarButton.widget(
          widget: BlocSelector<FilesBloc, FilesState,
              (String sortBy, bool ascending)>(
            selector: (state) => (state.currentSortBy, state.isAscending),
            builder: (context, sortData) {
              final currentSortBy = sortData.$1;
              final isAscending = sortData.$2;

              return showSortMenu(context, currentSortBy, isAscending);
            },
          ),
        ),
        BottomBarButton.extension(
          iconTheme: const MechanixBottomBarIconThemeData(),
          outsideClickDisabled: true,
          floatingActionBarController: _fabController,
          offset: const Offset(-100, -6),
          iconWidget: IconWidget(
            iconPath: Images.checkCircle,
            iconColor: selectionMode
                ? Theme.of(context).extension<FilesTheme>()!.primaryColor
                : Colors.white70,
          ),
          isSelected: selectionMode,
          onPressed: () {
            if (!selectionMode) enableSelect();
            isSelectionActionMenuOpen = false;
          },
          onExtensionClose: () {
            if (selectionMode) clearSelection();
            isFolderActionMenuOpen =
                false; // TODO: Temporary fix - to show inactive menu button color
          },
          floatingActionBarTheme: MechanixFloatingActionBarThemeData(
            barMainAxisAlignment: MainAxisAlignment.center,
            decoration: BoxDecoration(color: Colors.grey.shade900),
            width: double.infinity,
            barSpacing: 30,
          ),
          extensionWidgets: [
            BottomBarButton(
              iconPath: Images.copy,
              onPressed: () {
                hasSelection ? handleCopy() : null;
              },
            ),
            BottomBarButton(
              iconPath: Images.move,
              onPressed: () {
                hasSelection ? handleMove() : null;
              },
            ),
            BottomBarButton(
              iconWidget: IconWidget(
                iconPath: Images.share,
                iconColor: Colors.grey.shade600,
              ),
              onPressed: () {},
              isDisabled: true, //TODO : add share functionality
            ),
            BottomBarButton(
              iconPath: Images.delete,
              onPressed: () {
                hasSelection ? handleDelete() : null;
              },
            ),
          ],
        ),
      ],
      anchorWidget: [
        BottomBarButton.widget(
            widget: selectionMode
                ? buildSelectionActionsMenu(context)
                : buildFolderActionsMenu(context)),
      ],
    );
  }

  Widget buildSelectionActionsMenu(BuildContext context) {
    final offset = const Offset(-8, -14);
    // Check if selected files are ZIP files
    final isZipFileSelected = selectedPaths.isNotEmpty &&
        selectedPaths.every((p) => p.toLowerCase().endsWith('.zip'));
    final isOnlyFileSelected = selectedPaths.length ==
        1; // TODO: and check selected path is file or folder

    return MechanixMenu(
      dropdownPosition: DropdownPosition.topRight,
      offset: offset,
      buttonIcon: IconWidget(
          iconPath: Images.dots,
          iconColor: isSelectionActionMenuOpen
              ? Theme.of(context).extension<FilesTheme>()!.primaryColor
              : Colors.white70),
      openMenu: () {
        setState(() => isSelectionActionMenuOpen = true);
      },
      closeMenu: () {
        setState(() => isSelectionActionMenuOpen = false);
      },
      items: [
        MechanixMenuItemsType(
          leading: Image.asset(
            Images.extract,
            color: isZipFileSelected
                ? Colors.white70
                : Theme.of(context).extension<FilesTheme>()!.disableColor,
            height: 20,
          ),
          title: 'Extract',
          onTap: () {
            handleExtraction(context, selectedPaths);
          },
          disabled: !isZipFileSelected,
        ),
        MechanixMenuItemsType(
          leading: Image.asset(
            Images.compress,
            color: hasSelection
                ? Colors.white70
                : Theme.of(context).extension<FilesTheme>()!.disableColor,
            height: 20,
          ),
          title: 'Compress',
          onTap: hasSelection ? handleCompress : null,
          disabled: !hasSelection,
        ),
        MechanixMenuItemsType(
          leading: Image.asset(
            Images.duplicate,
            color: isOnlyFileSelected
                ? Colors.white70
                : Theme.of(context).extension<FilesTheme>()!.disableColor,
            height: 20,
          ),
          title: 'Duplicate',
          onTap: isOnlyFileSelected
              ? () => ()
              : null, // TODO: add duplicate functionality
          disabled: !isOnlyFileSelected,
        ),
        MechanixMenuItemsType(
          leading: Image.asset(
            Images.rename,
            color: selectedPaths.length == 1
                ? Colors.white70
                : Theme.of(context).extension<FilesTheme>()!.disableColor,
            height: 20,
          ),
          title: 'Rename',
          onTap: selectedPaths.length == 1
              ? () {
                  final selectedPath = selectedPaths.first;
                  showRenameSheet(initialName: p.basename(selectedPath));
                  clearSelection();
                }
              : null,
          disabled: selectedPaths.length != 1,
        ),
        MechanixMenuItemsType(
          title: "Select all",
          leading: Image.asset(
            Images.listChecks,
            color: Colors.white70,
            height: mechanixIconSize,
          ),
          onTap: () {
            handleSelectAll();
          },
        ),
        MechanixMenuItemsType(
          title: controller.showHiddenFiles
              ? "Hide Hidden Files"
              : "Show Hidden Files",
          leading: Image.asset(
            controller.showHiddenFiles ? Images.eye : Images.eyeSlash,
            color: Colors.white70,
            height: mechanixIconSize,
          ),
          onTap: () {
            _toggleHiddenFiles();
          },
        ),
        MechanixMenuItemsType(
          leading: Image.asset(
            Images.info,
            color: selectedPaths.length == 1
                ? Colors.white70
                : Theme.of(context).extension<FilesTheme>()!.disableColor,
            height: 20,
          ),
          title: 'Properties',
          onTap: selectedPaths.length == 1 ? handleProperties : null,
          disabled: selectedPaths.length != 1,
        ),
      ],
    ).padRight(8);
  }

  Widget buildFolderActionsMenu(BuildContext context) {
    final currentPath = controller.getPathNotifier.value;
    bool actionTaken = false;

    final state = BlocProvider.of<FilesBloc>(context).state;
    final offset = const Offset(-8, -14);
    bool isPasteDisabled = !(state.isCopyMode || state.isMoveMode);

    return MechanixMenu(
      offset: offset,
      dropdownPosition: DropdownPosition.topRight,
      animationDuration: const Duration(milliseconds: 300),
      buttonIcon: IconWidget(
          iconPath: Images.dots,
          iconColor: isFolderActionMenuOpen
              ? Theme.of(context).extension<FilesTheme>()!.primaryColor
              : Colors.white70),
      openMenu: () {
        setState(() => isFolderActionMenuOpen = true);
      },
      closeMenu: () {
        setState(() => isFolderActionMenuOpen = false);
      },
      items: [
        MechanixMenuItemsType(
          title: "Paste",
          leading: Image.asset(
            Images.paste,
            color: isPasteDisabled
                ? Theme.of(context).extension<FilesTheme>()!.disableColor
                : Colors.white70,
            height: mechanixIconSize,
          ),
          disabled: isPasteDisabled,
          onTap: () {
            actionTaken = true;
            handlePaste(context);
            clearSelection();
            reload();
          },
        ),
        MechanixMenuItemsType(
          title: "New Folder",
          leading: Image.asset(
            Images.createFolder,
            color: Colors.white70,
            height: mechanixIconSize,
          ),
          onTap: () {
            actionTaken = true;
            createFolderAndRename();
            clearSelection();
          },
        ),
        MechanixMenuItemsType(
          title: "Select all",
          leading: Image.asset(
            Images.listChecks,
            color: Colors.white70,
            height: mechanixIconSize,
          ),
          onTap: () {
            actionTaken = true;
            handleSelectAll();
          },
        ),
        MechanixMenuItemsType(
          title: controller.showHiddenFiles
              ? "Hide Hidden Files"
              : "Show Hidden Files",
          leading: Image.asset(
            controller.showHiddenFiles ? Images.eye : Images.eyeSlash,
            color: Colors.white70,
            height: mechanixIconSize,
          ),
          onTap: () {
            actionTaken = true;
            _toggleHiddenFiles();
          },
        ),
        MechanixMenuItemsType(
          title: "Properties",
          leading: Image.asset(
            Images.info,
            color: Colors.white70,
            height: mechanixIconSize,
          ),
          onTap: () {
            actionTaken = true;
            _showDetailsDialog(context, currentPath);
            clearSelection();
          },
        ),
      ],
    ).padRight(8);
  }

  void handleSelectAll() {
    final files = controller.paginatedEntities.value; // current page

    setState(() {
      // Always select ALL files (no toggle)
      selectedPaths = {for (final f in files) f.path};
      selectionMode = true;
    });

    // Update notifiers
    hasSelectionNotifier.value = selectedPaths.isNotEmpty;
    allSelectedNotifier.value = true;

    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted) return;
      _openFabMenuProgrammatically();
    });
  }

  bool isSelected(String path) => selectedPaths.contains(path);

  void toggleSelection(String path) {
    setState(() {
      if (selectedPaths.contains(path)) {
        selectedPaths.remove(path);
      } else {
        selectedPaths.add(path);
      }

      // Keep notifier in sync
      final newValue = selectedPaths.isNotEmpty;
      if (hasSelectionNotifier.value != newValue) {
        hasSelectionNotifier.value = newValue;
      }

      // Keep selectionMode true once initiated
      selectionMode = true;
    });

    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted) return;

      if (selectionMode) {
        _openFabMenuProgrammatically();
      } else {
        _closeFabMenuProgrammatically();
      }
    });
  }

  void clearSelection() {
    if (!mounted) return;

    setState(() {
      selectionMode = false;
      selectedPaths.clear();
    });

    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) {
        _closeFabMenuProgrammatically();
      }
    });
  }

  void enableSelect() {
    setState(() {
      selectionMode = true;
      WidgetsBinding.instance.addPostFrameCallback((_) {
        _openFabMenuProgrammatically(); // Open FAB when selection starts
      });
      selectedPaths.clear();
    });
  }

  Widget _buildCustomMoveSheet() {
    final filesBloc = context.read<FilesBloc>();
    return Builder(
      builder: (context) {
        final screenHeight = MediaQuery.of(context).size.height;
        final sheetWidth = MediaQuery.of(context).size.width;

        return GestureDetector(
          onTap: () => Navigator.pop(context),
          child: Stack(
            children: [
              Positioned(
                left: 0,
                right: 0,
                bottom: 0,
                child: GestureDetector(
                  onTap: () {}, // block outside taps
                  child: ClipPath(
                    clipper: TabClipper(shift: sheetWidth * 0.70),
                    child: Container(
                      height: screenHeight * 0.98,
                      decoration: const BoxDecoration(
                        color: Color(0xFF2E2E2E),
                        borderRadius: BorderRadius.only(
                          topLeft: Radius.circular(12),
                          topRight: Radius.circular(12),
                        ),
                      ),
                      child: MoveBottomSheetContent(
                        filesBloc: filesBloc,
                        selectedCount: selectedPaths.length,
                        reload: reload,
                        currentPath: controller.getPathNotifier.value,
                        rootContext: context,
                      ),
                    ),
                  ),
                ),
              ),
            ],
          ),
        );
      },
    );
  }

  void handleMove() {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) _closeFabMenuProgrammatically();
    });

    final filesBloc = context.read<FilesBloc>();
    filesBloc.add(StartMoveMode(selectedPaths.toList()));

    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.transparent,
      isScrollControlled: true,
      enableDrag: true,
      barrierColor: Colors.black54,
      builder: (_) {
        return BlocProvider.value(
          value: filesBloc, // keep same instance
          child: BlocListener<FilesBloc, FilesState>(
            listenWhen: (prev, curr) =>
                prev.conflictingPaths != curr.conflictingPaths,
            listener: (context, state) async {
              if (!state.loading &&
                  state.conflictingPaths.isNotEmpty &&
                  state.isMoveMode) {
                final rootContext = Navigator.of(context).context;

                final conflicts = state.conflictingPaths.map((path) {
                  return FileConflict(
                    path: path,
                    fileName: p.basename(path),
                    destination: state.conflictDestinationPath,
                  );
                }).toList();

                totalMovedCount = state.movedPaths.length;

                await handleConflictsSequentially(rootContext, conflicts);

                if (rootContext.mounted) {
                  final folderName = p.basename(state.conflictDestinationPath);

                  ScaffoldMessenger.of(rootContext).showSnackBar(
                    SnackBar(
                      content: Text(
                        totalMovedCount > 0
                            ? "Moved $totalMovedCount item${totalMovedCount > 1 ? 's' : ''} to '$folderName'"
                            : "No items were moved",
                        style: const TextStyle(color: Colors.white),
                      ),
                      duration: const Duration(seconds: 2),
                      backgroundColor: Colors.grey[800],
                    ),
                  );

                  // exit move mode
                  filesBloc.add(CancelMoveMode());
                  reload();
                }
              }
            },
            child: _buildCustomMoveSheet(),
          ),
        );
      },
    ).whenComplete(() => clearSelection());
  }

  void handleDelete() {
    _confirmDelete(context, selectedPaths);
    clearSelection();
  }

  void handleCopy() {
    BlocProvider.of<FilesBloc>(context)
        .add(StartCopyMode(selectedPaths.toList()));

    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(
            "Copied ${selectedPaths.length} item${selectedPaths.length > 1 ? 's' : ''}",
            style: const TextStyle(color: Colors.white)),
        duration: const Duration(seconds: 2),
        backgroundColor: Colors.grey[800],
      ),
    );
    clearSelection();
  }

  void showCompressOverlay({
    required String defaultZipName,
    required String destinationDirPath,
    required List<String> selectedPaths,
  }) {
    String currentName = defaultZipName;
    final filesBloc = BlocProvider.of<FilesBloc>(context);

    final overlay = Overlay.of(context);
    OverlayEntry? entry;

    entry = OverlayEntry(
      builder: (ctx) {
        return Positioned(
          left: 0,
          right: 0,
          bottom: 0,
          child: Material(
            color: Colors.transparent,
            child: StatefulBuilder(
              builder: (context, setState) {
                final bool isEmpty = currentName.trim().isEmpty;
                final bool showCheck = !isEmpty;

                return Padding(
                  padding: EdgeInsets.only(
                    bottom: MediaQuery.of(ctx).viewInsets.bottom,
                  ),
                  child: SizedBox(
                    height: 60,
                    child: MechanixTextInputTheme(
                      style: MechanixTextInputThemeData(
                        fillColor: const Color(0xFF151515),
                        borderSide: const BorderSide(color: Color(0xFF151515)),
                        focusedBorderSide:
                            const BorderSide(color: Color(0xFF151515)),
                        borderRadius: BorderRadius.circular(8),
                      ),
                      child: MechanixTextInput.textInput(
                        cursorColor: Theme.of(context)
                            .extension<FilesTheme>()!
                            .primaryColor,
                        initialValue: defaultZipName,
                        onChanged: (v) {
                          setState(() {
                            currentName = v;
                          });
                        },
                        anchorWidget: showCheck
                            ? IconButton(
                                icon: const Icon(Icons.check,
                                    color: Colors.white),
                                onPressed: () async {
                                  final trimmed = currentName.trim();

                                  if (trimmed.isEmpty) return;

                                  final zipPath = p.join(
                                    destinationDirPath,
                                    trimmed.endsWith('.zip')
                                        ? trimmed
                                        : '$trimmed.zip',
                                  );

                                  final exists = await File(zipPath).exists();
                                  if (exists) {
                                    setState(() {
                                      currentName = "";
                                    });
                                    return;
                                  }

                                  entry?.remove();

                                  filesBloc.add(
                                    CompressEntitiesEvent(
                                      sourcePaths: selectedPaths.toList(),
                                      destinationZipPath: zipPath,
                                      controller: controller,
                                    ),
                                  );

                                  clearSelection();
                                },
                              )
                            : IconButton(
                                icon: const Icon(Icons.close,
                                    color: Colors.white),
                                onPressed: () {
                                  entry?.remove();
                                },
                              ),
                      ),
                    ),
                  ),
                );
              },
            ),
          ),
        );
      },
    );

    overlay.insert(entry);
  }

  void handleCompress() async {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) _closeFabMenuProgrammatically();
    });
    if (selectedPaths.isEmpty) return;

    final destinationDirPath = p.dirname(selectedPaths.first);
    const defaultZipName = "Archive.zip";

    showCompressOverlay(
      defaultZipName: defaultZipName,
      destinationDirPath: destinationDirPath,
      selectedPaths: selectedPaths.toList(),
    );
  }

  Widget _buildCustomExtractSheet() {
    final filesBloc = context.read<FilesBloc>();
    return Builder(
      builder: (context) {
        final screenHeight = MediaQuery.of(context).size.height;
        final sheetWidth = MediaQuery.of(context).size.width;

        return GestureDetector(
          onTap: () => Navigator.pop(context),
          child: Stack(
            children: [
              Positioned(
                left: 0,
                right: 0,
                bottom: 0,
                child: GestureDetector(
                  onTap: () {}, // block outside taps
                  child: ClipPath(
                    clipper: TabClipper(shift: sheetWidth * 0.70),
                    child: Container(
                      height: screenHeight * 0.98,
                      decoration: const BoxDecoration(
                        color: Color(0xFF2E2E2E),
                        borderRadius: BorderRadius.only(
                          topLeft: Radius.circular(12),
                          topRight: Radius.circular(12),
                        ),
                      ),
                      child: ExtractBottomSheetContent(
                        filesBloc: filesBloc,
                        selectedCount: selectedPaths.length,
                        reload: reload,
                        currentPath: controller.getPathNotifier.value,
                        rootContext: context,
                      ),
                    ),
                  ),
                ),
              ),
            ],
          ),
        );
      },
    );
  }

  void handleExtraction(BuildContext context, Set<String> selectedPaths) {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) _closeFabMenuProgrammatically();
    });

    final filesBloc = context.read<FilesBloc>();
    filesBloc.add(StartExtractMode(selectedPaths.toList()));

    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.transparent,
      isScrollControlled: true,
      enableDrag: true,
      barrierColor: Colors.black54,
      builder: (_) {
        return BlocProvider.value(
          value: filesBloc, // keep same bloc instance
          child: BlocListener<FilesBloc, FilesState>(
            listenWhen: (prev, curr) =>
                prev.extractStatus != curr.extractStatus &&
                curr.extractStatus == FileExtractStatus.completed,
            listener: (context, state) async {
              final rootContext = Navigator.of(context).context;
              if (!rootContext.mounted) return;

              final s = state.extractSuccessCount;
              final f = state.extractFailureCount;

              String msg;
              if (f == 0) {
                msg = "Extracted $s file${s > 1 ? 's' : ''} successfully";
              } else if (s == 0) {
                msg = "Failed to extract $f file${f > 1 ? 's' : ''}";
              } else {
                msg = "$s extracted successfully • $f failed";
              }

              // Show final summary
              ScaffoldMessenger.of(rootContext).showSnackBar(
                SnackBar(
                  content: Text(
                    msg,
                    style: const TextStyle(color: Colors.white),
                  ),
                  backgroundColor: Colors.grey[800],
                  duration: const Duration(seconds: 2),
                ),
              );

              // Exit extract mode
              filesBloc.add(CancelExtractMode());
              reload();
            },
            child: _buildCustomExtractSheet(),
          ),
        );
      },
    ).whenComplete(() {
      clearSelection();
    });
  }

  void handleProperties() {
    if (selectedPaths.length == 1) {
      final selectedPath = selectedPaths.first;
      _showDetailsDialog(context, selectedPath);
    } else {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(
          content: const Text("Select a single item to view details",
              style: TextStyle(color: Colors.white)),
          duration: const Duration(seconds: 1),
          backgroundColor: Colors.grey[800],
        ),
      );
    }
    clearSelection();
  }

  void handlePaste(BuildContext context) {
    final targetPath = controller.getPathNotifier.value;

    final bloc = BlocProvider.of<FilesBloc>(context);
    // Read the current bloc state synchronously and perform the paste operation.
    final state = bloc.state;

    if (state.isCopyMode) {
      bloc.add(Copy(
        sourcePaths: state.copiedPaths,
        destinationPath: targetPath,
        controller: controller,
      ));
      bloc.add(CancelCopyMode());
    }
  }

  void handleSortMode(String value, bool isAscending) {
    BlocProvider.of<FilesBloc>(context).add(SortFiles(value, isAscending));
  }

  void _showDetailsDialog(BuildContext context, String path) {
    final bloc = context.read<FilesBloc>();
    bloc.add(FetchFileDetails(path));

    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.transparent,
      isScrollControlled: true,
      barrierColor: Colors.transparent,
      enableDrag: false,
      builder: (sheetContext) {
        return GestureDetector(
          behavior: HitTestBehavior.opaque,
          onTap: () => Navigator.pop(sheetContext),
          child: FileDetailsDialog(path: path),
        );
      },
    );
  }

  Future<void> createFolderAndRename() async {
    final path = controller.getCurrentPath;
    final bloc = context.read<FilesBloc>();

    final folderName = await generateUniqueFolderName(path);

    // Create the folder
    bloc.add(CreateFolder(
      path: path,
      folderName: folderName,
      controller: controller,
    ));

    // After creation, show rename sheet with initial value
    Future.delayed(const Duration(milliseconds: 200), () {
      showRenameSheet(initialName: folderName);
    });
  }

  void showRenameSheet({required String initialName}) {
    String folderName = initialName;
    final filesBloc = BlocProvider.of<FilesBloc>(context);
    final oldPath = p.join(controller.getCurrentPath, initialName);

    controller.markNewFolder(oldPath);

    final overlay = Overlay.of(context);

    OverlayEntry? entry;

    entry = OverlayEntry(
      builder: (ctx) {
        return Positioned(
          left: 0,
          right: 0,
          bottom: 0,
          child: Material(
            color: Colors.transparent,
            child: StatefulBuilder(
              builder: (context, setState) {
                final bool isEmpty = folderName.trim().isEmpty;
                final bool isSame = folderName.trim() == initialName.trim();
                final bool showCheck = !isEmpty && !isSame;

                return Padding(
                  padding: EdgeInsets.only(
                    bottom: MediaQuery.of(ctx).viewInsets.bottom,
                  ),
                  child: SizedBox(
                    height: 60,
                    child: MechanixTextInputTheme(
                      style: MechanixTextInputThemeData(
                        fillColor: const Color(0xFF151515),
                        borderSide: const BorderSide(color: Color(0xFF151515)),
                        focusedBorderSide:
                            const BorderSide(color: Color(0xFF151515)),
                        borderRadius: BorderRadius.circular(8),
                      ),
                      child: MechanixTextInput.textInput(
                        cursorColor: Theme.of(context)
                            .extension<FilesTheme>()!
                            .primaryColor,
                        onChanged: (v) {
                          setState(() {
                            folderName = v;
                          });
                        },
                        initialValue: initialName,
                        anchorWidget: showCheck
                            ? IconButton(
                                icon: const Icon(Icons.check,
                                    color: Colors.white),
                                onPressed: () {
                                  entry?.remove();
                                  filesBloc.add(Rename(
                                    oldPath: oldPath,
                                    newName: folderName,
                                    controller: controller,
                                  ));
                                  controller.clearNewFolder();
                                },
                              )
                            : IconButton(
                                icon: const Icon(Icons.close,
                                    color: Colors.white),
                                onPressed: () {
                                  entry?.remove();
                                  controller.clearNewFolder();
                                },
                              ),
                      ),
                    ),
                  ),
                );
              },
            ),
          ),
        );
      },
    );

    overlay.insert(entry);
  }

  void _toggleHiddenFiles() {
    controller.toggleShowHiddenFiles();
    final filesBloc = BlocProvider.of<FilesBloc>(context);
    filesBloc.add(ToggleHiddenFiles());
  }

  void reload() {
    controller.reload();
  }

  void _confirmDelete(BuildContext context, Set<String> selectedPaths) {
    final pathsToDelete = selectedPaths.toList();

    final isSingle = pathsToDelete.length == 1;
    final message = isSingle
        ? "This action will delete the file permanently"
        : "This action will delete the files permanently";

    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.transparent,
      isScrollControlled: true,
      builder: (bottomSheetContext) {
        final double bottomSheetWidth =
            MediaQuery.of(bottomSheetContext).size.width;

        return ClipPath(
          clipper: TabClipper(shift: bottomSheetWidth * 0.5),
          child: Container(
            padding:
                const EdgeInsets.only(left: 16, right: 16, bottom: 16, top: 32),
            decoration: BoxDecoration(
              color: Colors.grey[850],
            ),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                RichText(
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  text: TextSpan(
                    children: isSingle
                        ? [
                            const TextSpan(
                              text: "Delete ",
                              style: TextStyle(
                                fontSize: 18,
                                color: Colors.white70,
                              ),
                            ),
                            TextSpan(
                              text:
                                  "'${pathsToDelete.first.split('/').last}' ?",
                              style: const TextStyle(
                                fontWeight: FontWeight.w600,
                                fontSize: 18,
                                color: Colors.white,
                              ),
                            ),
                          ]
                        : [
                            const TextSpan(
                              text: "Delete ",
                              style: TextStyle(
                                fontSize: 18,
                                color: Colors.white70,
                              ),
                            ),
                            TextSpan(
                              text: "'${pathsToDelete.length}' files?",
                              style: const TextStyle(
                                fontWeight: FontWeight.w600,
                                fontSize: 18,
                                color: Colors.white,
                              ),
                            ),
                          ],
                  ),
                ),
                const SizedBox(height: 8),
                Text(
                  message,
                  style: const TextStyle(color: Colors.white70),
                ),
                const SizedBox(height: 16),
                Row(
                  children: [
                    Expanded(
                        child: MechanixFilledButton(
                      onPressed: () => Navigator.pop(bottomSheetContext),
                      theme: buttonThemeData(context,
                          type: MechanixButtonType.cancel),
                      label: "Cancel",
                    )),
                    const SizedBox(width: 12),
                    Expanded(
                      child: MechanixFilledButton(
                        theme: buttonThemeData(context,
                            type: MechanixButtonType.delete),
                        label: "Delete",
                        onPressed: () {
                          Navigator.pop(bottomSheetContext);
                          BlocProvider.of<FilesBloc>(context)
                              .add(DeleteEntities(pathsToDelete, controller));
                          reload();
                        },
                      ),
                    ),
                  ],
                ),
              ],
            ),
          ),
        );
      },
    );
  }
}
