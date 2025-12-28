import 'package:flutter/material.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_menu.dart';

class PlaylistCard extends StatelessWidget {
  final VoidCallback onPlaylistTap;
  final PlaylistInfo playlistInfo;

  const PlaylistCard({
    super.key,
    required this.playlistInfo,
    required this.onPlaylistTap,
  });

  bool get hasCover =>
      playlistInfo.coverImagePath != null &&
      playlistInfo.coverImagePath!.isNotEmpty;

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      cursor: SystemMouseCursors.click,
      child: GestureDetector(
        onTap: () => onPlaylistTap(),
        // () => Navigator.push(
        //   context,
        //   MaterialPageRoute(
        //     builder: (context) => PlaylistView(playlistInfo: playlistInfo),
        //   ),
        // ),
        child: Container(
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(8),
            color: hasCover ? null : Colors.red.withValues(alpha: 0.3),
            image:
                hasCover
                    ? DecorationImage(
                      image: AssetImage(playlistInfo.coverImagePath!),
                      fit: BoxFit.cover, // 🔥 fills entire container
                    )
                    : null,
          ),
          child: Stack(
            children: [
              // Fallback icon ONLY when cover is missing
              if (!hasCover)
                Center(
                  child: Image.asset(
                    MusicIcons.musicIcon,
                    width: 60,
                    height: 60,
                  ),
                ),

              Positioned(
                left: 12,
                bottom: 30,
                right: 40,
                child: Text(
                  playlistInfo.name,
                  overflow: TextOverflow.ellipsis,
                  style: const TextStyle(
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
                  "${playlistInfo.songIds.length} Tracks",
                  style: const TextStyle(
                    fontWeight: FontWeight.w300,
                    fontSize: 16,
                    height: 1.25,
                    color: MusicColors.secondaryTextColor,
                  ),
                ),
              ),

              Positioned(
                right: 4,
                top: 4,
                child: PlaylistMenu(playlistInfo: playlistInfo),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
