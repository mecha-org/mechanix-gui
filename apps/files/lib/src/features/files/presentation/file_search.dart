import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_state.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:mechanix_files/src/features/files/presentation/files_home.dart';
import 'package:mechanix_files/src/features/files/presentation/list_view.dart';
import 'package:widgets/mechanix.dart';

class FileSearch extends SearchDelegate<String> {
  final List<FileItem> items;

  FileSearch(this.items);

  @override
  List<Widget> buildActions(BuildContext context) {
    return [
      if (query.isNotEmpty)
        IconButton(
          icon: const Icon(Icons.clear),
          onPressed: () => query = '',
        ),
    ];
  }

  @override
  Widget buildLeading(BuildContext context) {
    return IconButton(
      icon: const Icon(Icons.arrow_back),
      onPressed: () => close(context, ''),
    );
  }

  @override
  Widget buildResults(BuildContext context) {
    final results = items
        .where((item) => item.name.toLowerCase().contains(query.toLowerCase()))
        .toList();

    if (results.isEmpty) {
      return const Center(child: Text('No matching files found'));
    }

    return ListView.builder(
      itemCount: results.length,
      itemBuilder: (context, index) {
        final item = results[index];
        return ListTile(
          leading:
              Icon(item.type == 'dir' ? Icons.folder : Icons.insert_drive_file),
          title: Text(item.name),
          onTap: () {
            close(context, item.name);
            // You could also trigger navigation/open file here
          },
        );
      },
    );
  }

  @override
  Widget buildSuggestions(BuildContext context) {
    final suggestions = items
        .where((item) => item.name.toLowerCase().contains(query.toLowerCase()))
        .toList();

    return ListView.builder(
      itemCount: suggestions.length,
      itemBuilder: (context, index) {
        final item = suggestions[index];
        return ListTile(
          title: Text(item.name),
          onTap: () {
            query = item.name;
            showResults(context);
          },
        );
      },
    );
  }
}

class SearchResultsPage extends StatelessWidget {
  final String query;
  const SearchResultsPage({super.key, required this.query});

  @override
  Widget build(BuildContext context) {
    void homeNavigation() {
      final filesBloc = BlocProvider.of<FilesBloc>(context);

      Navigator.pushReplacement(
        context,
        MaterialPageRoute(
          builder: (_) => BlocProvider.value(
            value: filesBloc,
            child: FileHomePage(),
          ),
        ),
      );
    }

    return Scaffold(
      appBar: MechanixNavigationBar(
        leadingWidget: IconButton(
            icon:
                const Icon(Icons.arrow_back_ios, size: 20, color: Colors.blue),
            onPressed: homeNavigation),
        title: "Results for \"$query\"",
        titleStyle: context.textTheme.titleLarge,
      ),
      body: BlocBuilder<FilesBloc, FilesState>(
        builder: (context, state) {
          if (state.loading) {
            return const Center(child: CircularProgressIndicator());
          } else {
            final results = state.fileSystemList;
            if (results.isEmpty) {
              return const Center(child: Text("No files found"));
            }
            return buildSearchResultsList(state.fileSystemList, context);
          }
        },
      ),
    );
  }
}
