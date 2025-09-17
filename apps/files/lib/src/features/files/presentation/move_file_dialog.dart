import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/app_config.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/blocs/file_state.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:mechanix_files/src/features/files/presentation/files_home.dart';
import 'package:mechanix_files/src/features/files/presentation/list_view.dart';
import 'package:widgets/extension.dart';
import 'package:widgets/widgets/icon_widget.dart';
import 'package:widgets/widgets/navigation_bar/mechanix_navigation_bar.dart';
import 'package:widgets/widgets/sectionList/mechanix_section_list.dart';
import 'package:widgets/widgets/sectionList/mechanix_section_list_theme.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';
import 'package:widgets/widgets/textInput/mechanix_text_input.dart';
import 'package:widgets/widgets/textInput/mechanix_text_input_theme.dart';

import 'files.dart';

class MoveExplorerBottomSheet extends StatefulWidget {
  final String path;
  final String title;
  final FilesBloc filesBloc;
  final FilesBloc filesBlocMainContext;
  final VoidCallback onMoveCompleted;

  const MoveExplorerBottomSheet({
    super.key,
    required this.path,
    required this.title,
    required this.filesBloc,
    required this.filesBlocMainContext,
    required this.onMoveCompleted,
  });

  @override
  State<MoveExplorerBottomSheet> createState() =>
      _MoveExplorerBottomSheetState();
}

class _MoveExplorerBottomSheetState extends State<MoveExplorerBottomSheet> {
  late List<FileItem> currentPath;
  bool isSearching = false;
  final TextEditingController searchController = TextEditingController();
  final FocusNode searchFocusNode = FocusNode();

  final downloadsDir = AppConfig().downloadsDir;
  final documentsDir = AppConfig().documentsDir;
  final homeDir = AppConfig().homeDir;

  List<FileItem> searchResults = [];

  void _loadFiles() {
    final pathString = '/${currentPath.map((e) => e.name).join('/')}';
    widget.filesBloc.add(LoadFilesAtPath(pathString));
  }

