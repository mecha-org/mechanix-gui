import 'package:flutter/material.dart';

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
      padding: const EdgeInsets.symmetric(horizontal: 40),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          IconButton(
            onPressed: onFavoriteToggle,
            icon: Icon(
              isFavorited ? Icons.star : Icons.star_border,
              color: Colors.white,
              size: 28,
            ),
          ),
          const Icon(Icons.menu, color: Colors.white, size: 28),
        ],
      ),
    );
  }
}
