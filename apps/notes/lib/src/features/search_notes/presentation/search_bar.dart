import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:widgets/widgets/searchbar/mechanix_search_bar.dart';

class SearchInputBar extends StatefulWidget {
  const SearchInputBar({super.key});

  @override
  State<SearchInputBar> createState() => _SearchInputBarState();
}

class _SearchInputBarState extends State<SearchInputBar> {
  final TextEditingController _titleController = TextEditingController();
  final FocusNode _focusNode = FocusNode();

  @override
  void initState() {
    super.initState();
    _openKeyboardAfterLoad();
  }

  void _openKeyboardAfterLoad() {
    WidgetsBinding.instance.addPostFrameCallback((_) async {
      if (!mounted) return;

      await Future.delayed(const Duration(milliseconds: 300));
      _focusNode.requestFocus();
    });
  }

  void searchNotes() {
    context.read<NotesBloc>().add(SearchEvent(_titleController.text));
  }

  @override
  void dispose() {
    _titleController.dispose();
    _focusNode.dispose();
    super.dispose();
  }

  void clearSearch() {
    _titleController.clear();
    searchNotes();
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 56,
      width: 508,
      child: MechanixSearchBar(
        controller: _titleController,
        focusNode: _focusNode,
        onChanged: (_) => searchNotes(),
        autoFocus: false,
        hintText: "Search notes...",
        onCloseIconPress: () => clearSearch(),
      ),
    );
  }
}
