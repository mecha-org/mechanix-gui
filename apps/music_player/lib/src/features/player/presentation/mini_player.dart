import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:media_kit/media_kit.dart';
import '../data/music_player_manager.dart';
import 'repeat_button.dart';

class MiniPlayer extends StatelessWidget {
  final bool isPlaying;
  final Duration position;
  final Duration duration;
  final int currentIndex;
  final VoidCallback? onTap;

  const MiniPlayer({
    super.key,
    required this.isPlaying,
    required this.position,
    required this.duration,
    required this.currentIndex,
    this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    final manager = MusicPlayerManager.instance;
    final player = manager.player;

    final playbackQueue = manager.playbackQueue;
    final song =
        (currentIndex >= 0 && currentIndex < playbackQueue.length)
            ? playbackQueue[currentIndex]
            : null;

    if (song == null) {
      return const SizedBox.shrink(); // Hide mini player if no song
    }

    final displayTitle = song.title.isNotEmpty ? song.title : 'Unknown';
    final displayArtist = song.artist.isNotEmpty ? song.artist : 'Unknown';

    return Container(
      padding: const EdgeInsets.all(8),
      margin: const EdgeInsets.only(left: 60, right: 60, top: 0, bottom: 20),
      decoration: BoxDecoration(
        // color: Theme.of(context).primaryColor,
        color: const Color.fromARGB(255, 54, 52, 52),
        borderRadius: BorderRadius.circular(12), // Rounded corners
      ),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          // Song title row
          Row(
            children: [
              buildArtworkOrIcon(
                artwork: song.artwork,
                fallbackIcon: Icons.music_note,
                isSelected: true,
                context: context,
              ),
              const SizedBox(width: 10),
              Expanded(
                // child: Text(
                //   "$displayTitle — $displayArtist",
                //   style: const TextStyle(color: Colors.white),
                //   overflow: TextOverflow.ellipsis,
                // ),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      displayTitle,
                      style: const TextStyle(
                        color: Colors.white,
                        fontWeight: FontWeight.bold,
                      ),
                      overflow: TextOverflow.ellipsis,
                    ),
                    Text(
                      displayArtist,
                      style: const TextStyle(
                        color: Colors.white70,
                        fontSize: 12,
                      ),
                      overflow: TextOverflow.ellipsis,
                    ),
                  ],
                ),
              ),
                  IconButton(
                icon: Icon(
                  isPlaying ? Icons.pause : Icons.play_arrow_outlined,
                  color: Colors.white,
                ),
                onPressed: () => isPlaying ? player.pause() : player.play(),
              ),
            ],
          ),

          // // Progress bar
          // Slider(
          //   activeColor: Colors.white,
          //   thumbColor: Colors.white,
          //   inactiveColor: Colors.white24,
          //   min: 0,
          //   max: duration.inMilliseconds.toDouble().clamp(1, double.infinity),
          //   value: position.inMilliseconds.toDouble().clamp(
          //     0,
          //     duration.inMilliseconds.toDouble(),
          //   ),
          //   onChanged: (value) {
          //     player.seek(Duration(milliseconds: value.toInt()));
          //   },
          // ),

          // // Controls row
          // Row(
          //   mainAxisAlignment: MainAxisAlignment.spaceBetween,
          //   children: [
          //     Text(
          //       _formatDuration(position),
          //       style: const TextStyle(color: Colors.white, fontSize: 12),
          //     ),
          //     IconButton(
          //       icon: const Icon(Icons.skip_previous, color: Colors.white),
          //       onPressed: () async {
          //         final mode = player.state.playlistMode;
          //         if (mode == PlaylistMode.single) {
          //           // Don’t skip, just restart current track
          //           await player.seek(Duration.zero);
          //         } else {
          //           await player.previous();
          //         }
          //       },
          //     ),

          //     IconButton(
          //       icon: Icon(
          //         isPlaying ? Icons.pause : Icons.play_arrow,
          //         color: Colors.white,
          //       ),
          //       onPressed: () => isPlaying ? player.pause() : player.play(),
          //     ),
          //     IconButton(
          //       icon: const Icon(Icons.skip_next, color: Colors.white),
          //       onPressed: () async {
          //         final mode = player.state.playlistMode;
          //         if (mode == PlaylistMode.single) {
          //           // Don’t skip, just restart current track
          //           await player.seek(Duration.zero);
          //         } else {
          //           await player.next();
          //         }
          //       },
          //     ),
          //     RepeatButton(player: player),
          //     Text(
          //       _formatDuration(duration),
          //       style: const TextStyle(color: Colors.white, fontSize: 12),
          //     ),
          //   ],
          // ),
        ],
      ),
    );
  }

  String _formatDuration(Duration d) =>
      "${d.inMinutes}:${(d.inSeconds % 60).toString().padLeft(2, '0')}";

  Widget buildArtworkOrIcon({
    required Uint8List? artwork,
    required IconData fallbackIcon,
    required bool isSelected,
    double size = 40,
    double radius = 50,
    required BuildContext context,
  }) {
    if (artwork != null) {
      return ClipRRect(
        borderRadius: BorderRadius.circular(radius),
        child: Image.memory(
          artwork,
          width: size,
          height: size,
          fit: BoxFit.cover,
        ),
      );
    } else {
      return Container(
        width: size,
        height: size,
        decoration: BoxDecoration(
          color: Colors.grey.shade200,
          borderRadius: BorderRadius.circular(radius),
          border: Border.all(
            color:
                isSelected
                    ? Theme.of(context).secondaryHeaderColor
                    : Colors.grey,
            width: 1.5,
          ),
        ),
        child: Icon(
          fallbackIcon,
          size: size * 0.6,
          color:
              isSelected
                  ? Theme.of(context).secondaryHeaderColor
                  : Colors.grey.shade600,
        ),
      );
    }
  }
}
