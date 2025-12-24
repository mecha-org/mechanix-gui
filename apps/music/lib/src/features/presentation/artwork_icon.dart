import 'package:flutter/material.dart';
import 'package:mechanix_music/src/commons/icons.dart';

class ArtworkIcon extends StatelessWidget {
  final String? artworkPath;
  final double? size;
  const ArtworkIcon({super.key, this.artworkPath, this.size = 44});
  @override
  Widget build(BuildContext context) {
    if (artworkPath != null) {
      return ClipRRect(
        borderRadius: BorderRadius.circular(50),
        child: Image.asset(artworkPath!, width: size, height: size),
      );
    } else {
      return Container(
        width: size,
        height: size,
        decoration: BoxDecoration(
          color: Colors.grey.shade200,
          borderRadius: BorderRadius.circular(50),
        ),
        child: Image.asset(MusicIcons.audioImage),
      );
    }
  }
}
