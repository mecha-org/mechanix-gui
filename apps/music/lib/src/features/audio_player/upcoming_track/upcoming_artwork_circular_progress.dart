import 'package:flutter/material.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/features/home/widgets/artwork_duration_border.dart';
import 'package:media_kit/media_kit.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/features/presentation/artwork_icon.dart';

class UpcomingArtworkCircularProgress extends StatelessWidget {
  final SongInfo songInfo;
  final Player player;

  const UpcomingArtworkCircularProgress({
    super.key,
    required this.songInfo,
    required this.player,
  });

  @override
  Widget build(BuildContext context) {
    return AnimatedCircularProgress(
      player: player,
      size: 115,
      strokeWidth: 3,
      progressColor: MusicColors.borderColor,
      child: ArtworkIcon(artworkPath: songInfo.artworkPath, size: 100),
    );
  }
}
