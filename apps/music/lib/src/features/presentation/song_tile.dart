import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/features/audio_player/audio_player.dart';
import 'package:mechanix_music/src/features/presentation/artwork_icon.dart';
import 'package:mechanix_music/src/features/presentation/equalizer.dart';
import 'package:mechanix_music/src/features/presentation/song_menu.dart';

class SongTile extends StatelessWidget {
  final SongInfo song;
  final bool? enableSwipeDelete;
  final bool isCurrentSong;
  final bool isDisabled;
  final bool isSelected;
  final VoidCallback? onTap;
  final bool isPaddingRequired;
  final bool isEditMode;
  final bool isMenuRequired;
  const SongTile({
    super.key,
    required this.song,
    this.enableSwipeDelete = true,
    this.isCurrentSong = false,
    this.isDisabled = false,
    this.isSelected = false,
    this.isPaddingRequired = false,
    this.onTap,
    this.isEditMode = false,
    this.isMenuRequired = true,
  });

  @override
  Widget build(BuildContext context) {
    return Opacity(
      opacity: isDisabled ? 0.5 : 1.0,
      child: Container(
        padding:
            isPaddingRequired
                ? EdgeInsets.symmetric(horizontal: 16)
                : EdgeInsets.all(0),
        color: isSelected ? MusicColors.backgroundColor : Colors.transparent,
        child: ListTile(
          onTap:
              isDisabled || isEditMode
                  ? null
                  : onTap ??
                      () => {
                        context.read<SongsBloc>().add(PlaySong(song)),
                        Navigator.push(
                          context,
                          MaterialPageRoute(
                            builder: (_) => AudioPlayer(songDetails: song),
                          ),
                        ),
                      },
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

          trailing:
              isEditMode
                  ? null
                  : isMenuRequired
                  ? Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      if (isCurrentSong)
                        BlocSelector<SongsBloc, SongsState, bool>(
                          selector: (state) => state.isPlaying,
                          builder:
                              (context, isPlaying) => SegmentedBarEqualizer(
                                isPlaying: isPlaying,
                                color: MusicColors.titleColor,
                              ),
                        ),
                      const SizedBox(width: 12),

                      SongMenu(
                        song: song,
                        onToggleFavourite:
                            isDisabled
                                ? null
                                : () => {
                                  context.read<SongsBloc>().add(
                                    FavouriteToggle(
                                      songIds: [song.id],
                                      isFavourite: !song.isFavourite,
                                    ),
                                  ),
                                },
                      ),
                    ],
                  )
                  : null,
        ),
      ),
    );
  }
}
