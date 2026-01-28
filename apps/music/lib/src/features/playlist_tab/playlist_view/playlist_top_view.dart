import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:widgets/extension.dart';

class PlaylistTopView extends StatelessWidget {
  final PlaylistInfo playlistInfo;

  const PlaylistTopView({super.key, required this.playlistInfo});

  @override
  Widget build(BuildContext context) {
    final hasCoverImage =
        playlistInfo.coverImagePath != null &&
        playlistInfo.coverImagePath!.isNotEmpty;
    return SliverPadding(
      padding: EdgeInsets.symmetric(horizontal: 12),
      sliver: SliverToBoxAdapter(
        child: Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Expanded(
              child: Stack(
                clipBehavior: Clip.none,
                children: [
                  Image.asset(
                    MusicIcons.halfSliderIcon,
                    width: 75,
                    height: 152,
                  ),
                  Positioned(
                    left: 72,
                    child: ClipRRect(
                      borderRadius: BorderRadius.circular(8),
                      child: SizedBox(
                        width: 160,
                        height: 160,
                        child: Stack(
                          fit: StackFit.expand,
                          children: [
                            // Image layer
                            Image.asset(
                              hasCoverImage
                                  ? playlistInfo.coverImagePath!
                                  : MusicIcons.playlistCardIcon,
                              fit: BoxFit.cover,
                            ),

                            // Gradient overlay
                            Container(
                              decoration: const BoxDecoration(
                                gradient: LinearGradient(
                                  begin: Alignment.topCenter,
                                  end: Alignment.bottomCenter,
                                  colors: [
                                    Colors.black26, // top - light
                                    Colors.black38, // middle
                                    Colors.black54, // bottom - darker
                                  ],
                                  stops: [0.0, 0.55, 1.0],
                                ),
                              ),
                            ),

                            // Plus icon
                            Center(
                              child: Image.asset(
                                MusicIcons.plusIcon,
                                width: 24,
                                height: 24,
                                color: Colors.white,
                              ),
                            ),
                          ],
                        ),
                      ),
                    ),
                  ),
                ],
              ),
            ),
            BlocSelector<SongsBloc, SongsState, List<SongInfo>>(
              selector: (state) => state.playlistSongs,
              builder: (context, playlistSongs) {
                final totalSeconds = calculateTotalSeconds(playlistSongs);
                final totalDuration = formatTotalMinutes(totalSeconds);
                final artists =
                    playlistSongs
                        .map((e) => e.artist.trim())
                        .where((e) => e.isNotEmpty)
                        .toSet();

                return Expanded(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    crossAxisAlignment: CrossAxisAlignment.center,
                    children: [
                      Text(
                        "${playlistSongs.length} tracks, $totalDuration",
                        style: TextStyle(
                          fontSize: 20,
                          color: context.colorScheme.onSurface,
                          height: 1.35,
                        ),
                      ),
                      const SizedBox(height: 12),
                      if (artists.isNotEmpty)
                        Text(
                          artists.first,
                          maxLines: 1,
                          overflow: TextOverflow.ellipsis,
                          style: TextStyle(
                            fontSize: 18,
                            height: 1.35,
                            color: context.onSurfaceVariant,
                          ),
                        ),
                    ],
                  ),
                );
              },
            ),
          ],
        ),
      ),
    );
  }
}

int calculateTotalSeconds(List<SongInfo> songs) {
  return songs.fold<int>(0, (total, song) {
    final seconds = int.tryParse(song.duration ?? '');
    return total + (seconds ?? 0);
  });
}

String formatTotalMinutes(int totalSeconds) {
  final minutes = totalSeconds ~/ 60;
  return '$minutes min';
}
