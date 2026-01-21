import 'dart:async';
import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/features/home/widgets/title_widget.dart';
import 'package:mechanix_music/src/features/presentation/song_tile.dart';
import 'package:widgets/mechanix.dart';

class FavouritesTab extends StatefulWidget {
  const FavouritesTab({super.key});

  @override
  State<FavouritesTab> createState() => _FavouritesTabState();
}

class _FavouritesTabState extends State<FavouritesTab> {
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
          scrollbars:
              false, // Disable default scrollbar since we're using Scrollbar widget
          dragDevices: {PointerDeviceKind.touch, PointerDeviceKind.mouse},
        ),
        child: Container(
          padding: EdgeInsets.only(left: 16, right: 16, top: 12, bottom: 0),
          child: CustomScrollView(
            physics: const BouncingScrollPhysics(),
            controller: scrollController,
            slivers: [
              // Title widget is now part of the scrollable content
              const SliverToBoxAdapter(
                child: TitleWidget(title: "Liked Songs"),
              ),

              BlocSelector<SongsBloc, SongsState, List<SongInfo>>(
                selector: (state) => state.favouriteSongs,
                builder: (context, songs) {
                  if (songs.isEmpty) {
                    return SliverToBoxAdapter(
                      child: Container(
                        margin: EdgeInsets.only(top: 2),
                        child: Text(
                          "No liked songs yet",
                          style: TextStyle(
                            fontSize: 18,
                            fontWeight: FontWeight.w600,
                            color: context.onSurface,
                          ),
                        ),
                      ),
                    );
                  }

                  return SliverPrototypeExtentList(
                    prototypeItem: const SizedBox(height: 79),
                    delegate: SliverChildBuilderDelegate(
                      childCount: songs.length,
                      (context, index) {
                        final song = songs[index];

                        return BlocSelector<SongsBloc, SongsState, bool>(
                          selector:
                              (state) =>
                                  state.currentSong?.id == song.id &&
                                  state.musicMode == MusicMode.favorite,
                          builder: (context, isCurrentSong) {
                            return SongTile(
                              song: song,
                              isCurrentSong: isCurrentSong,
                              onTap: () {
                                context.read<SongsBloc>().add(
                                  PlayFavoriteSongs(song: song),
                                );
                              },
                            );
                          },
                        );
                      },
                    ),
                  );
                },
              ),
            ],
          ),
        ),
      ),
    );
  }
}
