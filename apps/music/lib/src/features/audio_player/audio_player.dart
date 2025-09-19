import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/features/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/features/bloc/songs_event.dart';
import 'package:mechanix_music/src/features/bloc/songs_state.dart';
import 'player_header.dart';
import 'player_vinyl.dart';
import 'player_side_controls.dart';
import 'player_controls.dart';

class AudioPlayer extends StatefulWidget {
  final SongInfo songDetails;
  const AudioPlayer({super.key, required this.songDetails});

  @override
  State<AudioPlayer> createState() => _AudioPlayerState();
}

class _AudioPlayerState extends State<AudioPlayer>
    with TickerProviderStateMixin {
  // bool isPlaying = true;
  // bool isShuffled = false;
  bool isRepeating = false;
  bool isFavorited = false;
  double currentPosition = 0.0;
  late AnimationController _rotationController;
  Timer? _timer;

  @override
  void initState() {
    super.initState();
    _rotationController = AnimationController(
      duration: const Duration(seconds: 10),
      vsync: this,
    );

    isFavorited = widget.songDetails.isFavourite;
    context.read<SongsBloc>().add(PlaySong(widget.songDetails.index));
    // _startProgressTimer();
  }

  @override
  void dispose() {
    _timer?.cancel();
    _rotationController.dispose();
    super.dispose();
  }

  // void _togglePlayPause() {
  //   context.read<SongsBloc>().add(TogglePlayPause());
  //   setState(() {
  //     isPlaying = !isPlaying;
  //     if (isPlaying) {
  //       _rotationController.repeat();
  //       _startProgressTimer();
  //     } else {
  //       _rotationController.stop(canceled: false);
  //       _timer?.cancel();
  //     }
  //   });
  // }

  // void _startProgressTimer() {
  //   final totalDuration = const Duration(minutes: 2, seconds: 30);
  //   _timer?.cancel();
  //   const interval = Duration(milliseconds: 100);

  //   _timer = Timer.periodic(interval, (timer) {
  //     if (!mounted) {
  //       timer.cancel();
  //       return;
  //     }
  //     setState(() {
  //       currentPosition +=
  //           interval.inMilliseconds / totalDuration.inMilliseconds;
  //       if (currentPosition >= 1.0) {
  //         currentPosition = 1.0;
  //         timer.cancel();
  //         isPlaying = false;
  //         _rotationController.stop();
  //       }
  //     });
  //   });
  // }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SongsBloc, SongsState>(
      builder: (context, state) {
        return Scaffold(
          backgroundColor: Colors.black,
          body: SafeArea(
            child: Column(
              children: [
                PlayerHeader(
                  isFavorited: isFavorited,
                  songDetails: state.currentSong ?? widget.songDetails,
                ),
                PlayerVinyl(
                  songDetails: state.currentSong ?? widget.songDetails,
                  isPlaying: state.isPlaying,
                  currentPosition: currentPosition,
                  currentDuration: state.position,
                  totalDuration: state.duration,
                  onPositionChange: (Duration seekPosition) {
                    // Trigger seek event in your BLoC
                    context.read<SongsBloc>().add(SeekSong(seekPosition));
                  },
                  // onPositionChange: (v) => setState(() => currentPosition = v),
                ),
                PlayerSideControls(
                  isFavorited: isFavorited,
                  onFavoriteToggle:
                      () => setState(() => isFavorited = !isFavorited),
                ),
                PlayerControls(
                  isPlaying: state.isPlaying,
                  isRepeating: isRepeating,
                  isShuffled: state.isShuffled,
                  onRepeatToggle:
                      () => setState(() => isRepeating = !isRepeating),
                  onShuffleToggle:
                      () => context.read<SongsBloc>().add(ShuffleToggle()),
                ),
              ],
            ),
          ),
        );
      },
    );
  }
}
