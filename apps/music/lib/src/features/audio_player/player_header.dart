import 'package:flutter/material.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/commons/colors.dart';

class PlayerHeader extends StatelessWidget {
  final SongInfo songDetails;
  const PlayerHeader({
    super.key,
    required this.songDetails,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.only(top: 20, left: 30, right: 30),
      alignment: Alignment.center,
      child: Column(
        children: [
          Text(
            '${songDetails.title}${songDetails.album != null ? ' (${songDetails.album})' : ''}',
            style: const TextStyle(
              color: MusicColors.headingTextColor,
              fontSize: 24,
              height: 1.25,
              fontWeight: FontWeight.w500,
            ),
            textAlign: TextAlign.center,
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
          ),
          const SizedBox(height: 4),
          Text(
            songDetails.artist,
            style: const TextStyle(
              color: MusicColors.headingTextColor,
              fontSize: 16,
              height: 1.25,
            ),
            textAlign: TextAlign.center,
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
          ),
        ],
      ),
    );
  }
}
