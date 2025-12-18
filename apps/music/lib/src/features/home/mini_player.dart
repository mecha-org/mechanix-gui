import 'package:flutter/material.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/presentation/artwork_icon.dart';

class MiniPlayer extends StatelessWidget {
  final SongInfo currentSong;
  final VoidCallback? onTap;

  const MiniPlayer({super.key, required this.currentSong, this.onTap});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
      decoration: BoxDecoration(
        color: MusicColors.backgroundColor,
        borderRadius: BorderRadius.only(
          topLeft: Radius.circular(8),
          topRight: Radius.circular(8),
        ),
      ),
      child: Row(
        children: [
          Stack(
            alignment: Alignment.center,
            children: [
              RepaintBoundary(
                child: SizedBox(
                  width: 48,
                  height: 48,
                  child: Transform.scale(
                    scaleX: 1, // anti-clockwise
                    child: CircularProgressIndicator(
                      value: 20,
                      strokeWidth: 2.18,
                      backgroundColor: Colors.transparent,
                      valueColor: const AlwaysStoppedAnimation<Color>(
                        Colors.red,
                      ),
                    ),
                  ),
                ),
              ),
              ArtworkIcon(size: 35, artworkPath: currentSong.artworkPath),
            ],
          ),
          const SizedBox(width: 16),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  currentSong.title,
                  style: const TextStyle(
                    color: MusicColors.primaryTextColor,
                    fontSize: 18,
                    fontWeight: FontWeight.w500,
                  ),
                ),
                Text(
                  currentSong.artist,
                  style: const TextStyle(
                    color: MusicColors.secondaryTextColor,
                    fontSize: 16,
                    fontWeight: FontWeight.w300,
                  ),
                ),
              ],
            ),
          ),
          IconButton(
            onPressed: onTap,
            icon: Image.asset(MusicIcons.prevIcon, width: 20, height: 20),
          ),
          IconButton(
            onPressed: onTap,
            icon: Image.asset(MusicIcons.pauseIcon, width: 28, height: 28),
          ),
          IconButton(
            onPressed: onTap,
            icon: Image.asset(MusicIcons.nextIcon, width: 20, height: 20),
          ),
        ],
      ),
    );
  }
}
