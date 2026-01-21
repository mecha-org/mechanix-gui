import 'dart:async';
import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/features/home/home_tab/empty_home_screen.dart';
import 'package:mechanix_music/src/features/home/home_tab/recent_songs.dart';
import 'package:mechanix_music/src/features/home/home_tab/top_music.dart';
import 'package:mechanix_music/src/features/home/home_tab/top_playlists.dart';
import 'package:mechanix_music/src/features/home/widgets/title_widget.dart';
import 'package:widgets/extensions/edge_insets.dart';

class HomeTab extends StatefulWidget {
  const HomeTab({super.key});

  @override
  State<HomeTab> createState() => _HomeTabState();
}

class _HomeTabState extends State<HomeTab> {
  final ScrollController scrollController = ScrollController();
  Timer? _scrollEndTimer;
  bool _isScrolling = false;

  @override
  void initState() {
    super.initState();
    scrollController.addListener(_onScroll);
  }

  void _onScroll() {
    // Scroll start (fire once)
    if (!_isScrolling) {
      _isScrolling = true;
      context.read<SongsBloc>().add(ToggleScrolling(true));
    }

    // Scroll end debounce
    _scrollEndTimer?.cancel();
    _scrollEndTimer = Timer(const Duration(milliseconds: 300), () {
      if (!mounted) return;
      _isScrolling = false;
      context.read<SongsBloc>().add(ToggleScrolling(false));
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
        child: BlocSelector<SongsBloc, SongsState, bool>(
          selector: (state) => state.isLoading,
          builder: (context, isLoading) {
            if (isLoading) {
              return const HomeLoader();
            }

            return BlocSelector<SongsBloc, SongsState, bool>(
              selector:
                  (state) =>
                      state.recentlyPlayedSongs.isEmpty &&
                      state.playlists.isEmpty &&
                      state.songs.isEmpty,
              builder: (context, isEmpty) {
                return SingleChildScrollView(
                  controller: scrollController,
                  physics: const BouncingScrollPhysics(),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      TitleWidget(
                        title: "Music",
                        textStyle: const TextStyle(
                          fontSize: 32,
                          fontWeight: FontWeight.w600,
                        ),
                      ).padOnly(left: 16, top: 12),

                      if (!isEmpty) ...[
                        const RecentSong(),

                        const TopPlaylists(),
                        TopMusic(),
                        const SizedBox(height: 40),
                      ] else
                        const EmptyHomeScreen(),
                    ],
                  ),
                );
              },
            );
          },
        ),
      ),
    );
  }
}

class HomeLoader extends StatelessWidget {
  const HomeLoader({super.key});

  @override
  Widget build(BuildContext context) {
    return const Center(
      child: Padding(
        padding: EdgeInsets.only(top: 120),
        child: CircularProgressIndicator(),
      ),
    );
  }
}
