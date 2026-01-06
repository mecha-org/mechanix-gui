import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/presentation/song_tile.dart';

class SwipeableList extends StatelessWidget {
  final SongInfo song;
  final VoidCallback? onTap;
  final bool? isPlaying;
  const SwipeableList({
    super.key,
    required this.song,
    this.onTap,
    this.isPlaying,
  });

  @override
  Widget build(BuildContext context) {
    return BlocSelector<SongsBloc, SongsState, bool>(
      selector: (state) => state.currentSong?.id == song.id,
      builder: (context, isCurrentSong) {
        return Dismissible(
          key: ValueKey(song.id), // IMPORTANT: stable & unique key
          direction: DismissDirection.endToStart, // RIGHT → LEFT
          background: const SizedBox(), // disable opposite swipe
          secondaryBackground: _FavoriteBackground(
            isFavourite: song.isFavourite,
          ),
          dismissThresholds: const {
            DismissDirection.endToStart: 0.2, 
          },
          resizeDuration: Duration.zero,
          confirmDismiss: (direction) async {
            context.read<SongsBloc>().add(
              FavouriteToggle(
                isFavourite: !song.isFavourite,
                songIds: [song.id],
              ),
            );
            return false; // ⬅ snap back
          },

          child: SongTile(
            onTap: onTap,
            song: song,
            isCurrentSong: isPlaying ?? isCurrentSong,
          ),
        );
      },
    );
  }
}

class _FavoriteBackground extends StatelessWidget {
  final bool isFavourite;
  const _FavoriteBackground({required this.isFavourite});

  @override
  Widget build(BuildContext context) {
    return Container(
      alignment: Alignment.centerRight,
      padding: const EdgeInsets.only(right: 24),
      color: MusicColors.buttonBackgroundColor,
      child: Image.asset(
        isFavourite
            ? MusicIcons.filledFavouriteIcon
            : MusicIcons.favouritesIcon,
        width: 20,
        height: 20,
      ),
    );
  }
}
