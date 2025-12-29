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
import 'package:mechanix_music/src/features/playlist_tab/add_playlist_bar.dart';
import 'package:tuple/tuple.dart';

class PlaylistList extends StatefulWidget {
  const PlaylistList({super.key});

  @override
  State<PlaylistList> createState() => _PlaylistListState();
}

class _PlaylistListState extends State<PlaylistList> {
  final scrollController = ScrollController();
  String playlistName = 'New Playlist';
  BottomBarView? previousBottomBarView;
  String renamePlaylistId = "";

  @override
  void dispose() {
    scrollController.dispose();
    super.dispose();
  }

  void _showAddPlaylistBottomSheet(BuildContext context) {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      backgroundColor: Colors.transparent,
      builder:
          (bottomSheetContext) => Container(
            decoration: BoxDecoration(
              color: Theme.of(context).scaffoldBackgroundColor,
              borderRadius: const BorderRadius.only(
                topLeft: Radius.circular(16),
                topRight: Radius.circular(16),
              ),
            ),
            child: AddPlaylistBar(
              initialValue: playlistName,
              playlistId: renamePlaylistId.isNotEmpty ? renamePlaylistId : null,
              key: const ValueKey('add_playlist'),
              onChanged: (value) {
                setState(() {
                  playlistName = value;
                });
              },
            ),
          ),
    ).whenComplete(() {
      if (mounted) {
        context.read<SongsBloc>().add(BottomBarToggle(BottomBarView.normal));
      }
      // Reset the playlist name when bottom sheet is closed
      setState(() {
        playlistName = 'New Playlist';
        renamePlaylistId = "";
      });
    });
  }

  @override
  Widget build(BuildContext context) {
    return BlocSelector<
      SongsBloc,
      SongsState,
      Tuple2<BottomBarView, PlaylistViewEnum>
    >(
      selector: (state) => Tuple2(state.bottomBarView, state.playlistView),
      builder: (context, state) {
        final bottomBarView = state.item1;
        final playlistView = state.item2;

        // Show bottom sheet when bottomBarView changes to add
        if (bottomBarView == BottomBarView.add &&
            previousBottomBarView != BottomBarView.add) {
          WidgetsBinding.instance.addPostFrameCallback((_) {
            _showAddPlaylistBottomSheet(context);
          });
        }
        previousBottomBarView = bottomBarView;

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

                BlocSelector<SongsBloc, SongsState, List<PlaylistInfo>>(
                  selector: (state) => state.playlists,
                  builder: (context, playlists) {
                    List<PlaylistInfo> displayPlaylists;

                    // Check if we're renaming an existing playlist
                    final isRenaming = renamePlaylistId.isNotEmpty;

                    if (isRenaming) {
                      // Show preview of renamed playlist in its current position
                      displayPlaylists =
                          playlists.map((playlist) {
                            if (playlist.id == renamePlaylistId) {
                              // Return a copy with the preview name
                              return PlaylistInfo(
                                id: playlist.id,
                                isShuffle: playlist.isShuffle,
                                createdAt: playlist.createdAt,
                                name: playlistName,
                                updatedAt: playlist.updatedAt,
                                songIds: playlist.songIds,
                                coverImagePath: playlist.coverImagePath,
                              );
                            }
                            return playlist;
                          }).toList();
                    } else if (bottomBarView == BottomBarView.add) {
                      // Create a new playlist info for preview when creating new
                      final newPlaylistInfo = PlaylistInfo(
                        id: "newplaylist",
                        isShuffle: false,
                        createdAt: DateTime.now(),
                        name: playlistName,
                        updatedAt: DateTime.now(),
                        songIds: [],
                        coverImagePath: null,
                      );
                      displayPlaylists = [newPlaylistInfo, ...playlists];
                    } else {
                      displayPlaylists = playlists;
                    }

                    return playlistView == PlaylistViewEnum.grid
                        ? SliverPadding(
                          padding: const EdgeInsets.all(16),
                          sliver: SliverGrid(
                            gridDelegate:
                                const SliverGridDelegateWithFixedCrossAxisCount(
                                  crossAxisCount: 3,
                                  mainAxisExtent: 164,
                                  crossAxisSpacing: 8,
                                  mainAxisSpacing: 8,
                                ),
                            delegate: SliverChildBuilderDelegate(
                              childCount: displayPlaylists.length,
                              (context, index) {
                                final playlist = displayPlaylists[index];
                                final isNewPlaylist =
                                    playlist.id == "newplaylist";
                                final isBeingRenamed =
                                    playlist.id == renamePlaylistId;

                                return AnimatedContainer(
                                  duration: const Duration(milliseconds: 200),
                                  decoration:
                                      isBeingRenamed
                                          ? BoxDecoration(
                                            borderRadius: BorderRadius.circular(
                                              8,
                                            ),
                                            border: Border.all(
                                              color:
                                                  Theme.of(
                                                    context,
                                                  ).primaryColor,
                                              width: 2,
                                            ),
                                          )
                                          : null,
                                  child: PlaylistCard(
                                    onRenameClick: (value) {
                                      setState(() {
                                        playlistName = playlist.name;
                                        renamePlaylistId = value;
                                      });
                                      context.read<SongsBloc>().add(
                                        BottomBarToggle(BottomBarView.add),
                                      );
                                    },
                                    playlistInfo: playlist,
                                    onPlaylistTap:
                                        isNewPlaylist || isBeingRenamed
                                            ? () {}
                                            : () {
                                              context.read<SongsBloc>().add(
                                                SelectedPlaylist(playlist.id),
                                              );
                                            },
                                  ),
                                );
                              },
                            ),
                          ),
                        )
                        : SliverPrototypeExtentList(
                          prototypeItem: const SizedBox(height: 79),
                          delegate: SliverChildBuilderDelegate(
                            childCount: displayPlaylists.length,
                            (context, index) {
                              final playlist = displayPlaylists[index];
                              final isNewPlaylist =
                                  playlist.id == "newplaylist";
                              final isBeingRenamed =
                                  playlist.id == renamePlaylistId;

                              return AnimatedContainer(
                                duration: const Duration(milliseconds: 200),
                                decoration:
                                    isBeingRenamed
                                        ? BoxDecoration(
                                          color: Theme.of(
                                            context,
                                          ).primaryColor.withValues(alpha: 0.1),
                                          border: Border(
                                            left: BorderSide(
                                              color:
                                                  Theme.of(
                                                    context,
                                                  ).primaryColor,
                                              width: 4,
                                            ),
                                          ),
                                        )
                                        : null,
                                child: PlaylistTile(
                                  onRenameClick:
                                      (value) => {
                                        setState(() {
                                          playlistName = playlist.name;
                                          renamePlaylistId = value;
                                        }),
                                        context.read<SongsBloc>().add(
                                          BottomBarToggle(BottomBarView.add),
                                        ),
                                      },
                                  playlistInfo: playlist,
                                  onTap:
                                      isNewPlaylist || isBeingRenamed
                                          ? () {}
                                          : () {
                                            context.read<SongsBloc>().add(
                                              SelectedPlaylist(playlist.id),
                                            );
                                          },
                                ),
                              );
                            },
                          ),
                        );
                  },
                ),
              ],
            ),
          ),
        );
      },
    );
  }
}
