import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/features/presentation/song_tile.dart';

class SongsListView extends StatelessWidget {
  final List<SongInfo> songs;
  final Function(SongInfo song)? onSongTap;

  const SongsListView({super.key, required this.songs, this.onSongTap});

  @override
  Widget build(BuildContext context) {
    return SliverPrototypeExtentList(
      prototypeItem: const SizedBox(height: 81),
      delegate: SliverChildBuilderDelegate(childCount: songs.length, (
        context,
        index,
      ) {
        final song = songs[index];

        return BlocSelector<SongsBloc, SongsState, bool>(
          selector: (state) => state.currentSong?.id == song.id,
          builder:
              (context, isCurrentSong) =>
                  SongTile(song: song, isCurrentSong: isCurrentSong),
        );
      }),
    );
  }
}
