import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/audio_player/audio_player.dart';
import 'package:mechanix_music/src/features/home/widgets/artwork_duration_border.dart';
import 'package:mechanix_music/src/features/presentation/artwork_icon.dart';
import 'package:tuple/tuple.dart';

class MiniPlayer extends StatelessWidget {
  final VoidCallback? onTap;

  const MiniPlayer({super.key, this.onTap});

  @override
  Widget build(BuildContext context) {
    return BlocSelector<SongsBloc, SongsState, Tuple2<bool, SongInfo?>>(
      selector: (state) => Tuple2(state.isPlaying, state.currentSong),
      builder: (context, state) {
        final bool isPlaying = state.item1;
        final SongInfo? currentSong = state.item2;

        if (currentSong == null) return const SizedBox.shrink();

        return Container(
          padding: const EdgeInsets.only(
            left: 12,
            right: 12,
            bottom: 8,
            top: 6,
          ),
          decoration: const BoxDecoration(
            color: MusicColors.backgroundColor,
            borderRadius: BorderRadius.only(
              topLeft: Radius.circular(8),
              topRight: Radius.circular(8),
            ),
          ),
          child: Row(
            children: [
              // Animated circular progress with artwork
              MouseRegion(
                cursor: SystemMouseCursors.click,
                child: GestureDetector(
                  onTap:
                      () => Navigator.push(
                        context,
                        MaterialPageRoute(
                          builder: (_) => AudioPlayer(songDetails: currentSong),
                        ),
                      ),
                  child: AnimatedCircularProgress(
                    player: context.read<SongsBloc>().player,
                    size: 52,
                    strokeWidth: 2.18,
                    progressColor: MusicColors.borderColor,
                    backgroundColor: Colors.transparent,
                    child: ArtworkIcon(
                      size: 35,
                      artworkPath: currentSong.artworkPath,
                    ),
                  ),
                ),
              ),

              const SizedBox(width: 16),

              Expanded(
                child: MouseRegion(
                  cursor: SystemMouseCursors.click,
                  child: GestureDetector(
                    onTap:
                        () => Navigator.push(
                          context,
                          MaterialPageRoute(
                            builder:
                                (_) => AudioPlayer(songDetails: currentSong),
                          ),
                        ),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          overflow: TextOverflow.ellipsis,
                          currentSong.title,
                          style: const TextStyle(
                            color: MusicColors.primaryTextColor,
                            fontSize: 18,
                            fontWeight: FontWeight.w500,
                          ),
                        ),
                        Text(
                          overflow: TextOverflow.ellipsis,
                          currentSong.artist,
                          style: const TextStyle(
                            color: MusicColors.secondaryTextColor,
                            fontSize: 16,
                            fontWeight: FontWeight.w300,
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              ),

              IconButton(
                onPressed:
                    () => {context.read<SongsBloc>().add(PlayPrevious())},
                icon: Image.asset(MusicIcons.prevIcon, width: 20, height: 20),
              ),

              IconButton(
                onPressed:
                    () => context.read<SongsBloc>().add(TogglePlayPause()),
                icon: Image.asset(
                  isPlaying ? MusicIcons.pauseIcon : MusicIcons.playIcon,
                  width: 28,
                  height: 28,
                ),
              ),

              IconButton(
                onPressed: () => {context.read<SongsBloc>().add(PlayNext())},
                icon: Image.asset(MusicIcons.nextIcon, width: 20, height: 20),
              ),
            ],
          ),
        );
      },
    );
  }
}
