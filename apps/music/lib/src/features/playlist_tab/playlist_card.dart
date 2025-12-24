import 'package:flutter/material.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_menu.dart';

class PlaylistCard extends StatelessWidget {
  final PlaylistInfo playlistInfo;

  const PlaylistCard({super.key, required this.playlistInfo});

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        Positioned.fill(
          child: Container(
            decoration: BoxDecoration(color: Colors.red.withValues(alpha: 0.3)),
          ),
        ),
        Center(child: Image.asset(MusicIcons.musicIcon, width: 60, height: 60)),
        Positioned(
          left: 12,
          bottom: 30,
          child: Text(
            playlistInfo.name,
            overflow: TextOverflow.ellipsis,
            style: TextStyle(
              fontWeight: FontWeight.w500,
              fontSize: 18,
              height: 1.25,
              color: MusicColors.primaryTextColor,
            ),
          ),
        ),
        Positioned(
          left: 12,
          bottom: 7,
          child: Text(
            "${playlistInfo.songIds.length.toString()} Tracks",
            overflow: TextOverflow.ellipsis,
            style: TextStyle(
              fontWeight: FontWeight.w300,
              fontSize: 16,
              height: 1.25,
              color: MusicColors.secondaryTextColor,
            ),
          ),
        ),
        Positioned(right: 4, child: PlaylistMenu(playlistInfo: playlistInfo)),
      ],
    );
  }
}
