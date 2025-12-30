import 'package:flutter/material.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_menu.dart';

class PlaylistCard extends StatelessWidget {
  final VoidCallback onPlaylistTap;
  final PlaylistInfo playlistInfo;
  final ValueChanged<String> onRenameClick;
  final bool isActive;
  const PlaylistCard({
    super.key,
    required this.playlistInfo,
    required this.onPlaylistTap,
    required this.onRenameClick,
    this.isActive = false,
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

        child: Container(
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(8),
            image: DecorationImage(
              image: AssetImage(
                hasCover
                    ? playlistInfo.coverImagePath!
                    : MusicIcons.playlistCardIcon,
              ),
              fit: BoxFit.cover,
            ),
            // : null,
          ),
          child: Stack(
            children: [
              Positioned(
                left: 12,
                bottom: 30,
                right: 40,
                child: Text(
                  playlistInfo.name,
                  overflow: TextOverflow.ellipsis,
                  style: TextStyle(
                    fontWeight: FontWeight.w500,
                    fontSize: 18,
                    height: 1.25,
                    color:
                        isActive
                            ? MusicColors.borderColor
                            : MusicColors.primaryTextColor,
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
                child: PlaylistMenu(
                  onRenameClick: onRenameClick,
                  playlistInfo: playlistInfo,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
