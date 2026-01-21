import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_view/playlist_actions_view.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_view/playlist_add_song.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_view/playlist_heading.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_view/playlist_top_view.dart';
import 'package:mechanix_music/src/features/presentation/song_tile.dart';
import 'package:tuple/tuple.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/filled_button/mechanix_filled_button_theme.dart';

class PlaylistView extends StatefulWidget {
  const PlaylistView({super.key});

  @override
  State<PlaylistView> createState() => _PlaylistViewState();
}

class _PlaylistViewState extends State<PlaylistView> {
  final ScrollController scrollController = ScrollController();
  bool isEditMode = false;
  List<SongInfo> editableSongs = [];
  List<String> deletedSongIds = [];
  List<String> originalSongIds = []; // Track original order by IDs only

  @override
  void initState() {
    super.initState();
    // context.read<SongsBloc>().add(GetPlaylistSongs(widget.playlistInfo.id));
  }

  // Toggle edit mode
  void toggleEditMode(List<SongInfo> songs, PlaylistInfo playlist) {
    setState(() {
      if (!isEditMode) {
        // Entering edit mode - save original order and create working copy
        originalSongIds = songs.map((s) => s.id).toList();
        editableSongs = List.from(songs);
        deletedSongIds.clear();
      } else {
        // Exiting edit mode - send updates to backend
        final currentSongIds = editableSongs.map((s) => s.id).toList();

        if (deletedSongIds.isNotEmpty ||
            !_isSameOrder(originalSongIds, currentSongIds)) {
          // Send update event with ordered song IDs and deleted IDs
          context.read<SongsBloc>().add(
            UpdatedPlaylistSongs(
              playlistId: playlist.id,
              orderedSongIds: currentSongIds,
              deletedSongIds: deletedSongIds,
            ),
          );
        }
      }
      isEditMode = !isEditMode;
    });
  }

  // Check if song order is the same
  bool _isSameOrder(List<String> list1, List<String> list2) {
    if (list1.length != list2.length) return false;
    for (int i = 0; i < list1.length; i++) {
      if (list1[i] != list2[i]) return false;
    }
    return true;
  }

  // Handle reorder
  void onReorder(int oldIndex, int newIndex) {
    setState(() {
      if (newIndex > oldIndex) {
        newIndex -= 1;
      }
      final song = editableSongs.removeAt(oldIndex);
      editableSongs.insert(newIndex, song);
    });
  }

  // Handle delete
  void onDelete(int index) {
    setState(() {
      final deletedSong = editableSongs.removeAt(index);
      deletedSongIds.add(deletedSong.id);
    });
  }

