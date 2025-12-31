import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_list.dart';

class PlaylistTab extends StatefulWidget {
  const PlaylistTab({super.key});

  @override
  State<PlaylistTab> createState() => _PlaylistTabState();
}

class _PlaylistTabState extends State<PlaylistTab> {
  final ScrollController controller = ScrollController();
  Timer? _scrollEndTimer;
  bool _isScrolling = false;

  @override
  void initState() {
    super.initState();
    controller.addListener(_onScroll);
  }

  void _onScroll() {
    final bloc = context.read<SongsBloc>();

    // 🔹 Scroll started (emit once)
    if (!_isScrolling) {
      _isScrolling = true;
      bloc.add(const ToggleScrolling(true));
    }

    // 🔹 Scroll ended (debounced)
    _scrollEndTimer?.cancel();
    _scrollEndTimer = Timer(const Duration(milliseconds: 300), () {
      if (!mounted) return;
      _isScrolling = false;
      bloc.add(const ToggleScrolling(false));
    });
  }

  @override
  void dispose() {
    _scrollEndTimer?.cancel();
    controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return PlaylistList(scrollController: controller);
  }
}
