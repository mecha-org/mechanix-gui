import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/home/common/music_icon_widget.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_menu.dart';
import 'package:tuple/tuple.dart';

class PlaylistActionsView extends StatefulWidget {
  final VoidCallback onEdit;
  final bool isEditMode;
  final PlaylistInfo playlistInfo;

  const PlaylistActionsView({
    super.key,
    required this.onEdit,
    required this.isEditMode,
    required this.playlistInfo,
  });

  @override
  State<PlaylistActionsView> createState() => _PlaylistActionsViewState();
}

class _PlaylistActionsViewState extends State<PlaylistActionsView> {
  bool isShuffle = false;

  @override
  void initState() {
    super.initState();
    setState(() {
      isShuffle = widget.playlistInfo.isShuffle;
    });
  }

  @override
  Widget build(BuildContext context) {
    return BlocSelector<SongsBloc, SongsState, List<SongInfo>>(
      selector: (state) => state.playlistSongs,
      builder: (context, playlistSongs) {
        final isFavourite =
            playlistSongs.isNotEmpty
                ? playlistSongs.every((element) => element.isFavourite)
                : false;

        return SliverToBoxAdapter(
          child: Row(
            mainAxisAlignment: MainAxisAlignment.center,
            spacing: 44,
            children: [
              MusicIconButton(
                enabled: !widget.isEditMode && playlistSongs.isNotEmpty,
                iconColor: widget.isEditMode ? Colors.grey : null,
                icon:
                    isFavourite
                        ? MusicIcons.filledFavouriteIcon
                        : MusicIcons.favouritesIcon,
                onPressed:
                    playlistSongs.isNotEmpty
                        ? () {
                          context.read<SongsBloc>().add(
                            FavouriteToggle(
                              isFavourite: !isFavourite,
                              songIds: playlistSongs.map((e) => e.id).toList(),
                            ),
                          );
                        }
                        : null,
              ),
              MusicIconButton(
                enabled: !widget.isEditMode && playlistSongs.isNotEmpty,
                onPressed: () {
                  context.read<SongsBloc>().add(
                    PlaylistShuffle(
                      playlistId: widget.playlistInfo.id,
                      isShuffle: !isShuffle,
                    ),
                  );
                  setState(() {
                    isShuffle = !isShuffle;
                  });
                },
                icon:
                    isShuffle
                        ? MusicIcons.shuffleEnableIcon
                        : MusicIcons.shuffleIcon,
                iconColor:
                    widget.isEditMode
                        ? Colors.grey
                        : isShuffle
                        ? MusicColors.titleColor
                        : null,
              ),

              BlocSelector<
                SongsBloc,
                SongsState,
                Tuple2<CurrentPlaylist, bool>
              >(
                selector:
                    (state) => Tuple2(state.currentPlaylist, state.isPlaying),
                builder: (context, state) {
                  final isPlaying =
                      state.item2 &&
                      state.item1.playlistId == widget.playlistInfo.id;

                  return MusicIconButton(
                    enabled: !widget.isEditMode && playlistSongs.isNotEmpty,
                    iconColor: widget.isEditMode ? Colors.grey : null,
                    onPressed: () {
                      context.read<SongsBloc>().add(
                        isPlaying
                            ? PausePlaylistSongs()
                            : PlayPlaylistSongs(
                              playlistId: widget.playlistInfo.id,
                              isShuffle: isShuffle,
                            ),
                      );
                    },
                    icon:
                        isPlaying
                            ? MusicIcons.pauseIcon
                            : MusicIcons.playlistPlayIcon,
                    iconSize: 32,
                    buttonSize: 52,
                  );
                },
              ),
              MusicIconButton(
                enabled: !widget.isEditMode && playlistSongs.isNotEmpty,
                iconColor: widget.isEditMode ? Colors.grey : null,

                onPressed: widget.onEdit,
                icon: MusicIcons.editIcon,
              ),
              PlaylistMenu(
                isBackgroundRequired: true,
                enabled: !widget.isEditMode,
                iconSize: Size(44, 44),
                isDeletePlaylist: true,
                isLiked: false,
                isRenamePlaylist: true,
                onRenameClick: (value) {
                  context.read<SongsBloc>().add(
                    BottomBarToggle(BottomBarView.add),
                  );
                },
                playlistInfo: widget.playlistInfo,
              ),
            ],
          ),
        );
      },
    );
  }
}