  @override
  Widget build(BuildContext context) {
    return BlocSelector<SongsBloc, SongsState, PlaylistInfo?>(
      selector: (state) => state.selectedPlaylist,
      builder: (context, playlistInfo) {
        if (playlistInfo == null) return const SizedBox.shrink();
        final playlist = playlistInfo;

        return Padding(
          padding: const EdgeInsets.fromLTRB(0, 12, 0, 0),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              //  Playlist name (fixed)
              isEditMode
                  ? Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      Text(
                        "Edit Playlist",
                        style: TextStyle(
                          color: context.colorScheme.onSurface,
                          fontWeight: FontWeight.w600,
                          fontSize: 24,
                          letterSpacing: -1.1,
                          height: 1.25,
                        ),
                      ),
                      BlocSelector<SongsBloc, SongsState, List<SongInfo>>(
                        selector: (state) => state.playlistSongs,
                        builder: (context, songs) {
                          return MechanixFilledButton(
                            theme: MechanixFilledButtonThemeData(
                              buttonSize: Size(85, 36),
                            ),
                            onPressed: () {
                              toggleEditMode(songs, playlist);
                            },
                            label: "Done",
                          );
                        },
                      ),
                    ],
                  ).padSymmetric(horizontal: 16)
                  : PlaylistHeading(playlist: playlist),

              const SizedBox(height: 16),

              /// 🔹 Scrollable content
              Expanded(
                child: Scrollbar(
                  controller: scrollController,
                  child: ScrollConfiguration(
                    behavior: const ScrollBehavior().copyWith(
                      overscroll: false,
                      scrollbars: false,
                      dragDevices: {
                        PointerDeviceKind.touch,
                        PointerDeviceKind.mouse,
                      },
                    ),
                    child: CustomScrollView(
                      controller: scrollController,
                      slivers: [
                        if (!isEditMode) ...[
                          PlaylistTopView(playlistInfo: playlist),
                          SliverToBoxAdapter(child: SizedBox(height: 16)),
                          // Audio Actions
                          BlocSelector<SongsBloc, SongsState, List<SongInfo>>(
                            selector: (state) => state.playlistSongs,
                            builder: (context, songs) {
                              return PlaylistActionsView(
                                playlistInfo: playlist,
                                isEditMode: isEditMode,
                                onEdit: () => toggleEditMode(songs, playlist),
                              );
                            },
                          ),
                          SliverToBoxAdapter(
                            child: Container(
                              height: 1,
                              color: context.surfaceContainerHigh,
                              margin: EdgeInsets.all(16),
                            ),
                          ),
                          // Add song button
                          SliverPadding(
                            padding: EdgeInsets.symmetric(horizontal: 16),
                            sliver: PlaylistAddSong(
                              playlistInfo: playlist,
                              isEditMode: isEditMode,
                            ),
                          ),
                          SliverToBoxAdapter(child: SizedBox(height: 16)),
                        ],

                        //Top View
                        BlocSelector<
                          SongsBloc,
                          SongsState,
                          Tuple2<List<SongInfo>, MusicMode>
                        >(
                          selector:
                              (state) =>
                                  Tuple2(state.playlistSongs, state.musicMode),
                          builder: (context, state) {
                            final songs = state.item1;
                            final musicMode = state.item2;
                            // Use editable songs in edit mode, original songs in normal mode
                            final displaySongs =
                                isEditMode ? editableSongs : songs;

                            if (!isEditMode) {
                              // Normal mode - regular list
                              return SliverPadding(
                                padding: EdgeInsets.only(left: 16, right: 16),
                                sliver: SliverList(
                                  delegate: SliverChildBuilderDelegate(
                                    (context, index) {
                                      return BlocSelector<
                                        SongsBloc,
                                        SongsState,
                                        bool
                                      >(
                                        selector:
                                            (state) =>
                                                state.currentSong?.id ==
                                                    displaySongs[index].id &&
                                                state
                                                        .currentPlaylist
                                                        .playlistId ==
                                                    playlist.id,
                                        builder:
                                            (
                                              context,
                                              isCurrentSong,
                                            ) => SongTile(
                                              song: displaySongs[index],
                                              onTap:
                                                  () => context
                                                      .read<SongsBloc>()
                                                      .add(
                                                        PlayPlaylistSongs(
                                                          playlistId:
                                                              playlist.id,
                                                          isShuffle:
                                                              playlist
                                                                  .isShuffle,
                                                          songIndex: index,
                                                        ),
                                                      ),
                                              isCurrentSong:
                                                  isCurrentSong &&
                                                  musicMode ==
                                                      MusicMode.playlist,
                                            ),
                                      );
                                    },
                                    childCount: displaySongs.length,
                                    addAutomaticKeepAlives: false,
                                    addRepaintBoundaries: true,
                                  ),
                                ),
                              );
                            } else {
                              // Edit mode - reorderable list with delete
                              return SliverPadding(
                                padding: EdgeInsets.only(left: 16, right: 16),
                                sliver: SliverReorderableList(
                                  proxyDecorator: (child, index, animation) {
                                    return Material(
                                      color: Colors.transparent,
                                      elevation: 6,
                                      child: SizedBox(height: 79, child: child),
                                    );
                                  },
                                  itemCount: displaySongs.length,
                                  onReorder: (oldIndex, newIndex) {
                                    onReorder(oldIndex, newIndex);
                                  },
                                  itemBuilder: (context, index) {
                                    return ReorderableDragStartListener(
                                      key: ValueKey(displaySongs[index].id),
                                      index: index,
                                      child: Material(
                                        child: BlocSelector<
                                          SongsBloc,
                                          SongsState,
                                          bool
                                        >(
                                          selector:
                                              (state) =>
                                                  state.currentSong?.id ==
                                                      displaySongs[index].id &&
                                                  state
                                                          .currentPlaylist
                                                          .playlistId ==
                                                      playlist.id,
                                          builder:
                                              (context, isCurrentSong) => Row(
                                                children: [
                                                  // Delete icon on left
                                                  IconWidget(
                                                    iconPath:
                                                        MusicIcons
                                                            .threeLineIcon,
                                                    boxHeight: 40,
                                                    boxWidth: 40,
                                                    iconHeight: 24,
                                                    iconWidth: 24,
                                                  ),
                                                  SizedBox(width: 8),
                                                  // Song tile
                                                  Expanded(
                                                    child: SongTile(
                                                      song: displaySongs[index],
                                                      isEditMode: true,

                                                      isCurrentSong:
                                                          isCurrentSong &&
                                                          musicMode ==
                                                              MusicMode
                                                                  .playlist,
                                                    ),
                                                  ),
                                                  // Drag handle icon on right
                                                  if (isCurrentSong)
                                                    SizedBox(width: 40)
                                                  else
                                                    IconButton(
                                                      disabledColor:
                                                          Theme.of(
                                                            context,
                                                          ).disabledColor,
                                                      onPressed:
                                                          isCurrentSong
                                                              ? null
                                                              : () => onDelete(
                                                                index,
                                                              ),
                                                      iconSize: 40,

                                                      icon: IconWidget(
                                                        iconPath:
                                                            MusicIcons
                                                                .deleteIcon,
                                                        boxHeight: 24,
                                                        boxWidth: 24,
                                                        iconHeight: 24,
                                                        iconWidth: 24,
                                                      ),
                                                    ),
                                                ],
                                              ),
                                        ),
                                      ),
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
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}
