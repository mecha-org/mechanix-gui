import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/src/features/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/features/bloc/songs_event.dart';

class PlayerControls extends StatelessWidget {
  final bool isPlaying;
  final bool isRepeating;
  final bool isShuffled;
  // final VoidCallback onPlayPause;
  final VoidCallback onRepeatToggle;
  final VoidCallback onShuffleToggle;

  const PlayerControls({
    super.key,
    required this.isPlaying,
    required this.isRepeating,
    required this.isShuffled,
    // required this.onPlayPause,
    required this.onRepeatToggle,
    required this.onShuffleToggle,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 40, vertical: 20),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceEvenly,
        children: [
          IconButton(
            onPressed: onRepeatToggle,
            icon: Icon(
              Icons.repeat,
              color: isRepeating ? Colors.white : Colors.grey.shade600,
              size: 28,
            ),
          ),
          IconButton(
            onPressed: () {
              context.read<SongsBloc>().add(PlayPrevious());
            },
            icon: Icon(Icons.skip_previous_outlined),
            color: Colors.white,
          ),
          IconButton(
            onPressed: () {
              context.read<SongsBloc>().add(TogglePlayPause());
            },
            icon: Icon(
              isPlaying ? Icons.pause : Icons.play_arrow,
              color: Colors.white,
              size: 48,
            ),
          ),

          IconButton(
            onPressed: () {
              context.read<SongsBloc>().add(PlayNext());
            },
            icon: Icon(Icons.skip_next_outlined),
            color: Colors.white,
          ),
          IconButton(
            onPressed: () {},
            icon: Icon(
              Icons.shuffle,
              color: isShuffled ? Colors.white : Colors.grey.shade600,
              size: 28,
            ),
          ),
        ],
      ),
    );
  }
}
