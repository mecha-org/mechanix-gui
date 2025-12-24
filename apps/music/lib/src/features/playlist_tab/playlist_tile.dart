import 'package:flutter/material.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_menu.dart';

class PlaylistTile extends StatelessWidget {
  final PlaylistInfo playlistInfo;
  final bool isSelected;
  final bool isDisabled;
  final VoidCallback? onTap;

  const PlaylistTile({
    super.key,
    required this.playlistInfo,
    this.isSelected = false,
    this.isDisabled = false,
    this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return Opacity(
      opacity: isDisabled ? 0.5 : 1.0,
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 16),
        color: isSelected ? MusicColors.backgroundColor : Colors.transparent,
        child: ListTile(
          enabled: !isDisabled, // 👈 disables ripple & gestures
          onTap: isDisabled ? null : onTap,

          contentPadding: const EdgeInsets.symmetric(vertical: 7.5),
          leading: Container(
            width: 44,
            height: 44,
            padding: const EdgeInsets.all(8),
            decoration: BoxDecoration(
              color: isDisabled ? MusicColors.dividerColor : Colors.red,
              borderRadius: const BorderRadius.all(Radius.circular(4)),
            ),
            child: Image.asset(
              MusicIcons.musicIcon,
              width: 28,
              height: 28,
              color: isDisabled ? MusicColors.secondaryTextColor : null,
            ),
          ),
          minVerticalPadding: 0,
          title: Text(
            playlistInfo.name,
            overflow: TextOverflow.ellipsis,
            style: TextStyle(
              fontWeight: FontWeight.w500,
              fontSize: 18,
              height: 1.25,
              color:
                  isDisabled
                      ? MusicColors.secondaryTextColor
                      : MusicColors.primaryTextColor,
            ),
          ),
          subtitle: Text(
            "${playlistInfo.songIds.length} Tracks",
            overflow: TextOverflow.ellipsis,
            style: TextStyle(
              fontWeight: FontWeight.w300,
              fontSize: 16,
              height: 1.25,
              color: MusicColors.secondaryTextColor,
            ),
          ),
          trailing: PlaylistMenu(playlistInfo: playlistInfo),
        ),
      ),
    );
  }
}
