import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/commons/constants.dart';
import 'package:widgets/widgets.dart';

class MusicSearchBar extends StatefulWidget {
  const MusicSearchBar({super.key});

  @override
  State<MusicSearchBar> createState() => _MusicSearchBarState();
}

class _MusicSearchBarState extends State<MusicSearchBar> {
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
        context.read<SongsBloc>().add(SearchSong(val));
      } else {
        if (context.read<SongsBloc>().state.searchResults.songs.isNotEmpty ||
            context
                .read<SongsBloc>()
                .state
                .searchResults
                .playlists
                .isNotEmpty) {
          context.read<SongsBloc>().add(SearchSong(''));
        }
      }
    });
  }

  void clearSearch() {
    context.read<SongsBloc>().add(SearchSong(''));
    context.read<SongsBloc>().add(BottomBarToggle(BottomBarView.normal));
    context.read<SongsBloc>().add(MusicTabSwitch(MusicTabs.home));
  }

  @override
  void dispose() {
    _debounceTimer?.cancel();
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
