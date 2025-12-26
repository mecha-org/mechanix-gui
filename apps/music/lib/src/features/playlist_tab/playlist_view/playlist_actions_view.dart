import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/home/common/music_icon_widget.dart';
import 'package:tuple/tuple.dart';

class PlaylistActionsView extends StatelessWidget {
  final VoidCallback onEdit;
  final String playlistId;
  const PlaylistActionsView({
    super.key,
    required this.onEdit,
    required this.playlistId,
  });

  @override
  Widget build(BuildContext context) {
    return SliverToBoxAdapter(
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        spacing: 44,
        children: [
          MusicIconButton(icon: MusicIcons.favouritesIcon),
          MusicIconButton(icon: MusicIcons.shuffleIcon),
          BlocSelector<SongsBloc, SongsState, Tuple2<CurrentPlaylist, bool>>(
            selector: (state) => Tuple2(state.currentPlaylist, state.isPlaying),
            builder: (context, state) {
              final isPlaying =
                  state.item2 && state.item1.playlistId == playlistId;

              return MusicIconButton(
                onPressed:
                    () => context.read<SongsBloc>().add(
                      isPlaying
                          ? PausePlaylistSongs()
                          : PlayPlaylistSongs(playlistId),
                    ),
                icon:
                    isPlaying
                        ? MusicIcons.pauseIcon
                        : MusicIcons.playlistPlayIcon,
                iconSize: 32,
                buttonSize: 52,
              );
            },
          ),
          MusicIconButton(onPressed: onEdit, icon: MusicIcons.editIcon),
          MusicIconButton(icon: MusicIcons.threeDotIcon),
        ],
      ),
    );
  }
}
