import 'package:flutter/material.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/presentation/songs_icon.dart';

class PlayerSideControls extends StatelessWidget {
  final bool isFavorited;
  final VoidCallback onFavoriteToggle;

  const PlayerSideControls({
    super.key,
    required this.isFavorited,
    required this.onFavoriteToggle,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 40,vertical: 0),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          IconButton(
            onPressed: onFavoriteToggle,
            icon: SongsIcon(
              iconPath:
                  isFavorited
                      ? MusicIcons.filledFavouriteIcon
                      : MusicIcons.favouriteIcon,
              color: isFavorited ? Colors.red : Colors.white,
            ),
          ),
          const Icon(Icons.menu, color: Colors.white, size: 28),
        ],
      ),
    );
  }
}
