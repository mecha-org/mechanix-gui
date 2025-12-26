import 'package:flutter/material.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/constants.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_view/add_music_sheet.dart';
import 'package:widgets/widgets/icon_widget.dart';

class PlaylistAddSong extends StatelessWidget {
  final PlaylistInfo playlistInfo;

  const PlaylistAddSong({super.key, required this.playlistInfo});
  void _showAddMusicSheet(BuildContext context) {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      backgroundColor: Colors.transparent,
      builder: (context) => AddMusicSheet(playlistInfo: playlistInfo),
    );
  }

  @override
  Widget build(BuildContext context) {
    return SliverToBoxAdapter(
      child: Row(
        spacing: 20,
        children: [
          IconButton(
            iconSize: 44,
            onPressed:
                playlistInfo.songIds.length >= Constants.maxSongsPerPlaylist
                    ? null
                    : () {
                      _showAddMusicSheet(context);
                    },

            style: ButtonStyle(
              shape: WidgetStatePropertyAll(
                RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
              ),
              backgroundColor: WidgetStatePropertyAll(
                MusicColors.buttonBackgroundColor,
              ),
            ),
            icon: IconWidget(
              iconColor: MusicColors.borderColor,
              iconPath: MusicIcons.plusIcon,
              iconHeight: 24,
              boxHeight: 24,
              boxWidth: 24,
              iconWidth: 24,
            ),
          ),
          Text(
            "Add a song",
            style: TextStyle(
              fontSize: 18,
              color: MusicColors.disabledColor,
              height: 1.3,
              // letterSpacing: -1.1,
              fontWeight: FontWeight.w500,
            ),
          ),
        ],
      ),
    );
  }
}
