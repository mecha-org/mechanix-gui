import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';

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
                  Image.asset(MusicIcons.audioImage, width: 152, height: 160),
                  Positioned(
                    left: 72,
                    child: Container(
                      width: 160,
                      height: 160,
                      decoration: BoxDecoration(
                        borderRadius: BorderRadius.circular(8),
                        color: hasCoverImage ? null : MusicColors.albumColor,
                        image:
                            hasCoverImage
                                ? DecorationImage(
                                  image: AssetImage(
                                    playlistInfo.coverImagePath!,
                                  ),
                                  fit: BoxFit.cover,
                                )
                                : null,
                      ),
                      child: Center(
                        child: Image.asset(
                          MusicIcons.plusIcon,
                          width: 24,
                          height: 24,
                          color: Colors.white,
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
                        style: const TextStyle(
                          fontSize: 18,
                          color: MusicColors.primaryTextColor,
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
                            fontSize: 16,
                            height: 1.35,
                            color: MusicColors.textColor,
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
