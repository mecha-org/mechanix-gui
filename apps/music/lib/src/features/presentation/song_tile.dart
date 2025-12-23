import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/features/presentation/artwork_icon.dart';
import 'package:mechanix_music/src/features/presentation/song_menu.dart';

class SongTile extends StatelessWidget {
  final SongInfo song;
  final bool? enableSwipeDelete;
  final bool isCurrentSong;
  const SongTile({
    super.key,
    required this.song,
    this.enableSwipeDelete = true,
    this.isCurrentSong = false,
  });

  @override
  Widget build(BuildContext context) {
    return ListTile(
      onTap: () => {context.read<SongsBloc>().add(PlaySong(song))},
      contentPadding: const EdgeInsets.symmetric(vertical: 7.5),
      minVerticalPadding: 0,

      leading: ArtworkIcon(artworkPath: song.artworkPath),

      title: Container(
        margin: const EdgeInsets.only(bottom: 4),
        child: Text(
          song.title,
          overflow: TextOverflow.ellipsis,
          style: TextStyle(
            fontWeight: FontWeight.w500,
            fontSize: 18,
            height: 1.25,
            color:
                isCurrentSong
                    ? MusicColors.borderColor
                    : MusicColors.primaryTextColor,
          ),
        ),
      ),

      subtitle: Text(
        song.artist,
        overflow: TextOverflow.ellipsis,
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
          // if (isCurrentSong)
          //   SegmentedBarEqualizer(color: MusicColors.titleColor),
          const SizedBox(width: 12),
          SongMenu(
            song: song,
            onToggleFavourite:
                () => {context.read<SongsBloc>().add(FavouriteToggle(song))},
          ),
        ],
      ),
    );
  }
}
