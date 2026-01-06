import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/audio_player/upcoming_track/upcoming_artwork_circular_progress.dart';
import 'package:mechanix_music/src/features/audio_player/upcoming_track/upcoming_song_progress_bar.dart';
import 'package:mechanix_music/src/features/home/common/music_icon_widget.dart';
import 'package:mechanix_music/src/features/playlist_tab/add_to_playlist_sheet.dart';
import 'package:mechanix_music/src/features/presentation/song_tile.dart';
import 'package:tuple/tuple.dart';
import 'package:widgets/widgets/bottom_sheet_modals/mechanix_bottom_sheet.dart';

class UpcomingTrack extends StatelessWidget {
  final SongInfo songDetails;
  final bool isFavorited;

  const UpcomingTrack({
    super.key,
    required this.songDetails,
    required this.isFavorited,
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
    return Container(
      color: Colors.black,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Now Playing Section
          Padding(
            padding: EdgeInsets.only(left: 16, right: 20, top: 24),
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                // Album Artwork
                UpcomingArtworkCircularProgress(
                  songInfo: songDetails,
                  player: context.read<SongsBloc>().player,
                ),
                SizedBox(width: 28),

                // Song Info and Controls Column
                Expanded(
                  child: Column(
                    spacing: 24,
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      // Title and Action Icons Row
                      Row(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          // Song Info
                          Expanded(
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Text(
                                  songDetails.title,
                                  overflow: TextOverflow.ellipsis,
                                  style: const TextStyle(
                                    color: MusicColors.headingTextColor,
                                    fontSize: 24,
                                    fontWeight: FontWeight.w600,
                                    height: 1.2,
                                  ),
                                  maxLines: 1,
                                ),
                                SizedBox(height: 8),
                                Text(
                                  songDetails.artist,
                                  overflow: TextOverflow.ellipsis,
                                  style: const TextStyle(
                                    color: Colors.white70,
                                    fontSize: 14,
                                  ),
                                  maxLines: 1,
                                ),
                              ],
                            ),
                          ),
                          // Action Icons
                          Row(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            mainAxisAlignment: MainAxisAlignment.start,
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
                              MusicIconButton(
                                onPressed:
                                    () => _showAddToPlaylistSheet(context),
                                backgroundColor: Colors.transparent,
                                icon: MusicIcons.addSongIcon,
                                iconSize: 24,
                              ),
                            ],
                          ),
                        ],
                      ),

                      // Progress Bar
                      const UpcomingSongProgressBar(),
                    ],
                  ),
                ),
              ],
            ),
          ),

          SizedBox(height: 24),

          // Upcoming Tracks Section
          Expanded(
            child: Container(
              width: double.infinity,
              decoration: BoxDecoration(
                image: DecorationImage(
                  image: AssetImage(MusicIcons.backgroundSliderIcon),
                  fit: BoxFit.fill,
                ),
              ),
              padding: EdgeInsets.symmetric(horizontal: 16, vertical: 16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    'Upcoming tracks',
                    style: TextStyle(
                      color: Colors.white,
                      fontSize: 20,
                      fontWeight: FontWeight.w600,
                    ),
                  ),

                  // Upcoming tracks list will go here
                  BlocSelector<SongsBloc, SongsState, MusicMode>(
                    selector: (state) => state.musicMode,
                    builder: (context, musicMode) {
                      return BlocSelector<
                        SongsBloc,
                        SongsState,
                        Tuple2<int?, List<SongInfo>>
                      >(
                        selector:
                            (state) =>
                                Tuple2(state.currentIndex, state.playbackQueue),
                        builder: (context, data) {
                          final int? currentIndex = data.item1;
                          final List<SongInfo> queue = data.item2;

                          //  Safety checks
                          final List<SongInfo> upcomingSongs =
                              (currentIndex != null &&
                                      currentIndex >= 0 &&
                                      currentIndex + 1 < queue.length)
                                  ? queue.sublist(currentIndex + 1)
                                  : [];

                          if (upcomingSongs.isEmpty) {
                            return const Text("No Upcoming Tracks");
                          }

                          return Expanded(
                            child: ListView.builder(
                              itemCount: upcomingSongs.length,
                              itemBuilder: (context, index) {
                                return SongTile(
                                  song: upcomingSongs[index],
                                  onTap:
                                      () => context.read<SongsBloc>().add(
                                        JumpToIndex(currentIndex! + 1 + index),
                                      ),
                                );
                              },
                            ),
                          );
                        },
                      );
                    },
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}
