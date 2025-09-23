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
import 'package:widgets/widgets/searchbar/mechanix_search_bar.dart';

class FileSearchPage extends StatefulWidget {
  const FileSearchPage({super.key});

  @override
  State<FileSearchPage> createState() => _FileSearchPageState();
}

class _FileSearchPageState extends State<FileSearchPage> {
  final TextEditingController _searchController = TextEditingController();
  final String homeDir = AppConfig().homeDir;
  final ValueNotifier<bool> viewModeNotifier = ValueNotifier(false);

  @override
  void initState() {
    super.initState();
    clearSearch(context);
  }

  @override
  void dispose() {
    _searchController.dispose();
    super.dispose();
  }

  void _performSearch(String query) {
    context.read<FilesBloc>().add(SearchFilesInDirectory(homeDir, query));
  }

  void clearSearch(BuildContext context) {
    final filesBloc = context.read<FilesBloc>();
    filesBloc.add(ClearSearchResults());
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<FilesBloc, FilesState>(
      builder: (context, state) {
        return Scaffold(
          backgroundColor: Colors.black,
          appBar: MechanixNavigationBar(
            title: "Search",
            titleStyle: const TextStyle(fontSize: 18),
            leadingWidget: IconButton(
              icon: const Icon(Icons.arrow_back_ios,
                  size: 20, color: Colors.blue),
              onPressed: () {
                Navigator.pop(context);
                // Optionally clear results when exiting
                clearSearch(context);
              },
            ),
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
          body: Stack(
            children: [
              Positioned.fill(
                child: state.loading
                    ? const Center(child: CircularProgressIndicator())
                    : state.fileSystemList.isEmpty
                        ? const Center(
                            child: Text("Find files and folders",
                                style: TextStyle(color: Colors.white70)))
                        : ValueListenableBuilder<bool>(
                            valueListenable: viewModeNotifier,
                            builder: (context, isGrid, _) {
                              return isGrid
                                  ? buildSearchResultsGrid(
                                      state.fileSystemList,
                                      context,
                                    )
                                  : buildSearchResultsList(
                                      state.fileSystemList,
                                      context,
                                    );
                            },
                          ),
              ),
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
                      controller: _searchController,
                      autoFocus: true,
                      hintText: "Search files",
                      onChanged: (text) {
                        if (text.isEmpty) {
                          clearSearch(context);
                        } else {
                          _performSearch(text);
                        }
                      },
                      onCloseIconPress: () {
                        _searchController.clear();
                        clearSearch(context);
                      },
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
}
