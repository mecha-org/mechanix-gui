import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/features/audio_player/audio_player.dart';
import 'package:mechanix_music/src/features/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/features/bloc/songs_event.dart';

class MiniPlayer extends StatelessWidget {
  final bool isPlaying;
  final int currentIndex;
  final SongInfo currentSong;
  final VoidCallback? onTap;
  final Duration? currentPosition; // Add this parameter
  final Duration? totalDuration; // Add this parameter

  const MiniPlayer({
    super.key,
    required this.isPlaying,
    required this.currentSong,
    required this.currentIndex,
    this.onTap,
    this.currentPosition,
    this.totalDuration,
  });

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap:
          onTap ??
          () {
            Navigator.push(
              context,
              MaterialPageRoute(
                builder: (_) => AudioPlayer(songDetails: currentSong),
              ),
            );
          },
      child: Container(
        padding: const EdgeInsets.all(8),
        margin: const EdgeInsets.only(left: 60, right: 60, top: 0, bottom: 20),
        decoration: BoxDecoration(
          color: const Color.fromARGB(255, 54, 52, 52),
          borderRadius: BorderRadius.circular(12),
        ),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            // Song title row
            Row(
              children: [
                buildArtworkOrIcon(
                  artwork: currentSong.artwork,
                  fallbackIcon: Icons.music_note,
                  isSelected: true,
                  context: context,
                ),
                const SizedBox(width: 10),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        currentSong.title,
                        style: const TextStyle(
                          color: Colors.white,
                          fontWeight: FontWeight.bold,
                        ),
                        overflow: TextOverflow.ellipsis,
                      ),
                      Text(
                        currentSong.artist,
                        style: const TextStyle(
                          color: Colors.white70,
                          fontSize: 12,
                        ),
                        overflow: TextOverflow.ellipsis,
                      ),
                      // Duration text
                      // if (currentPosition != null && totalDuration != null)
                      //   Padding(
                      //     padding: const EdgeInsets.only(top: 2),
                      //     child: Text(
                      //       "${_formatDuration(currentPosition!)} / ${_formatDuration(totalDuration!)}",
                      //       style: const TextStyle(
                      //         color: Colors.white54,
                      //         fontSize: 10,
                      //       ),
                      //     ),
                      //   ),
                    ],
                  ),
                ),
                IconButton(
                  icon: Icon(
                    isPlaying ? Icons.pause : Icons.play_arrow_outlined,
                    color: Colors.white,
                  ),
                  onPressed: () {
                    context.read<SongsBloc>().add(TogglePlayPause());
                  },
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget buildArtworkOrIcon({
    Uint8List? artwork,
    required IconData fallbackIcon,
    required bool isSelected,
    double size = 40,
    double radius = 50,
    required BuildContext context,
  }) {
    // Calculate progress value
    double progress = 0.0;
    if (currentPosition != null &&
        totalDuration != null &&
        totalDuration!.inMilliseconds > 0) {
      progress =
          currentPosition!.inMilliseconds / totalDuration!.inMilliseconds;
    }

    return SizedBox(
      width: size + 8, // Add space for progress border
      height: size + 8,
      child: Stack(
        alignment: Alignment.center,
        children: [
          // Circular progress indicator (anti-clockwise)
          SizedBox(
            width: size + 6,
            height: size + 6,
            child: Transform.scale(
              scaleX: -1, // Flip horizontally to make it anti-clockwise
              child: CircularProgressIndicator(
                value: progress,
                strokeWidth: 3,
                backgroundColor: Colors.white.withOpacity(0.3),
                valueColor: const AlwaysStoppedAnimation<Color>(Colors.white),
              ),
            ),
          ),
          // Artwork or icon
          if (artwork != null)
            ClipRRect(
              borderRadius: BorderRadius.circular(radius),
              child: Image.memory(
                artwork,
                width: size,
                height: size,
                fit: BoxFit.cover,
              ),
            )
          else
            Container(
              width: size,
              height: size,
              decoration: BoxDecoration(
                color: Colors.grey.shade200,
                borderRadius: BorderRadius.circular(radius),
              ),
              child: Icon(
                fallbackIcon,
                size: size * 0.6,
                color: Colors.grey.shade600,
              ),
            ),
        ],
      ),
    );
  }

  // String _formatDuration(Duration duration) {
  //   String twoDigits(int n) => n.toString().padLeft(2, '0');
  //   final minutes = twoDigits(duration.inMinutes.remainder(60));
  //   final seconds = twoDigits(duration.inSeconds.remainder(60));
  //   return "$minutes:$seconds";
  // }
}
