import 'package:flutter/material.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/src/commons/constants.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_menu.dart';
import 'package:widgets/mechanix.dart';

class PlaylistTile extends StatelessWidget {
  final PlaylistInfo playlistInfo;
  final bool isSelected;
  final bool isDisabled;
  final VoidCallback? onTap;
  final bool isMenuRequired;
  final ValueChanged<String> onRenameClick;
  final bool isActive;
  final bool isRenamePlaylist;
  final bool isDeletePlaylist;

  const PlaylistTile({
    super.key,
    required this.playlistInfo,
    this.isSelected = false,
    this.isDisabled = false,
    this.onTap,
    this.isMenuRequired = true,
    required this.onRenameClick,
    this.isActive = false,
    this.isRenamePlaylist = true,
    this.isDeletePlaylist = false,
  });

  @override
  Widget build(BuildContext context) {
    return Opacity(
      opacity: isDisabled ? 0.5 : 1.0,
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 16),
        color: isSelected ? context.secondaryContainer : Colors.transparent,
        child: ListTile(
          enabled: !isDisabled, //  disables ripple & gestures
          onTap: isDisabled ? null : onTap,
          contentPadding: const EdgeInsets.symmetric(vertical: 7.5),
          leading: _PlaylistCover(
            playlistInfo: playlistInfo,
            isDisabled: isDisabled,
          ),

          minVerticalPadding: 0,
          title: Text(
            playlistInfo.name,
            overflow: TextOverflow.ellipsis,
            style: TextStyle(
              fontWeight: FontWeight.w500,
              fontSize: 20,
              height: 1.25,
              color:
                  isActive
                      ? context.primary
                      : isDisabled
                      ? context.onSecondaryFixedVariant
                      : context.colorScheme.onSurface,
            ),
          ),
          subtitle:
              isDisabled &&
                      playlistInfo.songIds.length >=
                          Constants.maxSongsPerPlaylist
                  ? Text(
                    "Playlist limit reached (${Constants.maxSongsPerPlaylist} tracks max)",
                    overflow: TextOverflow.ellipsis,
                    style: TextStyle(
                      fontWeight: FontWeight.w400,
                      fontSize: 16,
                      color: Colors.redAccent,
                    ),
                  )
                  : Text(
                    "${playlistInfo.songIds.length} Tracks",
                    overflow: TextOverflow.ellipsis,
                    style: TextStyle(
                      fontWeight: FontWeight.w300,
                      fontSize: 18,
                      height: 1.25,
                      color: context.onSecondary,
                    ),
                  ),

          trailing:
              isMenuRequired
                  ? PlaylistMenu(
                    isDeletePlaylist: isDeletePlaylist,
                    isLiked: true,
                    isRenamePlaylist: isRenamePlaylist,
                    playlistInfo: playlistInfo,
                    onRenameClick: onRenameClick,
                  )
                  : null,
        ),
      ),
    );
  }
}

class _PlaylistCover extends StatelessWidget {
  final PlaylistInfo playlistInfo;
  final bool isDisabled;

  const _PlaylistCover({required this.playlistInfo, required this.isDisabled});

  bool get hasCover =>
      playlistInfo.coverImagePath != null &&
      playlistInfo.coverImagePath!.isNotEmpty;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 44,
      height: 44,

      child: Stack(
        fit: StackFit.expand,
        children: [
          Image.asset(MusicIcons.playlistCardIcon, fit: BoxFit.cover),

          //  Cover image overlay (only if exists, auto-fallback on error)
          if (hasCover)
            Image.asset(
              playlistInfo.coverImagePath!,
              fit: BoxFit.cover,
              errorBuilder: (_, __, ___) {
                return const SizedBox.shrink(); // fallback already below
              },
            ),
        ],
      ),
    );
  }
}
