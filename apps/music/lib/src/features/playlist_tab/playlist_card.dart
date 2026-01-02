import 'package:flutter/material.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_menu.dart';

class PlaylistCard extends StatelessWidget {
  final VoidCallback onPlaylistTap;
  final PlaylistInfo playlistInfo;
  final ValueChanged<String> onRenameClick;
  final bool isDeletePlaylist;
  final bool isRenamePlaylist;
  final bool isLiked;
  final bool isActive;

  const PlaylistCard({
    super.key,
    required this.playlistInfo,
    required this.onPlaylistTap,
    required this.onRenameClick,
    this.isActive = false,
    this.isDeletePlaylist = true,
    this.isRenamePlaylist = true,
    this.isLiked = true,
  });

  bool get hasCover =>
      playlistInfo.coverImagePath != null &&
      playlistInfo.coverImagePath!.isNotEmpty;

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      cursor: SystemMouseCursors.click,
      child: GestureDetector(
        onTap: onPlaylistTap,
        child: ClipRRect(
          borderRadius: BorderRadius.circular(8),
          child: Stack(
            fit: StackFit.expand,
            children: [
              /// 🎨 COVER IMAGE
              Image.asset(
                hasCover
                    ? playlistInfo.coverImagePath!
                    : MusicIcons.playlistCardIcon,
                fit: BoxFit.cover,
              ),

              ///  OVERLAY
              const _TextContrastOverlay(),

              ///  TEXT CONTENT
              _CardContent(playlistInfo: playlistInfo, isActive: isActive),

              /// ☰ MENU
              Positioned(
                right: 4,
                top: 4,
                child: PlaylistMenu(
                  isDeletePlaylist: isDeletePlaylist,
                  isRenamePlaylist: isRenamePlaylist,
                  isLiked: isLiked,
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

class _TextContrastOverlay extends StatelessWidget {
  const _TextContrastOverlay();

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: const BoxDecoration(
        gradient: LinearGradient(
          begin: Alignment.topCenter,
          end: Alignment.bottomCenter,
          colors: [
            Colors.black26, // top shadow
            Colors.transparent,
            Colors.black45, // bottom shadow for text
          ],
          stops: [1, 0.4, 1],
        ),
      ),
    );
  }
}

class _CardContent extends StatelessWidget {
  final PlaylistInfo playlistInfo;
  final bool isActive;

  const _CardContent({required this.playlistInfo, required this.isActive});

  @override
  Widget build(BuildContext context) {
    return Positioned(
      left: 12,
      right: 40,
      bottom: 8,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisSize: MainAxisSize.min,
        children: [
          Text(
            playlistInfo.name,
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
            style: TextStyle(
              fontWeight: FontWeight.w600,
              fontSize: 18,
              height: 1.25,
              color:
                  isActive
                      ? MusicColors.borderColor
                      : Colors.white, // force readable
            ),
          ),
          const SizedBox(height: 2),
          Text(
            '${playlistInfo.songIds.length} Tracks',
            style: const TextStyle(
              fontWeight: FontWeight.w400,
              fontSize: 14,
              color: Colors.white70,
            ),
          ),
        ],
      ),
    );
  }
}
