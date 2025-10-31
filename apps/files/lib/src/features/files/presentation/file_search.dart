import 'package:file/file.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/app_config.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/blocs/file_state.dart';
import 'package:mechanix_files/src/features/files/presentation/grid_view.dart';
import 'package:mechanix_files/src/features/files/presentation/list_view.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/navigation_bar/mechanix_navigation_bar_theme.dart';
import 'package:widgets/widgets/search_bar/mechanix_search_bar.dart';

class FileSearchPage extends StatefulWidget {
  const FileSearchPage({super.key});

  @override
  State<FileSearchPage> createState() => _FileSearchPageState();
}

class _FileSearchPageState extends State<FileSearchPage> {
  final FocusNode _searchFocusNode = FocusNode();
  final String homeDir = AppConfig().homeDir;
  final ValueNotifier<bool> viewModeNotifier = ValueNotifier(false);

  @override
  void initState() {
    super.initState();
    clearSearch(context);
    focusSearchField(); // default 300ms delay
  }

  @override
  void dispose() {
    viewModeNotifier.dispose();
    _searchFocusNode.dispose();
    super.dispose();
  }

  /// Requests focus on the search field after the page has fully built.
  /// Optional [delayMillis] can be used to adjust the delay before focusing.
  void focusSearchField({int delayMillis = 300}) {
    WidgetsBinding.instance.addPostFrameCallback((_) async {
      if (!mounted) return;

      // Wait for optional delay to ensure page transition is complete
      await Future.delayed(Duration(milliseconds: delayMillis));

      if (mounted) {
        FocusScope.of(context).requestFocus(_searchFocusNode);
      }
    });
  }

  void _performSearch(String query) {
    context.read<FilesBloc>().add(SearchFilesInDirectory(homeDir, query));
  }

  void clearSearch(BuildContext context) {
    context.read<FilesBloc>().add(ClearSearchResults());
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.black,
      appBar: MechanixNavigationBar(
        theme: const MechanixNavigationBarThemeData(titleSpacing: 0),
        title: "Search",
        actionWidgets: [
          IconButton(
            icon: ValueListenableBuilder<bool>(
              valueListenable: viewModeNotifier,
              builder: (context, isGrid, _) {
                return Image.asset(isGrid ? Images.list : Images.grid);
              },
            ),
            onPressed: () {
              viewModeNotifier.value = !viewModeNotifier.value;
            },
          ),
        ],
      ),
      body: BlocListener<FilesBloc, FilesState>(
        listenWhen: (previous, current) =>
            previous.error != current.error && current.error != null,
        listener: (context, state) {
          if (state.error != null && state.error!.isNotEmpty) {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: Text(state.error!),
                backgroundColor: Colors.redAccent,
              ),
            );
          }
        },
        child: Stack(
          children: [
            // BlocSelector for loading state
            Positioned.fill(
              child: BlocSelector<FilesBloc, FilesState, bool>(
                selector: (state) => state.loading,
                builder: (context, loading) {
                  if (loading) {
                    return const Center(child: CircularProgressIndicator());
                  }

                  // When not loading, show list selector below
                  return BlocSelector<FilesBloc, FilesState,
                      List<FileSystemEntity>>(
                    selector: (state) => state.fileSystemList,
                    builder: (context, fileList) {
                      if (fileList.isEmpty) {
                        return const Center(
                          child: Text(
                            "Find files and folders",
                            style: TextStyle(color: Colors.white70),
                          ),
                        );
                      }

                      return ValueListenableBuilder<bool>(
                        valueListenable: viewModeNotifier,
                        builder: (context, isGrid, _) {
                          return isGrid
                              ? buildSearchResultsGrid(fileList, context)
                              : buildSearchResultsList(fileList, context);
                        },
                      );
                    },
                  );
                },
              ),
            ),

            // Search bar (independent of bloc rebuilds)
            Positioned(
              left: 0,
              right: 0,
              bottom: 0,
              child: Padding(
                padding:
                    const EdgeInsets.symmetric(horizontal: 12, vertical: 12),
                child: SizedBox(
                  height: 56,
                  child: MechanixSearchBar(
                    focusNode: _searchFocusNode,
                    autoFocus: false,
                    hintText: "Search files",
                    onChanged: (text) {
                      if (text.isEmpty) {
                        clearSearch(context);
                      } else {
                        _performSearch(text);
                      }
                    },
                    showDefaultTrailing: true,
                  ),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
