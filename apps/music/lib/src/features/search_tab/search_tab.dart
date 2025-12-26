import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/features/home/widgets/title_widget.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_tile.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_view/playlist_view.dart';
import 'package:mechanix_music/src/features/presentation/song_tile.dart';
import 'package:widgets/mechanix.dart';

class SearchTab extends StatefulWidget {
  const SearchTab({super.key});

  @override
  State<SearchTab> createState() => _SearchTabState();
}

class _SearchTabState extends State<SearchTab> {
  final ScrollController scrollController = ScrollController();
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
        child: CustomScrollView(
          controller: scrollController,
          slivers: [
            const SliverToBoxAdapter(
              child: Padding(
                padding: EdgeInsets.only(top: 12, left: 16, right: 16),
                child: TitleWidget(
                  title: "Search Music",
                  textStyle: TextStyle(color: MusicColors.primaryTextColor),
                ),
              ),
            ),
            BlocSelector<SongsBloc, SongsState, SearchResults>(
              selector: (state) => state.searchResults,
              builder: (context, state) {
                final hasResults =
                    state.playlists.isNotEmpty || state.songs.isNotEmpty;

                if (state.query.trim().length < 3 || !hasResults) {
                  return SliverToBoxAdapter(
                    child: Text(
                      !hasResults
                          ? "No Results Found"
                          : "Search for Tracks, Playlists and Artists",
                      style: const TextStyle(
                        color: MusicColors.disabledColor,
                        fontSize: 18,
                        height: 1.45,
                      ),
                    ).padSymmetric(horizontal: 16, vertical: 6),
                  );
                }

                return SliverList(
                  delegate: SliverChildListDelegate([
                    if (state.songs.isNotEmpty) ...[
                      ...state.songs.map(
                        (song) => BlocSelector<SongsBloc, SongsState, bool>(
                          selector: (state) => state.currentSong?.id == song.id,
                          builder: (context, isCurrentSong) {
                            return SongTile(
                              song: song,
                              isCurrentSong: true,
                              isPaddingRequired: true,
                            );
                          },
                        ),
                      ),
                    ],

                    if (state.playlists.isNotEmpty) ...[
                      ...state.playlists.map(
                        (playlist) => PlaylistTile(
                          playlistInfo: playlist,
                          onTap: () {
                            context.read<SongsBloc>().add(
                              BottomBarToggle(BottomBarView.normal),
                            );
                            Navigator.push(
                              context,
                              MaterialPageRoute(
                                builder:
                                    (_) => PlaylistView(playlistInfo: playlist),
                              ),
                            );
                          },
                        ),
                      ),
                    ],
                  ]),
                );
              },
            ),
          ],
        ),
      ),
    );
  }
}
