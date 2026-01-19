import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/features/audio_player/audio_player_bottom_bar.dart';
import 'package:mechanix_music/src/features/audio_player/player_vinyl.dart';
import 'package:mechanix_music/src/features/audio_player/upcoming_track/upcoming_track.dart';
import 'package:widgets/mechanix.dart';
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
  bool isUpcomingTrackWindow = false;

  @override
  Widget build(BuildContext context) {
    return BlocSelector<SongsBloc, SongsState, SongInfo?>(
      selector: (state) => state.currentSong,
      builder: (context, currentSong) {
        final SongInfo song = currentSong ?? widget.songDetails;

        return Scaffold(
          backgroundColor: context.surface,
          bottomNavigationBar: AudioPlayerBottomBar(
            isUpcomingTrackWindow: isUpcomingTrackWindow,
            togglePlayer: (value) {
              setState(() => isUpcomingTrackWindow = value);
            },
          ),
          body: AnimatedSwitcher(
            duration: const Duration(milliseconds: 500),
            reverseDuration: const Duration(milliseconds: 400),
            switchInCurve: Curves.easeOutCubic,
            switchOutCurve: Curves.easeInCubic,
            transitionBuilder: _slideFadeTransition,
            child:
                isUpcomingTrackWindow
                    ? UpcomingTrack(
                      key: const ValueKey('upcoming'),
                      songDetails: song,
                      isFavorited: song.isFavourite,
                    )
                    : _MainPlayerView(
                      key: const ValueKey('player'),
                      song: song,
                    ),
          ),
        );
      },
    );
  }

  ///  Slide from bottom + fade animation
  Widget _slideFadeTransition(Widget child, Animation<double> animation) {
    final isUpcoming = child.key == const ValueKey('upcoming');

    final offsetAnimation = Tween<Offset>(
      begin: isUpcoming ? const Offset(0, 0.25) : const Offset(0, -0.15),
      end: Offset.zero,
    ).animate(animation);

    return SlideTransition(
      position: offsetAnimation,
      child: FadeTransition(opacity: animation, child: child),
    );
  }
}

class _MainPlayerView extends StatelessWidget {
  final SongInfo song;

  const _MainPlayerView({super.key, required this.song});

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        PlayerHeader(songDetails: song),
        PlayerVinyl(songDetails: song),
        PlayerSideControls(songDetails: song, isFavorited: song.isFavourite),
      ],
    );
  }
}
