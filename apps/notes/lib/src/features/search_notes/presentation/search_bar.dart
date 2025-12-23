import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_notes/src/constants/constants.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_bloc.dart';
import 'package:mechanix_notes/src/features/home/bloc/notes_event.dart';
import 'package:widgets/widgets.dart';

class SearchInputBar extends StatefulWidget {
  final ValueChanged<String> onChanged;

  const SearchInputBar({super.key, required this.onChanged});

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
      if (val.trim().length > 2) {
        context.read<NotesBloc>().add(SearchEvent(val));
      } else {
        if (context.read<NotesBloc>().state.searchedNotes.isNotEmpty) {
          context.read<NotesBloc>().add(SearchEvent(''));
        }
      }
      widget.onChanged(val);
    });
  }

  void clearSearch() {
    context.read<NotesBloc>().add(ClearSearch());
    context.read<NotesBloc>().add(SearchPageToggle(isSearchPage: false));
    widget.onChanged('');
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
    return MechanixTextInput.search(
      canRequestFocus: true,
      autofocus: false,
      focusNode: _focusNode,
      hintText: "Search here",
      onClear: clearSearch,
      onChanged: (val) => _onSearchChanged(val),
    );
  }
}
