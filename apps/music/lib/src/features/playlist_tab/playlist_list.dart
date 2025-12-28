import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/features/home/widgets/title_widget.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_card.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_tile.dart';
import 'package:tuple/tuple.dart';

class PlaylistList extends StatefulWidget {
  const PlaylistList({super.key});

  @override
  State<PlaylistList> createState() => _PlaylistListState();
}

class _PlaylistListState extends State<PlaylistList> {
  final scrollController = ScrollController();

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
                child: TitleWidget(title: "Playlists"),
              ),
            ),

            BlocSelector<
              SongsBloc,
              SongsState,
              Tuple2<List<PlaylistInfo>, PlaylistViewEnum>
            >(
              selector: (state) => Tuple2(state.playlists, state.playlistView),
              builder: (context, state) {
                final playlists = state.item1;
                final viewType = state.item2;

                // Check if it's grid view (adjust the enum value as per your implementation)
                if (viewType == PlaylistViewEnum.grid) {
                  return SliverPadding(
                    padding: EdgeInsets.all(16),
                    sliver: SliverGrid(
                      gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                        crossAxisCount: 3,
                        mainAxisExtent: 164,
                        crossAxisSpacing: 8,
                        mainAxisSpacing: 8,
                      ),
                      delegate: SliverChildBuilderDelegate(
                        childCount: playlists.length,
                        (context, index) {
                          final playlist = playlists[index];
                          return PlaylistCard(
                            playlistInfo: playlist,
                            onPlaylistTap: () {
                              context.read<SongsBloc>().add(
                                SelectedPlaylist(playlist.id),
                              );
                            },
                          );
                        },
                      ),
                    ),
                  );
                } else {
                  // List view
                  return SliverPrototypeExtentList(
                    prototypeItem: const SizedBox(height: 79),
                    delegate: SliverChildBuilderDelegate(
                      childCount: playlists.length,
                      (context, index) {
                        final playlist = playlists[index];
                        return PlaylistTile(
                          playlistInfo: playlist,
                          onTap: () {
                            context.read<SongsBloc>().add(
                              SelectedPlaylist(playlist.id),
                            );
                          },
                          // () => Navigator.push(
                          //   context,
                          //   MaterialPageRoute(
                          //     builder:
                          //         (context) =>
                          //             PlaylistView(playlistInfo: playlist),
                          //   ),
                          // ),
                        );
                      },
                    ),
                  );
                }
              },
            ),
          ],
        ),
      ),
    );
  }
}
