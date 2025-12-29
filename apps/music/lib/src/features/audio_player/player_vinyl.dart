import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/features/audio_player/player_disc_rotate.dart';

class PlayerVinyl extends StatefulWidget {
  final SongInfo songDetails;

  const PlayerVinyl({super.key, required this.songDetails});

  @override
  State<PlayerVinyl> createState() => _PlayerVinylState();
}

class _PlayerVinylState extends State<PlayerVinyl>
    with SingleTickerProviderStateMixin {
  @override
  Widget build(BuildContext context) {
    return Center(
      child: Container(
        padding: EdgeInsets.all(0),
        child: Column(
          children: [
            // BlocSelector
            BlocSelector<SongsBloc, SongsState, bool>(
              selector: (state) => state.isPlaying,
              builder:
                  (context, isPlaying) => PlayerDiscRotate(
                    songDetails: widget.songDetails,
                    isPlaying: isPlaying,
                  ),
            ),

            SizedBox(height: 20),
          ],
        ),
      ),
    );
  }
}
