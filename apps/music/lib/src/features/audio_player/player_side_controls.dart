import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/home/common/music_icon_widget.dart';
import 'package:mechanix_music/src/features/playlist_tab/add_to_playlist_sheet.dart';
import 'package:widgets/widgets/bottom_sheet_modals/mechanix_bottom_sheet.dart';

class PlayerSideControls extends StatelessWidget {
  final bool isFavorited;
  final SongInfo songDetails;

  const PlayerSideControls({
    super.key,
    required this.isFavorited,
    required this.songDetails,
  });
  void _showAddToPlaylistSheet(BuildContext context) {
    context.read<SongsBloc>().add(LoadPlaylist());

    MechanixBottomSheet.show(
      topTabWidth: 370,
      topTabRightSideShiftLength: 40,
      context,
      child: AddToPlaylistSheet(song: songDetails),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(left: 12, right: 12, bottom: 16),
      child: Row(
        spacing: 0,
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          MusicIconButton(
            onPressed: () {
              context.read<SongsBloc>().add(
                FavouriteToggle(
                  songIds: [songDetails.id],
                  isFavourite: !isFavorited,
                ),
              );
            },
            backgroundColor: Colors.transparent,
            icon:
                isFavorited
                    ? MusicIcons.filledFavouriteIcon
                    : MusicIcons.favouritesIcon,
            buttonSize: 44,
            iconSize: 24,
          ),

          StreamBuilder<Duration>(
            stream: context.read<SongsBloc>().player.stream.position,
            builder: (context, snapshot) {
              final position = snapshot.data ?? Duration.zero;

              String twoDigits(int n) => n.toString().padLeft(2, '0');
              final minutes = twoDigits(position.inMinutes.remainder(60));
              final seconds = twoDigits(position.inSeconds.remainder(60));

              return Container(
                padding: EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                decoration: BoxDecoration(
                  color: MusicColors.tapColor,
                  borderRadius: BorderRadius.circular(4),
                ),
                child: Text("$minutes:$seconds"),
              );
            },
          ),

          MusicIconButton(
            onPressed: ()=>_showAddToPlaylistSheet(context),

            backgroundColor: Colors.transparent,
            icon: MusicIcons.addSongIcon,
            iconSize: 24,
          ),
        ],
      ),
    );
  }
}
