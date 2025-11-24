import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/src/constants/constants.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:widgets/widgets/search_bar/mechanix_search_bar.dart';

class SearchInputBar extends StatefulWidget {
  const SearchInputBar({super.key});

  @override
  State<SearchInputBar> createState() => _SearchInputBarState();
}

class _SearchInputBarState extends State<SearchInputBar> {
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

  void _onSearchChanged(String val) {
    // Cancel any running timer
    _debounceTimer?.cancel();

    // Start a new timer
    _debounceTimer = Timer(Constants.debounceDuration, () {
      context.read<NotesBloc>().add(SearchEvent(val));
    });
  }

  void clearSearch() {
    _onSearchChanged('');
  }

  @override
  void dispose() {
    _debounceTimer?.cancel();
    // _titleController.dispose();
    _focusNode.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 56,
      width: 508,
      child: MechanixSearchBar(
        focusNode: _focusNode,
        onChanged: (val) => _onSearchChanged(val),
        autoFocus: false,
        hintText: "Search notes...",
        onCloseIconPress: clearSearch,
      ),
    );
  }
}
