import 'dart:async';
import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/features/home/widgets/title_widget.dart';
import 'package:mechanix_music/src/features/presentation/song_list_view.dart';

class MusicTab extends StatefulWidget {
  const MusicTab({super.key});

  @override
  State<MusicTab> createState() => _MusicTabState();
}

class _MusicTabState extends State<MusicTab> {
  final ScrollController scrollController = ScrollController();

  Timer? _scrollEndTimer;
  bool _isScrolling = false;

  @override
  void initState() {
    super.initState();
    scrollController.addListener(_onScroll);
  }

  void _onScroll() {
    final bloc = context.read<SongsBloc>();

    // 🔹 Scroll start (fire once)
    if (!_isScrolling) {
      _isScrolling = true;
      bloc.add(const ToggleScrolling(true));
    }

    // 🔹 Scroll end debounce
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
    scrollController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scrollbar(
      controller: scrollController,
      child: ScrollConfiguration(
        behavior: const ScrollBehavior().copyWith(
          overscroll: false,
          scrollbars: false,
          dragDevices: {PointerDeviceKind.touch, PointerDeviceKind.mouse},
        ),
        child: Padding(
          padding: const EdgeInsets.only(left: 16, right: 16, top: 12),
          child: CustomScrollView(
            controller: scrollController,
            physics: const BouncingScrollPhysics(),
            slivers: [
              const SliverToBoxAdapter(child: TitleWidget(title: "My Music")),

              BlocSelector<SongsBloc, SongsState, List<SongInfo>>(
                selector: (state) => state.songs,
                builder: (context, songs) {
                  return SongsListView(songs: songs);
                },
              ),

              const SliverToBoxAdapter(child: SizedBox(height: 60)),
            ],
          ),
        ),
      ),
    );
  }
}
