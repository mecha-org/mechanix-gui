import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/models/search_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/home/common/music_icon_widget.dart';
import 'package:mechanix_music/src/features/home/widgets/title_widget.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_tile.dart';
import 'package:mechanix_music/src/features/presentation/song_tile.dart';
import 'package:tuple/tuple.dart';
import 'package:widgets/widgets/filled_button/mechanix_filled_button.dart';
import 'package:widgets/widgets/filled_button/mechanix_filled_button_theme.dart';

class SearchTab extends StatefulWidget {
  const SearchTab({super.key});

  @override
  State<SearchTab> createState() => _SearchTabState();
}

class _SearchTabState extends State<SearchTab> {
  final ScrollController scrollController = ScrollController();

  @override
  Widget build(BuildContext context) {
    return BlocSelector<
      SongsBloc,
      SongsState,
      Tuple2<SearchResults, List<SearchInfo>>
    >(
      selector: (state) => Tuple2(state.searchResults, state.searchItems),
      builder: (context, state) {
        final searchResults = state.item1;
        final searchItems = state.item2;

        final query = searchResults.query.trim();
        final queryLength = query.length;

        final showClearButton = queryLength < 3 && searchItems.isNotEmpty;

        final hasResults =
            searchResults.songs.isNotEmpty ||
            searchResults.playlists.isNotEmpty;

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
                /// 🔹 HEADER
                SliverToBoxAdapter(
                  child: Padding(
                    padding: const EdgeInsets.only(
                      top: 12,
                      left: 16,
                      right: 16,
                    ),
                    child: Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      crossAxisAlignment: CrossAxisAlignment.center,
                      children: [
                        const TitleWidget(
                          title: "Search Music",
                          textStyle: TextStyle(
                            color: MusicColors.primaryTextColor,
                          ),
                        ),

                        if (showClearButton)
                          MechanixFilledButton(
                            theme: MechanixFilledButtonThemeData(
                              buttonSize: const Size(90, 36),
                            ),
                            onPressed: () {
                              context.read<SongsBloc>().add(
                                ClearSerachItems(clearAll: true),
                              );
                            },
                            label: "Clear",
                          ),
                      ],
                    ),
                  ),
                ),

                /// 🔹 QUERY < 3 → RECENT SEARCHES / HINT
                if (queryLength < 3)
                  if (searchItems.isEmpty)
                    const SliverToBoxAdapter(
                      child: Padding(
                        padding: EdgeInsets.symmetric(
                          horizontal: 16,
                          vertical: 6,
                        ),
                        child: Text(
                          "Search for Tracks, Playlists and Artists",
                          style: TextStyle(
                            color: MusicColors.disabledColor,
                            fontSize: 18,
                            height: 1.45,
                          ),
                        ),
                      ),
                    )
                  else
                    SliverList(
                      delegate: SliverChildBuilderDelegate((context, index) {
                        final item = searchItems[index];

                        final playlist = item.playlistInfo;
                        if (item.isPlaylist && playlist != null) {
                          return Row(
                            children: [
                              Expanded(
                                child: PlaylistTile(
                                  onRenameClick: (value) {},
                                  isMenuRequired: false,
                                  playlistInfo: playlist,
                                  onTap: () {
                                    {
                                      context.read<SongsBloc>().add(
                                        SelectedPlaylist(playlist.id),
                                      );
                                      context.read<SongsBloc>().add(
                                        StoreSearchItem(
                                          playlist: item.playlistInfo!,
                                        ),
                                      );
                                    }
                                  },
                                ),
                              ),
                              Container(
                                margin: EdgeInsets.only(right: 16),
                                child: MusicIconButton(
                                  icon: MusicIcons.closeIcon,
                                  onPressed: () {
                                    context.read<SongsBloc>().add(
                                      ClearSerachItems(clearId: item.id),
                                    );
                                  },
                                  backgroundColor: Colors.transparent,
                                  iconSize: 24,
                                  buttonSize: 40,
                                ),
                              ),
                            ],
                          );
                        }

                        if (item.songInfo == null) {
                          return const SizedBox.shrink();
                        }

                        final song = item.songInfo!;

                        return Row(
                          children: [
                            BlocSelector<SongsBloc, SongsState, bool>(
                              selector:
                                  (state) => state.currentSong?.id == song.id,
                              builder: (context, isCurrentSong) {
                                return Expanded(
                                  child: SongTile(
                                    isCurrentSong: isCurrentSong,
                                    isMenuRequired: false,
                                    song: song,
                                    isPaddingRequired: true,
                                    onTap: () {
                                      context.read<SongsBloc>().add(
                                        StoreSearchItem(song: song),
                                      );
                                      context.read<SongsBloc>().add(
                                        PlaySong(song),
                                      );
                                    },
                                  ),
                                );
                              },
                            ),
                            Container(
                              margin: EdgeInsets.only(right: 16),
                              child: MusicIconButton(
                                icon: MusicIcons.closeIcon,
                                onPressed: () {
                                  context.read<SongsBloc>().add(
                                    ClearSerachItems(clearId: item.id),
                                  );
                                },
                                backgroundColor: Colors.transparent,
                                iconSize: 24,
                                buttonSize: 40,
                              ),
                            ),
                          ],
                        );
                      }, childCount: searchItems.length),
                    ),

                /// 🔹 QUERY ≥ 3 BUT NO RESULTS
                if (queryLength >= 3 && !hasResults)
                  const SliverToBoxAdapter(
                    child: Padding(
                      padding: EdgeInsets.symmetric(
                        horizontal: 16,
                        vertical: 6,
                      ),
                      child: Text(
                        "No Results Found",
                        style: TextStyle(
                          color: MusicColors.disabledColor,
                          fontSize: 18,
                          height: 1.45,
                        ),
                      ),
                    ),
                  ),

                /// 🔹 QUERY ≥ 3 → SEARCH RESULTS
                if (queryLength >= 3 && hasResults)
                  SliverList(
                    delegate: SliverChildListDelegate([
                      if (searchResults.songs.isNotEmpty) ...[
                        ...searchResults.songs.map(
                          (song) => BlocSelector<SongsBloc, SongsState, bool>(
                            selector:
                                (state) => state.currentSong?.id == song.id,
                            builder: (context, isCurrentSong) {
                              return SongTile(
                                song: song,
                                isCurrentSong: isCurrentSong,
                                isPaddingRequired: true,
                                onTap: () {
                                  context.read<SongsBloc>().add(
                                    StoreSearchItem(song: song),
                                  );
                                  context.read<SongsBloc>().add(PlaySong(song));
                                },
                              );
                            },
                          ),
                        ),
                      ],

                      if (searchResults.playlists.isNotEmpty) ...[
                        ...searchResults.playlists.map(
                          (playlist) => PlaylistTile(
                            onRenameClick: (value) {},
                            playlistInfo: playlist,
                            onTap: () {
                              context.read<SongsBloc>().add(
                                SelectedPlaylist(playlist.id),
                              );
                              context.read<SongsBloc>().add(
                                StoreSearchItem(playlist: playlist),
                              );
                              // Navigator.push(
                              //   context,
                              //   MaterialPageRoute(
                              //     builder: (context) => PlaylistView(),
                              //   ),
                              // );
                            },
                          ),
                        ),
                      ],
                    ]),
                  ),
              ],
            ),
          ),
        );
      },
    );
  }
}
