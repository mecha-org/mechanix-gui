import 'package:flutter/material.dart';
import 'package:mechanix_music/src/commons/icons.dart';

class ArtworkIcon extends StatelessWidget {
  final String? artworkPath;
  final double? size;
  const ArtworkIcon({super.key, this.artworkPath, this.size = 44});
  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(50),
      child: Image.asset(
        artworkPath ?? MusicIcons.audioImage,
        width: size,
        height: size,
        errorBuilder: (context, error, stackTrace) {
          return Image.asset(
            MusicIcons.audioImage,
            fit: BoxFit.cover,
            width: size,
            height: size,
          );
        },
      ),
    );
  }
}
