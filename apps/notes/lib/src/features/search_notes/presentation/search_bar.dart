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

  void searchNotes() {
    context.read<NotesBloc>().add(SearchEvent(_titleController.value.text));
  }

  @override
  void dispose() {
    _titleController.dispose();
    super.dispose();
  }

  void _handleBackwardPress() {
    final text = _titleController.text;
    if (text.isNotEmpty) {
      _titleController.text = text.substring(0, text.length - 1);
      _titleController.selection = TextSelection.fromPosition(
        TextPosition(offset: _titleController.text.length),
      );
      searchNotes();
    }
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 56,
      width: 508,
      child: MechanixSearchBar(
        controller: _titleController,
        onChanged: (value) => searchNotes(),
        autoFocus: true,
        hintText: "Type Here",
        onBackwardIconPress: _handleBackwardPress,
        onCloseIconPress: () => _titleController.clear(),
      ),
    );
  }
}
