import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/features/audio_player/audio_player_bottom_bar.dart';
import 'package:mechanix_music/src/features/audio_player/player_vinyl.dart';
import 'player_header.dart';
import 'player_side_controls.dart';

class AudioPlayer extends StatefulWidget {
  final SongInfo songDetails;
  const AudioPlayer({super.key, required this.songDetails});

  @override
  State<AudioPlayer> createState() => _AudioPlayerState();
}

class _AudioPlayerState extends State<AudioPlayer>
    with TickerProviderStateMixin {
  @override
  Widget build(BuildContext context) {
    return BlocSelector<SongsBloc, SongsState, SongInfo?>(
      selector: (state) => state.currentSong,
      builder: (context, currentSong) {
        final SongInfo song = currentSong ?? widget.songDetails;
        return Scaffold(
          bottomNavigationBar: const AudioPlayerBottomBar(),
          body: Container(
            padding: EdgeInsets.all(0),
            child: Column(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                PlayerHeader(songDetails: song),
                PlayerVinyl(songDetails: song),
                PlayerSideControls(songDetails:song, isFavorited: song.isFavourite),
              ],
            ),
          ),
        );
      },
    );
  }
}
