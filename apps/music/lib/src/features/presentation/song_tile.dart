import 'package:flutter/material.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/features/presentation/artwork_icon.dart';
import 'package:mechanix_music/src/features/presentation/equalizer.dart';
import 'package:mechanix_music/src/features/presentation/song_menu.dart';

class SongTile extends StatelessWidget {
  final SongInfo song;
  const SongTile({super.key, required this.song});

  @override
  Widget build(BuildContext context) {
    return ListTile(
      contentPadding: EdgeInsets.symmetric(vertical: 7.5),
      minVerticalPadding: 0,
      leading: ArtworkIcon(artworkPath: song.artworkPath),
      title: Container(
        margin: EdgeInsets.only(bottom: 4),
        child: Text(
          song.title,
          style: TextStyle(
            fontWeight: FontWeight.w500,
            fontSize: 18,
            height: 1.25,
            color: MusicColors.primaryTextColor,
          ),
        ),
      ),

      subtitle: Text(
        song.artist,
        style: TextStyle(
          fontWeight: FontWeight.w300,
          fontSize: 16,
          height: 1.25,
          color: MusicColors.secondaryTextColor,
        ),
      ),
      trailing: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          SegmentedBarEqualizer(
            color: MusicColors.deleteColor, // yellow
          ),
          const SizedBox(width: 12),
          SongMenu(),
        ],
      ),
    );
  }
}
