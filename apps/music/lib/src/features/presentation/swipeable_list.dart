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
  const SwipeableList({super.key, required this.song});

  @override
  Widget build(BuildContext context) {
    return BlocSelector<SongsBloc, SongsState, bool>(
      selector: (state) => state.currentSong?.id == song.id,
      builder: (context, isCurrentSong) {
        return Dismissible(
          key: ValueKey(song.id), // IMPORTANT: stable & unique key
          direction: DismissDirection.endToStart, // RIGHT → LEFT
          background: const SizedBox(), // disable opposite swipe
          secondaryBackground: _DeleteBackground(),
          confirmDismiss: (direction) async {
            // Optional confirmation (recommended)
            return true;
          },
          onDismissed:
              isCurrentSong
                  ? (_) => {context.read<SongsBloc>().add(DeleteSong(song))}
                  : null,
          child: SongTile(
            song: song,
            isCurrentSong: isCurrentSong,
            isDeleteMenu: true,
          ),
        );
      },
    );
  }
}

class _DeleteBackground extends StatelessWidget {
  const _DeleteBackground();

  @override
  Widget build(BuildContext context) {
    return Container(
      alignment: Alignment.centerRight,
      padding: const EdgeInsets.only(right: 24),
      color: MusicColors.buttonBackgroundColor,
      child: Image.asset(MusicIcons.swipeDeleteIcon, width: 20, height: 20),
    );
  }
}
