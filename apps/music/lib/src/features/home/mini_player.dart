import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/audio_player/audio_player.dart';
import 'package:mechanix_music/src/features/home/widgets/artwork_duration_border.dart';
import 'package:mechanix_music/src/features/presentation/artwork_icon.dart';
import 'package:mechanix_music/src/features/presentation/songs_icon.dart';
import 'package:tuple/tuple.dart';
import 'package:widgets/mechanix.dart';

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
          decoration: BoxDecoration(
            color: context.secondary,
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
                    progressColor: context.primaryContainer,
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
                    behavior: HitTestBehavior.translucent,
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
                          style: TextStyle(
                            color: context.onSecondaryFixedVariant,
                            fontSize: 18,
                            fontWeight: FontWeight.w500,
                          ),
                        ),
                        Text(
                          overflow: TextOverflow.ellipsis,
                          currentSong.artist,
                          style: TextStyle(
                            color: context.colorScheme.onSecondary,
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
                icon: SongsIcon(
                  iconPath: MusicIcons.prevIcon,
                  iconSize: 20,
                  boxSize: 20,
                ),
              ),

              IconButton(
                onPressed:
                    () => context.read<SongsBloc>().add(TogglePlayPause()),
                icon: SongsIcon(
                  iconPath:
                      isPlaying ? MusicIcons.pauseIcon : MusicIcons.playIcon,
                  iconSize: 28,
                  boxSize: 28,
                ),
              ),

              IconButton(
                onPressed: () => {context.read<SongsBloc>().add(PlayNext())},
                icon: SongsIcon(
                  iconPath: MusicIcons.nextIcon,
                  iconSize: 20,
                  boxSize: 20,
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}
