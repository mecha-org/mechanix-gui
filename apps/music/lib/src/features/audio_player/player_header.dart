import 'package:flutter/material.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/commons/colors.dart';

class PlayerHeader extends StatelessWidget {
  final SongInfo songDetails;
  final bool isFavorited;
  const PlayerHeader({
    super.key,
    required this.songDetails,
    required this.isFavorited,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.only(top: 20, left: 12, right: 12),
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