  @override
  void initState() {
    super.initState();
    currentPath = pathToSegments(widget.path);
    _loadFiles();
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<FilesBloc, FilesState>(
      bloc: widget.filesBloc,
      builder: (context, state) {
        final displayedFiles =
            getFilesAtPath(currentPath, state.fileSystemList);

        return DraggableScrollableSheet(
          expand: false,
          initialChildSize: 0.7,
          minChildSize: 0.4,
          maxChildSize: 0.9,
          builder: (context, scrollController) {
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
                        if (currentPath.isNotEmpty) {
                          setState(() {
                            currentPath.removeLast();
                          });
                          _loadFiles();
                        } else {
                          Navigator.pop(context);
                          moveMainBottomSheet(widget.onMoveCompleted);
                        }
                      },
                    ),
                    title: currentPath.isEmpty ? "Root" : currentPath.last.name,
                    titleStyle: const TextStyle(
                      color: Colors.white,
                      fontSize: 18,
                    ),
                    actionWidgets: [
                      // TODO : implement search functionality
                      // if (!isSearching)
                      //   IconButton(
                      //     icon: const Icon(Icons.search, color: Colors.white),
                      //     onPressed: () {
                      //       setState(() {
                      //         isSearching = true;
                      //         searchFocusNode.requestFocus();
                      //       });
                      //     },
                      //   ),
                    ],
                  ).padTop(8),
                  Expanded(
                    child: buildListViewMove(displayedFiles, context,
                        currentPath, widget.filesBloc, widget.onMoveCompleted),
                  ),
                  if (isSearching)
                    Padding(
                      padding: const EdgeInsets.symmetric(
                          horizontal: 12, vertical: 12),
                      child: RawKeyboardListener(
                        focusNode: searchFocusNode,
                        onKey: (event) {
                          if (event is RawKeyDownEvent &&
                              event.logicalKey == LogicalKeyboardKey.enter) {
                            _performSearch(); // Call search logic
                          }
                        },
                        child: MechanixTextInputTheme(
                          style: MechanixTextInputThemeData(
                            borderRadius: BorderRadius.circular(50),
                          ),
                          child: MechanixTextInput.textInput(
                            onChanged: (value) {
                              // Live update logic (optional)
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
                              hintText: "Search files",
                              hintStyle: const TextStyle(color: Colors.white54),
                              prefixIcon: const Padding(
                                padding: EdgeInsets.only(left: 12, right: 8),
                                child:
                                    Icon(Icons.search, color: Colors.white54),
                              ),
                              prefixIconConstraints: const BoxConstraints(
                                minWidth: 40,
                                minHeight: 40,
                              ),
                              suffixIcon: Row(
                                mainAxisSize: MainAxisSize.min,
                                children: [
                                  OutlinedButton(
                                    style: OutlinedButton.styleFrom(
                                      padding: const EdgeInsets.symmetric(
                                          vertical: 14),
                                      side: BorderSide(
                                          color: Colors.white70, width: 0.4),
                                      backgroundColor: Colors.transparent,
                                    ).copyWith(
                                      splashFactory: NoSplash
                                          .splashFactory, // Disable ripple
                                    ),
                                    child: const Icon(Icons.close,
                                        color: Colors.white54),
                                    onPressed: () {
                                      searchController.clear();
                                      Navigator.pop(context, true);
                                    },
                                  ),
                                  Container(
                                    width: 1,
                                    height: 28,
                                    color: Colors.white24,
                                    margin: const EdgeInsets.symmetric(
                                        horizontal: 8),
                                  ),
                                  ElevatedButton(
                                          style: ElevatedButton.styleFrom(
                                            backgroundColor: Colors.grey[800],
                                          ).copyWith(
                                            splashFactory: NoSplash
                                                .splashFactory, // Disable ripple animation
                                          ),
                                          onPressed: () {
                                            // Confirm action
                                            _performSearch();
                                            // Navigator.pop(context, true);
                                          },
                                          child: Icon(Icons.check,
                                              color: Colors.white70))
                                      .padRight(8),
                                ],
                              ),
                              suffixIconConstraints: const BoxConstraints(
                                minWidth: 80,
                                minHeight: 40,
                              ),
                            ),
                          ),
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
                                side: BorderSide(
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
                                handlePaste(context, widget.filesBloc.state);

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

  void _performSearch() {
    final query = searchController.text.trim();
    if (query.isEmpty) return;

    setState(() {
      searchResults = getFilesAtPath(
              currentPath, widget.filesBloc.state.fileSystemList)
          .where(
              (file) => file.name.toLowerCase().contains(query.toLowerCase()))
          .toList();
    });
  }

  void moveMainBottomSheet(onMoveCompleted) {
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
              Text(
                'Select destination',
                style: const TextStyle(
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
                        onTap: () => onTap(context, homeDir, "Home", filesBloc,
                            onMoveCompleted),
                        leading: IconWidget(
                          iconWidth: 20,
                          iconHeight: 20,
                          iconPath: Images.home,
                          iconColor: Colors.blueAccent,
                        ),
                        defaultTrailingIcon: false,
                        trailing: SizedBox(
                          child: Icon(
                            size: 16,
                            Icons.arrow_forward_ios,
                            color: Colors.grey,
                          ).padAll(4),
                        )),
                    SectionListItems(
                        title: "Downloads",
                        titleTextStyle: const TextStyle(fontSize: 14),
                        onTap: () => onTap(context, downloadsDir, "Downloads",
                            filesBloc, onMoveCompleted),
                        leading: IconWidget(
                          iconWidth: 20,
                          iconHeight: 20,
                          iconPath: Images.downloads,
                          iconColor: Colors.deepPurpleAccent,
                        ),
                        defaultTrailingIcon: false,
                        trailing: SizedBox(
                          child: Icon(
                            size: 16,
                            Icons.arrow_forward_ios,
                            color: Colors.grey,
                          ).padAll(4),
                        )),
                    SectionListItems(
                        title: "Documents",
                        titleTextStyle: const TextStyle(fontSize: 14),
                        onTap: () => onTap(context, documentsDir, "Documents",
                            filesBloc, onMoveCompleted),
                        leading: IconWidget(
                          iconWidth: 20,
                          iconHeight: 20,
                          iconPath: Images.homeDocuments,
                          iconColor: Colors.orangeAccent,
                        ),
                        defaultTrailingIcon: false,
                        trailing: SizedBox(
                          child: Icon(
                            size: 16,
                            Icons.arrow_forward_ios,
                            color: Colors.grey,
                          ).padAll(4),
                        )),
                    SectionListItems(
                        title: "Root (/)",
                        titleTextStyle: const TextStyle(fontSize: 14),
                        onTap: () => onTap(
                            context, "/", "Root", filesBloc, onMoveCompleted),
                        leading: IconWidget(
                          iconWidth: 20,
                          iconHeight: 20,
                          iconPath: Images.hardDrive,
                        ),
                        defaultTrailingIcon: false,
                        trailing: SizedBox(
                          child: Icon(
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

  Future<void> handlePaste(BuildContext context, FilesState state) async {
    final targetPath = '/${currentPath.map((e) => e.name).join('/')}';
    final bloc = BlocProvider.of<FilesBloc>(context);

    // Show SnackBar
    final movedCount = state.movedPaths.length;
    final folderName = targetPath.split('/').last;

    await bloc.fileRepository
        .moveEntities(state.movedPaths, targetPath); // wait for move
    bloc.add(CancelMoveMode());

    // safe to reload
    widget.onMoveCompleted();

    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(
            "Moved $movedCount item${movedCount > 1 ? 's' : ''} to '$folderName'",
            style: TextStyle(color: Colors.white)),
        duration: const Duration(seconds: 5),
        backgroundColor: Colors.grey[800],
      ),
    );
  }
}

void onTap(
  BuildContext context,
  String path,
  String title,
  FilesBloc filesBlocMain,
  VoidCallback onMoveCompleted,
) {
  final localBloc = FilesBloc(
    fileRepository: filesBlocMain.fileRepository,
    recentFilesManager: filesBlocMain.recentFilesManager,
  );

// Clone the state
  localBloc.emit(
    localBloc.state.copyWith(
      movedPaths: List<String>.from(filesBlocMain.state.movedPaths),
    ),
  );
  // Close any existing bottom sheet

  showModalBottomSheet(
    context: context,
    isScrollControlled: true,
    backgroundColor: Colors.transparent,
    shape: const RoundedRectangleBorder(
      borderRadius: BorderRadius.vertical(top: Radius.circular(16)),
    ),
    builder: (_) => BlocProvider.value(
      value: localBloc,
      child: MoveExplorerBottomSheet(
        path: path,
        title: title,
        filesBloc: localBloc,
        filesBlocMainContext: filesBlocMain,
        onMoveCompleted: onMoveCompleted,
      ),
    ),
  );
}
