import 'dart:async';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/app_config.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_loading_dialog.dart';
import 'package:mechanix_files/src/controllers/file_manager_controller.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/blocs/file_state.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:mechanix_files/src/features/files/presentation/extract_file_dialog.dart';
import 'package:mechanix_files/src/features/files/presentation/move_file_dialog.dart';
import 'package:widgets/constants.dart';
import 'package:widgets/widgets/floating_action_bar/mechanix_floating_action_bar_theme.dart';
import 'package:widgets/widgets/listItems/mechanix_simple_list_theme.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/mechanix_menu_theme.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';
import 'package:widgets/widgets/navigation_bar/mechanix_navigation_bar_theme.dart';
import 'package:widgets/widgets/search_bar/mechanix_search_bar.dart';
import 'package:widgets/widgets/sectionList/mechanix_section_list_theme.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';
import 'package:widgets/widgets/textInput/mechanix_text_input_theme.dart';
import 'view_mode_notifier.dart';
import 'grid_view.dart';
import 'list_view.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:path/path.dart' as p;
import 'dart:math' as Math;
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
  String zipFilePath = '';
  List<String> copiedPaths = [];
  List<String> movedPaths = [];

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
    final isHomePageDir = isHomeDir ||
        isDownloadsDir ||
        isDocumentsDir ||
        isAtRoot ||
        isRecentDir;

    void handleBack() {
      controller.goToParentDirectory();
    }

    void homeNavigation() {
      Navigator.pop(context);
    }

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
              final fileName = p.basename(state.conflictingPaths.first);
              showModalBottomSheet<ConflictResolutionStrategy>(
                context: context,
                useRootNavigator: true,
                isScrollControlled: true,
                backgroundColor: Colors.transparent,
                builder: (sheetContext) {
                  return Container(
                    decoration: BoxDecoration(
                      color: Colors.grey[850],
                      borderRadius:
                          const BorderRadius.vertical(top: Radius.circular(16)),
                    ),
                    padding: const EdgeInsets.all(16),
                    child: SafeArea(
                      top: false,
                      child: Column(
                        mainAxisSize: MainAxisSize.min,
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            '‘$fileName’ already exists',
                            style: const TextStyle(
                              fontWeight: FontWeight.bold,
                              fontSize: 16,
                              color: Colors.white,
                            ),
                          ),
                          const SizedBox(height: 8),
                          const Text(
                            'What would you like to do?',
                            style: TextStyle(color: Colors.white70),
                          ),
                          const SizedBox(height: 16),
                          Row(
                            children: [
                              Expanded(
                                child: MechanixOutlinedButton(
                                  label: "Cancel",
                                  textColor: Colors.white,
                                  borderRadius: 50,
                                  borderWidth: 0.5,
                                  onPressed: () {
                                    context.read<FilesBloc>().add(
                                          ContinueCopyWithConflictResolution(
                                            sourcePaths: state.conflictingPaths,
                                            destinationPath:
                                                state.conflictDestinationPath,
                                            strategy:
                                                ConflictResolutionStrategy.skip,
                                            controller: controller,
                                          ),
                                        );
                                    Navigator.pop(sheetContext);
                                  },
                                ),
                              ),
                              const SizedBox(width: 12),
                              Expanded(
                                child: MechanixElevatedButton(
                                  label: "Replace",
                                  backgroundColor: Colors.blue,
                                  textColor: Colors.white,
                                  borderRadius: 50,
                                  onPressed: () {
                                    context.read<FilesBloc>().add(
                                          ContinueCopyWithConflictResolution(
                                            sourcePaths: state.conflictingPaths,
                                            destinationPath:
                                                state.conflictDestinationPath,
                                            strategy: ConflictResolutionStrategy
                                                .replace,
                                            controller: controller,
                                          ),
                                        );
                                    Navigator.pop(sheetContext);
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
        appBar: MechanixNavigationBar(
          theme: const MechanixNavigationBarThemeData(
            titleSpacing: 0,
          ),
          titleWidget: selectionMode
              ? Text("Select", style: context.textTheme.bodySmall)
              : isSearching
                  ? Text("Search", style: context.textTheme.bodySmall)
                  : ValueListenableBuilder<String>(
                      valueListenable: controller.getPathNotifier,
                      builder: (context, path, _) {
                        final title = widget.title == "Recent"
                            ? "Recents"
                            : (path == '/'
                                ? "Root"
                                : getCurrentFolderName(path));
                        return Text(title, style: context.textTheme.bodySmall);
                      },
                    ),
          leadingWidget: IconButton(
            icon:
                const Icon(Icons.arrow_back_ios, size: 16, color: Colors.blue),
            onPressed: selectionMode
                ? clearSelection
                : isSearching
                    ? clearSearch
                    : (isHomePageDir ? homeNavigation : handleBack),
            highlightColor: Colors.transparent,
          ).padLeft(8),
          actionWidgets: selectionMode
              ? [
                  Text(
                    "${selectedPaths.length} item${selectedPaths.length > 1 ? 's' : ''} selected",
                    style: const TextStyle(color: Colors.grey, fontSize: 14),
                  ).padRight(24),
                ]
              : isSearching
                  ? null
                  : [
                      // View toggle
                      ValueListenableBuilder<bool>(
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

                      // Sort options
                      BlocSelector<FilesBloc, FilesState, String>(
                        selector: (state) => state.currentSortBy,
                        builder: (context, currentSortBy) {
                          return showSortMenu(context, currentSortBy);
                        },
                      ),

                      // Search toggle
                      IconButton(
                        icon: const Icon(Icons.search,
                            color: Colors.white, size: 22),
                        onPressed: () {
                          setState(() => isSearching = true);
                          showSearchBottomSheet(context, searchQuery);
                        },
                      ),

                      // More options
                      buildMoreOptionsMenu(context),
                    ],
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
        floatingActionButtonLocation: FloatingActionButtonLocation.centerFloat,
        floatingActionButton:
            selectionMode ? _buildFloatingActionMenu(context) : null,
      ),
    );
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
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 12),
          child: SizedBox(
            height: 48,
            child: MechanixSearchBar(
              autoFocus: true,
              hintText: "Type here",
              onChanged: (query) => controller.search(query),
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
  Widget showSortMenu(BuildContext context, String currentSortBy) {
    final currentSort = sortByFromKey(currentSortBy);
    final isAscending = controller.isSizeAscending;
    final selectedKey = keyFromSort(currentSort, isAscending);
    final isSizeSort = selectedKey.startsWith('size');
    final isDescending = selectedKey == 'size_desc';
    final newSizeKey = isDescending ? 'size_asc' : 'size_desc';
    final sizeIcon =
        isDescending ? Images.sortDescending : Images.sortAscending;

    // Calculate offset so menu appears at bottom of screen
    final screenHeight = MediaQuery.of(context).size.height;
    final menuHeight = menuItemHeight * 4; // Approximate menu height
    final offset = Offset(80, screenHeight - menuHeight - 48);

    return MechanixMenu(
      theme: const MechanixMenuThemeData(
        constraints: BoxConstraints(maxWidth: double.infinity),
        itemHeight: menuItemHeight,
      ),
      offset: offset,
      dropdownPosition: DropdownPosition.bottomRight,
      animationDuration: const Duration(milliseconds: 300),
      buttonIcon: const IconWidget(
        iconPath: Images.sortAscending,
        iconColor: Colors.white,
      ),
      items: [
        _buildSortMenuItem(context, key: 'name', label: 'Name'),
        _buildSortMenuItem(
          context,
          key: newSizeKey,
          label: 'Size',
          trailingIcon:
              isSizeSort ? Image.asset(sizeIcon, width: 16, height: 16) : null,
        ),
        _buildSortMenuItem(context, key: 'type', label: 'Type'),
        _buildSortMenuItem(context, key: 'mod_time', label: 'File Modified'),
      ],
    );
  }

  /// Builds a selectable menu item
  MechanixMenuItemsType _buildSortMenuItem(
    BuildContext context, {
    required String key,
    required String label,
    Widget? trailingIcon,
  }) {
    final selectedKey = keyFromSort(
      controller.getSortedByNotifier.value,
      controller.isSizeAscending,
    );

    final isSelected = selectedKey == key ||
        (key.startsWith('size') && selectedKey.startsWith('size'));

    return MechanixMenuItemsType(
      title: label,
      leading: Icon(
        isSelected ? Icons.radio_button_checked : Icons.radio_button_unchecked,
        color: isSelected ? Colors.blue : Colors.grey,
        size: 18,
      ),
      trailing: trailingIcon,
      onTap: () {
        final sortBy = sortByFromKey(key);
        handleSortMode(sortBy.name);

        // Navigator.of(context).pop();

        bool? ascending;
        if (key == 'size_asc') ascending = true;
        if (key == 'size_desc') ascending = false;

        controller.sortBy(sortBy, sizeAscending: ascending);
        controller.reload();
      },
    );
  }

  bool get hasSelection => selectedPaths.isNotEmpty;

  Widget _buildFloatingActionMenu(BuildContext context) {
    return MechanixFloatingActionBar(
      theme: MechanixFloatingActionBarThemeData(
        width: 500,
        decoration: BoxDecoration(
          color: Colors.grey[800],
          borderRadius: BorderRadius.circular(50),
        ),
      ),
      dropdownPosition: DropdownPosition.topCenter,
      floatingActionBarController: _fabController,
      isMenuButtonRequired: false,
      outsideClickDisabled: true,
      menus: [
        // Wrap only reactive buttons in ValueListenableBuilder
        ValueListenableBuilder<bool>(
          valueListenable: allSelectedNotifier,
          builder: (context, allSelected, _) {
            return IconButton(
              onPressed: onToggleSelectAll,
              icon: Image.asset(
                Images.listChecks,
                color: allSelected ? Colors.blue : Colors.white,
                height: 20,
              ),
            );
          },
        ),

        ValueListenableBuilder<bool>(
          valueListenable: hasSelectionNotifier,
          builder: (context, hasSelection, _) {
            return IconButton(
              onPressed: hasSelection ? handleCopy : null,
              icon: const ImageIcon(AssetImage(Images.copy), size: 20),
              color: hasSelection ? Colors.white : Colors.grey,
            );
          },
        ),
        ValueListenableBuilder<bool>(
          valueListenable: hasSelectionNotifier,
          builder: (context, hasSelection, _) {
            return IconButton(
              onPressed: hasSelection ? handleDelete : null,
              icon: const ImageIcon(AssetImage(Images.delete), size: 20),
              color: hasSelection ? Colors.white : Colors.grey,
            );
          },
        ),

        // Dropdown menu (some items depend on selection)
        ValueListenableBuilder<bool>(
          valueListenable: hasSelectionNotifier,
          builder: (context, hasSelection, _) {
            return MechanixMenu(
              dropdownPosition: DropdownPosition.topCenter,
              buttonIcon: const IconWidget(
                iconPath: Images.dots,
                iconColor: Colors.white,
              ),
              items: [
                MechanixMenuItemsType(
                  leading: Image.asset(
                    Images.rename,
                    color: selectedPaths.length == 1
                        ? Colors.white70
                        : Colors.grey,
                    height: 20,
                  ),
                  title: 'Rename',
                  onTap: selectedPaths.length == 1
                      ? () {
                          final selectedPath = selectedPaths.first;
                          _showRenameDialog(selectedPath);
                        }
                      : null,
                  disabled: selectedPaths.length != 1,
                ),
                MechanixMenuItemsType(
                  leading: Image.asset(
                    Images.move,
                    color: hasSelection ? Colors.white70 : Colors.grey,
                    height: 20,
                  ),
                  title: 'Move',
                  onTap: hasSelection ? handleMove : null,
                  disabled: !hasSelection,
                ),
                MechanixMenuItemsType(
                  leading: Image.asset(
                    Images.createFolder,
                    color: Colors.white70,
                    height: 20,
                  ),
                  title: 'Create folder',
                  onTap: showCreateFolderDialogFloatingMenu,
                ),
                MechanixMenuItemsType(
                  leading: Image.asset(
                    Images.compress,
                    color: hasSelection ? Colors.white70 : Colors.grey,
                    height: 20,
                  ),
                  title: 'Compress',
                  onTap: hasSelection ? handleCompress : null,
                  disabled: !hasSelection,
                ),
                MechanixMenuItemsType(
                  leading: Image.asset(
                    Images.info,
                    color: selectedPaths.length == 1
                        ? Colors.white70
                        : Colors.grey,
                    height: 20,
                  ),
                  title: 'Properties',
                  onTap: selectedPaths.length == 1 ? handleProperties : null,
                  disabled: selectedPaths.length != 1,
                ),
              ],
            );
          },
        ),

        IconButton(
          onPressed: () {
            if (mounted) {
              final bloc = BlocProvider.of<FilesBloc>(context);
              if (isCopyMode) {
                bloc.add(CancelCopyMode());
              } else if (isMoveMode) {
                bloc.add(CancelMoveMode());
              } else if (isExtractMode) {
                bloc.add(CancelExtractMode());
              }
              clearSelection();
            }
          },
          icon: const Icon(Icons.close, size: 20),
        ),
      ],
    );
  }

  Widget buildMoreOptionsMenu(BuildContext context) {
    final currentPath = controller.getPathNotifier.value;
    bool actionTaken = false;
    final state = BlocProvider.of<FilesBloc>(context).state;
    // Calculate offset so menu appears at bottom of screen
    final screenHeight = MediaQuery.of(context).size.height;
    final pasteMenuHeight =
        state.isCopyMode || state.isMoveMode ? menuItemHeight : 0;
    final menuHeight =
        (menuItemHeight * 8) + pasteMenuHeight; // Approximate menu height
    final offset = Offset(0, screenHeight - menuHeight - 54);

    return MechanixMenu(
      theme: const MechanixMenuThemeData(
          constraints: BoxConstraints(maxWidth: double.infinity),
          itemHeight: menuItemHeight),
      offset: offset,
      dropdownPosition: DropdownPosition.bottomRight,
      animationDuration: const Duration(milliseconds: 300),
      buttonIcon: const IconWidget(
        iconPath: Images.dots,
        iconColor: Colors.white,
      ),
      items: [
        MechanixMenuItemsType(
          title: "Select",
          leading: Image.asset(
            Images.listChecks,
            color: Colors.white70,
            height: mechanixIconSize,
          ),
          onTap: () {
            actionTaken = true;
            clearSelection();
            enableSelect();
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
            showCreateFolderDialog();
            clearSelection();
          },
        ),
        MechanixMenuItemsType(
          title: "Refresh",
          leading: Image.asset(
            Images.refresh,
            color: Colors.white70,
            height: mechanixIconSize,
          ),
          onTap: () {
            actionTaken = true;
            reload();
            clearSelection();
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
            clearSelection();
          },
        ),
        MechanixMenuItemsType(
          title: "Copy",
          leading: Image.asset(
            Images.copy,
            color: Colors.white70,
            height: mechanixIconSize,
          ),
          onTap: () {
            actionTaken = true;
            selectedPaths.add(currentPath);
            handleCopy();
            clearSelection();
          },
        ),
        MechanixMenuItemsType(
          title: "Copy Path",
          leading: const Icon(Icons.copy_all,
              color: Colors.white70, size: mechanixIconSize),
          onTap: () {
            actionTaken = true;
            copyPath(currentPath);
            clearSelection();
          },
        ),
        if (state.isCopyMode || state.isMoveMode)
          MechanixMenuItemsType(
            title: "Paste",
            leading: const Icon(Icons.paste, color: Colors.white70),
            onTap: () {
              actionTaken = true;
              handlePaste(context);
              clearSelection();
              reload();
            },
          ),
        MechanixMenuItemsType(
          title: "Open in Terminal",
          leading: Image.asset(
            Images.terminal,
            color: Colors.white70,
            height: mechanixIconSize,
          ),
          onTap: () {
            actionTaken = true;
            openInTerminal(currentPath);
            clearSelection();
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
    );
  }

  void onToggleSelectAll() {
    final files = controller.paginatedEntities.value; // current page

    final allSelectedNow = selectedPaths.length == files.length;

    setState(() {
      if (allSelectedNow) {
        selectedPaths.clear();
      } else {
        selectedPaths = {for (final f in files) f.path};
        selectionMode = true;
      }
    });

    // Update notifiers after UI state changes
    hasSelectionNotifier.value = selectedPaths.isNotEmpty;
    allSelectedNotifier.value = !allSelectedNow; // toggle value

    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted) return;

      if (selectionMode) {
        _openFabMenuProgrammatically();
      } else {
        _closeFabMenuProgrammatically();
      }
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

  void handleMove() {
    _closeFabMenuProgrammatically();
    List<String> selectedPathsList = selectedPaths.toList();
    final filesBloc = BlocProvider.of<FilesBloc>(context); // get bloc
    // Start move mode
    filesBloc.add(StartMoveMode(selectedPathsList));

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
                'Select destination',
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
                      onTap: () => onTap(
                          context, homeDir, "Home", filesBloc, () => reload()),
                      leading: const IconWidget(
                        iconWidth: 20,
                        iconHeight: 20,
                        iconPath: Images.home,
                        iconColor: Colors.blueAccent,
                      ),
                      defaultTrailingIcon: false,
                      trailing: _trailingIcon(),
                    ),
                    SectionListItems(
                      title: "Downloads",
                      titleTextStyle: const TextStyle(fontSize: 14),
                      onTap: () => onTap(context, downloadsDir, "Downloads",
                          filesBloc, () => reload()),
                      leading: const IconWidget(
                        iconWidth: 20,
                        iconHeight: 20,
                        iconPath: Images.downloads,
                        iconColor: Colors.deepPurpleAccent,
                      ),
                      defaultTrailingIcon: false,
                      trailing: _trailingIcon(),
                    ),
                    SectionListItems(
                      title: "Documents",
                      titleTextStyle: const TextStyle(fontSize: 14),
                      onTap: () => onTap(context, documentsDir, "Documents",
                          filesBloc, () => reload()),
                      leading: const IconWidget(
                        iconWidth: 20,
                        iconHeight: 20,
                        iconPath: Images.homeDocuments,
                        iconColor: Colors.orangeAccent,
                      ),
                      defaultTrailingIcon: false,
                      trailing: _trailingIcon(),
                    ),
                    SectionListItems(
                      title: "Root (/)",
                      titleTextStyle: const TextStyle(fontSize: 14),
                      onTap: () => onTap(
                          context, "/", "Root", filesBloc, () => reload()),
                      leading: const IconWidget(
                        iconWidth: 20,
                        iconHeight: 20,
                        iconPath: Images.hardDrive,
                      ),
                      defaultTrailingIcon: false,
                      trailing: _trailingIcon(),
                    ),
                  ],
                ),
              ),
            ],
          ),
        );
      },
    ).whenComplete(() {
      // Clear selection only if needed when bottom sheet is closed
      clearSelection();
    });
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

  void handleCompress() async {
    _closeFabMenuProgrammatically();
    if (selectedPaths.isEmpty) return;

    final destinationDirPath = p.dirname(selectedPaths.first);
    final defaultZipName = "Archive.zip";
    String zipName = defaultZipName;
    final filesBloc = BlocProvider.of<FilesBloc>(context);

    await showModalBottomSheet(
      context: context,
      backgroundColor: Colors.black,
      isScrollControlled: true,
      builder: (bottomSheetContext) {
        String? errorText;

        return StatefulBuilder(
          builder: (context, setState) {
            return Padding(
              padding: EdgeInsets.only(
                bottom: MediaQuery.of(bottomSheetContext).viewInsets.bottom,
              ),
              child: Container(
                padding: const EdgeInsets.all(20),
                decoration: const BoxDecoration(
                  color: Colors.transparent,
                  borderRadius: BorderRadius.vertical(top: Radius.circular(12)),
                ),
                child: MechanixTextInputTheme(
                  style: MechanixTextInputThemeData(
                    borderRadius: BorderRadius.circular(50),
                  ),
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Text(
                        'Compress to...',
                        style: TextStyle(
                          fontSize: 16,
                          color: Colors.grey,
                        ),
                      ).padBottom(12),
                      Row(
                        children: [
                          Expanded(
                            flex: 3,
                            child: MechanixTextInput<String>.textInput(
                              hintText: defaultZipName,
                              initialValue: defaultZipName,
                              onChanged: (value) {
                                zipName = value;
                                setState(() => errorText = null);
                              },
                              inputDecoration: InputDecoration(
                                contentPadding: const EdgeInsets.symmetric(
                                    horizontal: 16, vertical: 16),
                                filled: true,
                                fillColor: const Color(0xFF2C2C2E),
                                border: OutlineInputBorder(
                                  borderRadius: BorderRadius.circular(50),
                                  borderSide: BorderSide.none,
                                ),
                                errorText: errorText,
                                suffixIcon: Row(
                                  mainAxisSize: MainAxisSize.min,
                                  children: [
                                    OutlinedButton(
                                      style: OutlinedButton.styleFrom(
                                        padding: const EdgeInsets.symmetric(
                                            vertical: 16),
                                      ).copyWith(
                                        splashFactory: NoSplash.splashFactory,
                                      ),
                                      onPressed: () =>
                                          Navigator.pop(bottomSheetContext),
                                      child: const Icon(
                                        Icons.close,
                                        color: Colors.white,
                                      ),
                                    ),
                                    const SizedBox(width: 6),
                                    ElevatedButton(
                                      style: ElevatedButton.styleFrom(
                                        padding: const EdgeInsets.symmetric(
                                            vertical: 16),
                                        backgroundColor: Colors.grey[800],
                                        foregroundColor: Colors.white,
                                        shape: RoundedRectangleBorder(
                                          borderRadius:
                                              BorderRadius.circular(50),
                                        ),
                                      ).copyWith(
                                        splashFactory: NoSplash.splashFactory,
                                      ),
                                      onPressed: () async {
                                        final trimmedName = zipName.trim();

                                        if (trimmedName.isEmpty) {
                                          setState(() {
                                            errorText =
                                                'ZIP file name cannot be empty';
                                          });
                                          return;
                                        }

                                        final zipPath = p.join(
                                          destinationDirPath,
                                          trimmedName.endsWith('.zip')
                                              ? trimmedName
                                              : '$trimmedName.zip',
                                        );

                                        // Async existence check
                                        final fileExists =
                                            await File(zipPath).exists();
                                        if (fileExists) {
                                          setState(() {
                                            errorText =
                                                'A ZIP file with that name already exists';
                                          });
                                          return;
                                        }

                                        // Safe to compress
                                        filesBloc.add(
                                          CompressEntitiesEvent(
                                            sourcePaths: selectedPaths.toList(),
                                            destinationZipPath: zipPath,
                                            controller: controller,
                                          ),
                                        );

                                        Navigator.pop(bottomSheetContext);
                                        clearSelection();
                                      },
                                      child: const Icon(Icons.check),
                                    ),
                                    const SizedBox(width: 6),
                                  ],
                                ),
                              ),
                            ),
                          ),
                        ],
                      ),
                    ],
                  ),
                ),
              ),
            );
          },
        );
      },
    );
  }

  void handleExtract(String zipFilePath) {
    final filesBloc = BlocProvider.of<FilesBloc>(context);

    // Start extract mode with the tapped file
    filesBloc.add(StartExtractMode(zipFilePath));

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
                'Extract to...',
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
                      onTap: () => onItemTap(
                        context,
                        homeDir,
                        "Home",
                        filesBloc,
                        () => reload(),
                      ),
                      leading: const IconWidget(
                        iconWidth: 20,
                        iconHeight: 20,
                        iconPath: Images.home,
                        iconColor: Colors.blueAccent,
                      ),
                      defaultTrailingIcon: false,
                      trailing: _trailingIcon(),
                    ),
                    SectionListItems(
                      title: "Downloads",
                      titleTextStyle: const TextStyle(fontSize: 14),
                      onTap: () => onItemTap(
                        context,
                        downloadsDir,
                        "Downloads",
                        filesBloc,
                        () => reload(),
                      ),
                      leading: const IconWidget(
                        iconWidth: 20,
                        iconHeight: 20,
                        iconPath: Images.downloads,
                        iconColor: Colors.deepPurpleAccent,
                      ),
                      defaultTrailingIcon: false,
                      trailing: _trailingIcon(),
                    ),
                    SectionListItems(
                      title: "Documents",
                      titleTextStyle: const TextStyle(fontSize: 14),
                      onTap: () => onItemTap(
                        context,
                        documentsDir,
                        "Documents",
                        filesBloc,
                        () => reload(),
                      ),
                      leading: const IconWidget(
                        iconWidth: 20,
                        iconHeight: 20,
                        iconPath: Images.homeDocuments,
                        iconColor: Colors.orangeAccent,
                      ),
                      defaultTrailingIcon: false,
                      trailing: _trailingIcon(),
                    ),
                    SectionListItems(
                      title: "Root (/)",
                      titleTextStyle: const TextStyle(fontSize: 14),
                      onTap: () => onItemTap(
                        context,
                        "/",
                        "Root",
                        filesBloc,
                        () => reload(),
                      ),
                      leading: const IconWidget(
                        iconWidth: 20,
                        iconHeight: 20,
                        iconPath: Images.hardDrive,
                      ),
                      defaultTrailingIcon: false,
                      trailing: _trailingIcon(),
                    ),
                  ],
                ),
              ),
            ],
          ),
        );
      },
    ).whenComplete(() {
      // Only clear selection if needed when bottom sheet closes
      clearSelection();
    });
  }

  Widget _trailingIcon() {
    return SizedBox(
      child: const Icon(
        size: 16,
        Icons.arrow_forward_ios,
        color: Colors.grey,
      ).padAll(4),
    );
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

  /// Opens a terminal window in the given path.
  /// Tries multiple terminal emulators (gnome-terminal, konsole, xfce4-terminal, xterm).
  Future<void> openInTerminal(String path) async {
    final candidates = [
      ['alacritty', '--working-directory', path],
      ['gnome-terminal', '--working-directory', path],
      ['konsole', '--workdir', path],
      ['xfce4-terminal', '--working-directory', path],
      ['xterm', '-e', 'cd $path; bash'],
    ];

    for (final cmd in candidates) {
      final executable = cmd.first;
      if (await Process.run('which', [executable])
          .then((p) => p.exitCode == 0)) {
        Process.start(executable, cmd.skip(1).toList());
        return;
      }
    }

    throw Exception("No terminal found");
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

  void handleSortMode(String value) {
    BlocProvider.of<FilesBloc>(context).add(SortFiles(value));
  }

  void _showDetailsDialog(BuildContext context, String path) {
    final bloc = context.read<FilesBloc>();
    bloc.add(FetchFileDetails(path));

    final fileItem = FileItem(
      name: p.basename(path),
      type: p.extension(path) == '' ? 'dir' : p.extension(path),
      modified: DateTime.now(),
    );
    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.transparent,
      isScrollControlled: true,
      builder: (bottomSheetContext) {
        return BlocProvider.value(
          value: bloc,
          child: BlocBuilder<FilesBloc, FilesState>(
            builder: (context, state) {
              final details = state.fileDetails;
              final hidden = p.basename(path).startsWith('.') ? 'Yes' : 'No';
              var readable = '-';
              var writable = '-';

              if (details != null) {
                final mode = details.mode;
                readable = (mode & 0x100) != 0 ? 'Yes' : 'No';
                writable = (mode & 0x80) != 0 ? 'Yes' : 'No';
              }

              if (details == null) {
                return Container(
                  padding: const EdgeInsets.all(16),
                  decoration: BoxDecoration(
                    color: Colors.grey[800],
                    borderRadius:
                        const BorderRadius.vertical(top: Radius.circular(12)),
                  ),
                  child: const SizedBox(
                    height: 100,
                    child: Center(child: CircularProgressIndicator()),
                  ),
                );
              }

              final items = [
                buildDetailRow("Type", details.type.toString()),
                buildDetailRow("Size", formatBytes(details.size)),
                buildDetailRow("Modified", formatDateTime(details.modified)),
                buildDetailRow("Accessed", formatDateTime(details.accessed)),
                buildDetailRow("Changed", formatDateTime(details.changed)),
                buildDetailRow("Readable", readable),
                buildDetailRow("Writable", writable),
                buildDetailRow("Hidden", hidden),
              ];

              return Container(
                padding:
                    const EdgeInsets.symmetric(horizontal: 16, vertical: 14),
                decoration: BoxDecoration(
                  color: Colors.grey[800],
                  borderRadius:
                      const BorderRadius.vertical(top: Radius.circular(12)),
                ),
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(children: [
                      Image.asset(fileItem.iconPath, width: 28, height: 28),
                      const SizedBox(width: 12),
                      Expanded(
                        child: Text(
                          fileItem.name,
                          style: const TextStyle(
                            fontWeight: FontWeight.bold,
                            fontSize: 20,
                            color: Colors.white,
                          ),
                          overflow: TextOverflow.ellipsis,
                          maxLines: 1,
                        ),
                      )
                    ]),
                    const SizedBox(height: 12),
                    MechanixSimpleListTheme(
                      style: const MechanixSimpleListThemeData(
                        itemPadding: EdgeInsets.zero,
                        backgroundColor: Colors.transparent,
                        widgetMargin: EdgeInsets.only(bottom: 8),
                      ),
                      child: MechanixSimpleList.builder(
                          isDividerRequired: false,
                          padding: EdgeInsets.zero,
                          itemCount: items.length,
                          itemBuilder: (context, index) {
                            return items[index];
                          }),
                    ),
                    SizedBox(
                      width: double.infinity,
                      child: ElevatedButton(
                        style: ElevatedButton.styleFrom(
                          backgroundColor: Colors.grey[700],
                          foregroundColor: Colors.white,
                          shape: RoundedRectangleBorder(
                            borderRadius: BorderRadius.circular(50),
                          ),
                        ).copyWith(
                          splashFactory: NoSplash
                              .splashFactory, // Disable ripple animation
                        ),
                        onPressed: () => Navigator.of(bottomSheetContext).pop(),
                        child: const Text("Close"),
                      ),
                    ),
                  ],
                ),
              );
            },
          ),
        );
      },
    );
  }

  Widget buildDetailRow(String title, String value) {
    return Container(
      padding: const EdgeInsets.symmetric(vertical: 4),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Text(title,
              style: const TextStyle(color: Colors.white, fontSize: 14)),
          Text(value,
              style: const TextStyle(color: Colors.white70, fontSize: 12)),
        ],
      ),
    );
  }

  void showCreateFolderDialogFloatingMenu() {
    _closeFabMenuProgrammatically();
    String folderName = ""; // Local variable to track input
    final filesBloc = BlocProvider.of<FilesBloc>(context);
    final hasSelection = selectedPaths.isNotEmpty;

    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.transparent,
      isScrollControlled: true,
      builder: (bottomSheetContext) {
        String? errorText;

        return StatefulBuilder(
          builder: (context, setState) {
            return Padding(
              padding: EdgeInsets.only(
                bottom: MediaQuery.of(bottomSheetContext).viewInsets.bottom,
              ),
              child: Container(
                padding: const EdgeInsets.all(20),
                decoration: const BoxDecoration(
                  color: Colors.transparent,
                  borderRadius: BorderRadius.vertical(top: Radius.circular(12)),
                ),
                child: MechanixTextInputTheme(
                  style: MechanixTextInputThemeData(
                    borderRadius: BorderRadius.circular(50),
                  ),
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Row(
                        children: [
                          Expanded(
                            flex: 3,
                            child: MechanixTextInput<String>.textInput(
                              hintText: "New folder",
                              onChanged: (value) {
                                folderName = value;
                                setState(() => errorText = null); // clear error
                              },
                              inputDecoration: InputDecoration(
                                contentPadding: const EdgeInsets.symmetric(
                                    horizontal: 16, vertical: 16),
                                filled: true,
                                fillColor: const Color(0xFF2C2C2E),
                                border: OutlineInputBorder(
                                  borderRadius: BorderRadius.circular(50),
                                  borderSide: BorderSide.none,
                                ),
                                errorText: errorText, // show error text
                                suffixIcon: Row(
                                  mainAxisSize: MainAxisSize.min,
                                  children: [
                                    OutlinedButton(
                                      style: OutlinedButton.styleFrom(
                                        padding: const EdgeInsets.symmetric(
                                            vertical: 16),
                                      ).copyWith(
                                        splashFactory: NoSplash.splashFactory,
                                      ),
                                      onPressed: () {
                                        Navigator.of(bottomSheetContext).pop();
                                      },
                                      child: const Icon(
                                        Icons.close,
                                        color: Colors.white,
                                      ),
                                    ),
                                    const SizedBox(width: 6),
                                    ElevatedButton(
                                      style: ElevatedButton.styleFrom(
                                        padding: const EdgeInsets.symmetric(
                                            vertical: 16),
                                        backgroundColor: Colors.grey[800],
                                        foregroundColor: Colors.white,
                                        shape: RoundedRectangleBorder(
                                          borderRadius:
                                              BorderRadius.circular(50),
                                        ),
                                      ).copyWith(
                                        splashFactory: NoSplash.splashFactory,
                                      ),
                                      onPressed: () async {
                                        final trimmedName = folderName.trim();

                                        if (trimmedName.isEmpty) {
                                          setState(() => errorText =
                                              'Folder name cannot be empty');
                                          return;
                                        }

                                        final currentPath =
                                            controller.getPathNotifier.value;
                                        final newFolderPath =
                                            '$currentPath/$trimmedName';

                                        // async folder existence check
                                        final exists =
                                            await Directory(newFolderPath)
                                                .exists();
                                        if (exists) {
                                          setState(() => errorText =
                                              'A folder with that name already exists');
                                          return;
                                        }

                                        // safe to create
                                        filesBloc.add(CreateFolder(
                                          path: currentPath,
                                          folderName: trimmedName,
                                          controller: controller,
                                        ));

                                        // move selected files into the new folder
                                        if (hasSelection) {
                                          final newFolderPath =
                                              '$currentPath/$trimmedName';
                                          final completer = Completer<void>();

                                          filesBloc.add(Move(
                                            sourcePaths: selectedPaths.toList(),
                                            destinationPath: newFolderPath,
                                            completer: completer,
                                          ));
                                          await completer.future;
                                          filesBloc.add(CancelMoveMode());
                                          reload();
                                          clearSelection();
                                        }

                                        Navigator.of(bottomSheetContext).pop();
                                      },
                                      child: const Icon(Icons.check),
                                    ),
                                    const SizedBox(width: 6),
                                  ],
                                ),
                              ),
                            ),
                          ),
                        ],
                      ),
                    ],
                  ),
                ),
              ),
            );
          },
        );
      },
    );
  }

  void showCreateFolderDialog() {
    String folderName = ""; // Track input
    final filesBloc = BlocProvider.of<FilesBloc>(context);
    final currentPath = controller.getPathNotifier.value;

    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.transparent,
      isScrollControlled: true,
      builder: (bottomSheetContext) {
        String? errorText;

        return StatefulBuilder(
          builder: (context, setState) {
            return Padding(
              padding: EdgeInsets.only(
                bottom: MediaQuery.of(bottomSheetContext).viewInsets.bottom,
              ),
              child: Container(
                padding: const EdgeInsets.all(20),
                decoration: const BoxDecoration(
                  color: Colors.transparent,
                  borderRadius: BorderRadius.vertical(top: Radius.circular(12)),
                ),
                child: MechanixTextInputTheme(
                  style: MechanixTextInputThemeData(
                    borderRadius: BorderRadius.circular(50),
                  ),
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      MechanixTextInput<String>.textInput(
                        hintText: "New folder",
                        onChanged: (value) {
                          folderName = value;
                          setState(() {
                            errorText = null; // clear previous error
                          });
                        },
                        inputDecoration: InputDecoration(
                          contentPadding: const EdgeInsets.symmetric(
                              horizontal: 16, vertical: 16),
                          filled: true,
                          fillColor: const Color(0xFF2C2C2E),
                          border: OutlineInputBorder(
                            borderRadius: BorderRadius.circular(50),
                            borderSide: BorderSide.none,
                          ),
                          errorText: errorText, // shows validation error
                          suffixIcon: Row(
                            mainAxisSize: MainAxisSize.min,
                            children: [
                              OutlinedButton(
                                style: OutlinedButton.styleFrom(
                                        padding: const EdgeInsets.symmetric(
                                            vertical: 16))
                                    .copyWith(
                                  splashFactory: NoSplash.splashFactory,
                                ),
                                onPressed: () {
                                  Navigator.of(bottomSheetContext).pop();
                                },
                                child: const Icon(
                                  Icons.close,
                                  color: Colors.white,
                                ),
                              ),
                              const SizedBox(width: 6),
                              ElevatedButton(
                                style: ElevatedButton.styleFrom(
                                  padding:
                                      const EdgeInsets.symmetric(vertical: 16),
                                  backgroundColor: Colors.grey[800],
                                  foregroundColor: Colors.white,
                                  shape: RoundedRectangleBorder(
                                    borderRadius: BorderRadius.circular(50),
                                  ),
                                ).copyWith(
                                  splashFactory: NoSplash.splashFactory,
                                ),
                                onPressed: () async {
                                  final trimmedName = folderName.trim();

                                  if (trimmedName.isEmpty) {
                                    setState(() {
                                      errorText = 'Folder name cannot be empty';
                                    });
                                    return;
                                  }

                                  final newFolderPath =
                                      '$currentPath/$trimmedName';

                                  // Async folder existence check
                                  final exists =
                                      await Directory(newFolderPath).exists();

                                  if (exists) {
                                    setState(() {
                                      errorText =
                                          'A folder with that name already exists';
                                    });
                                    return;
                                  }

                                  // Folder does not exist, safe to create
                                  filesBloc.add(CreateFolder(
                                    path: currentPath,
                                    folderName: trimmedName,
                                    controller: controller,
                                  ));

                                  Navigator.of(bottomSheetContext).pop();
                                },
                                child: const Icon(Icons.check),
                              ),
                              const SizedBox(width: 6),
                            ],
                          ),
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            );
          },
        );
      },
    );
  }

  void _showRenameDialog(String oldPath) {
    String newName = p.basename(oldPath);
    final filesBloc = BlocProvider.of<FilesBloc>(context);

    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.black,
      isScrollControlled: true,
      builder: (bottomSheetContext) {
        return StatefulBuilder(
          builder: (context, setState) {
            final trimmedName = newName.trim();
            final isSubmitDisabled =
                trimmedName.isEmpty || trimmedName == p.basename(oldPath);

            return Padding(
              padding: EdgeInsets.only(
                bottom: MediaQuery.of(bottomSheetContext).viewInsets.bottom,
              ),
              child: Container(
                padding: const EdgeInsets.all(20),
                decoration: const BoxDecoration(
                  color: Colors.transparent,
                  borderRadius: BorderRadius.vertical(top: Radius.circular(12)),
                ),
                child: MechanixTextInputTheme(
                  style: MechanixTextInputThemeData(
                    borderRadius: BorderRadius.circular(50),
                  ),
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        'Rename \'${p.basename(oldPath)}\'',
                        style: const TextStyle(
                          fontSize: 16,
                          color: Colors.grey,
                        ),
                      ).padBottom(12),

                      // Input field
                      Row(
                        children: [
                          Expanded(
                            flex: 3,
                            child: MechanixTextInput<String>.textInput(
                              initialValue: p.basename(oldPath),
                              onChanged: (value) {
                                setState(() {
                                  newName = value;
                                });
                              },
                              inputDecoration: InputDecoration(
                                contentPadding: const EdgeInsets.symmetric(
                                    horizontal: 16, vertical: 16),
                                filled: true,
                                fillColor: const Color(0xFF2C2C2E),
                                border: OutlineInputBorder(
                                  borderRadius: BorderRadius.circular(50),
                                  borderSide: BorderSide.none,
                                ),
                                suffixIcon: Row(
                                  mainAxisSize: MainAxisSize.min,
                                  children: [
                                    OutlinedButton(
                                      style: OutlinedButton.styleFrom(
                                        padding: const EdgeInsets.symmetric(
                                            vertical: 16),
                                      ).copyWith(
                                        splashFactory: NoSplash.splashFactory,
                                      ),
                                      onPressed: () =>
                                          Navigator.of(bottomSheetContext)
                                              .pop(),
                                      child: const Icon(
                                        Icons.close,
                                        color: Colors.white,
                                      ),
                                    ),
                                    const SizedBox(width: 6),
                                    ElevatedButton(
                                      style: ElevatedButton.styleFrom(
                                        padding: const EdgeInsets.symmetric(
                                            vertical: 16),
                                        backgroundColor: isSubmitDisabled
                                            ? Colors.grey[700]
                                            : Colors.grey[800],
                                        foregroundColor: Colors.white,
                                        shape: RoundedRectangleBorder(
                                          borderRadius:
                                              BorderRadius.circular(50),
                                        ),
                                      ).copyWith(
                                        splashFactory: NoSplash.splashFactory,
                                      ),
                                      onPressed: isSubmitDisabled
                                          ? null
                                          : () {
                                              filesBloc.add(Rename(
                                                oldPath: oldPath,
                                                newName: trimmedName,
                                                controller: controller,
                                              ));
                                              Navigator.of(bottomSheetContext)
                                                  .pop();
                                              clearSelection();
                                            },
                                      child: const Icon(Icons.check),
                                    ),
                                    const SizedBox(width: 6),
                                  ],
                                ),
                              ),
                            ),
                          ),
                        ],
                      ),
                    ],
                  ),
                ),
              ),
            );
          },
        );
      },
    );

    clearSelection();
  }

  void _toggleHiddenFiles() {
    controller.toggleShowHiddenFiles();
    final filesBloc = BlocProvider.of<FilesBloc>(context);
    filesBloc.add(ToggleHiddenFiles());
  }

  void reload() {
    controller.reload();
  }

  Future<void> copyPath(String path) async {
    await Clipboard.setData(ClipboardData(text: path));
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: const Text('Path copied to clipboard',
            style: TextStyle(color: Colors.white)),
        duration: const Duration(seconds: 2),
        backgroundColor: Colors.grey[800],
      ),
    );
  }

  void _confirmDelete(BuildContext context, Set<String> selectedPaths) {
    final pathsToDelete = selectedPaths.toList();

    final isSingle = pathsToDelete.length == 1;
    final title = isSingle
        ? "Delete '${pathsToDelete.first.split('/').last}'"
        : "Delete ${pathsToDelete.length} selected files";
    final message = isSingle
        ? "This action will delete the file permanently"
        : "This action will delete the files permanently";

    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.transparent,
      isScrollControlled: true,
      builder: (bottomSheetContext) => Container(
        padding: const EdgeInsets.all(16),
        decoration: BoxDecoration(
          color: Colors.grey[850],
          borderRadius: const BorderRadius.vertical(top: Radius.circular(12)),
        ),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              title,
              style: const TextStyle(
                fontWeight: FontWeight.w500,
                fontSize: 18,
                color: Colors.white,
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
                  child: MechanixOutlinedButton(
                    label: "Cancel",
                    textColor: Colors.white,
                    borderRadius: 50,
                    borderWidth: 0.5,
                    onPressed: () => Navigator.pop(bottomSheetContext),
                  ),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: MechanixElevatedButton(
                    label: "Delete",
                    backgroundColor: Colors.red,
                    textColor: Colors.white,
                    borderRadius: 50,
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
  }
}

List<FileItem> getFilesAtPath(
    List<FileItem> path, List<FileSystemEntity> fileSystemList) {
  // Build full path from root and path list
  String currentPath = '/';
  for (final item in path) {
    currentPath = p.join(currentPath, item.name);
  }

  final List<FileItem> items = [];

  try {
    for (final entity in fileSystemList) {
      final String name = p.basename(entity.path);
      if (name.isEmpty) continue;

      final stat = entity.statSync();
      final modifiedTime = stat.modified;

      if (entity is Directory) {
        items.add(FileItem(name: name, type: 'dir', modified: modifiedTime));
      } else if (entity is File) {
        final ext = p.extension(name);
        items.add(FileItem(
            name: name,
            type: ext.isNotEmpty ? ext : 'file',
            modified: modifiedTime));
      }
    }
  } catch (e) {
    print('Error reading directory at $currentPath: $e');
  }

  return items;
}

String formatBytes(int bytes, [int decimals = 2]) {
  if (bytes <= 0) return "0 B";
  const suffixes = ["B", "KB", "MB", "GB", "TB"];
  final i = (bytes == 0) ? 0 : (Math.log(bytes) / Math.log(1024)).floor();
  final size = bytes / Math.pow(1024, i);
  return "${size.toStringAsFixed(decimals)} ${suffixes[i]}";
}
