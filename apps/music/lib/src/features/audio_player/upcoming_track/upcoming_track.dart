import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/home/common/music_icon_widget.dart';
import 'package:mechanix_music/src/features/playlist_tab/add_to_playlist_sheet.dart';
import 'package:mechanix_music/src/features/presentation/artwork_icon.dart';
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
      padding: EdgeInsets.only(left: 16, right: 20, top: 24),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Now Playing Section
          Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Album Artwork
              ArtworkIcon(artworkPath: songDetails.artworkPath, size: 108),
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
                              SizedBox(height: 14),
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
                        const SizedBox(height: 12),
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
                              onPressed: () => _showAddToPlaylistSheet(context),
                              backgroundColor: Colors.transparent,
                              icon: MusicIcons.addSongIcon,
                              iconSize: 24,
                            ),
                          ],
                        ),
                      ],
                    ),

                    // Progress Bar
                    Container(
                      alignment: Alignment.center,
                      child: Row(
                        mainAxisAlignment: MainAxisAlignment.start,
                        crossAxisAlignment: CrossAxisAlignment.center,
                        children: [
                          Expanded(
                            child: SliderTheme(
                              data: SliderThemeData(
                                padding: EdgeInsets.zero,
                                trackHeight: 2,
                                thumbShape: RoundSliderThumbShape(
                                  enabledThumbRadius: 6,
                                ),
                                overlayShape: RoundSliderOverlayShape(
                                  overlayRadius: 12,
                                ),
                                activeTrackColor: Colors.white,
                                inactiveTrackColor: Colors.white30,
                                thumbColor: Colors.white,
                              ),
                              child: Slider(
                                value: 0.3, // Replace with actual progress
                                onChanged: (value) {},
                                min: 0,
                                max: 1,
                              ),
                            ),
                          ),

                          SizedBox(width: 12),
                          Text(
                            '00:25',
                            style: TextStyle(
                              color: Colors.white,
                              fontSize: 16,
                              fontWeight: FontWeight.w500,
                            ),
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ),

          SizedBox(height: 24),

          // Upcoming Tracks Section Header
          Expanded(
            child: Container(
              color: MusicColors.tapColor,
              child: Column(
                children: [
                  Text(
                    'Upcoming tracks',
                    style: TextStyle(
                      color: Colors.white,
                      fontSize: 20,
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                ],
              ),
            ),
          ),
          // Upcoming tracks list would go here
        ],
      ),
    );
  }
}
