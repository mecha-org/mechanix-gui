import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/src/constants/constants.dart';
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
  Timer? _debounceTimer;

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

  void _onSearchChanged() {
    // Cancel any running timer
    _debounceTimer?.cancel();

    // Start a new timer
    _debounceTimer = Timer(Constants.debounceDuration, () {
      context.read<NotesBloc>().add(SearchEvent(_titleController.text));
    });
  }

  void clearSearch() {
    _titleController.clear();
    _onSearchChanged();
  }

  @override
  void dispose() {
    _debounceTimer?.cancel();
    _titleController.dispose();
    _focusNode.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 56,
      width: 508,
      child: MechanixSearchBar(
        controller: _titleController,
        onChanged: (_) => _onSearchChanged(),
        autoFocus: false,
        hintText: "Search notes...",
        onCloseIconPress: clearSearch,
      ),
    );
  }
}
