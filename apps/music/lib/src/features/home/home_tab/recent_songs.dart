import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/presentation/song_tile.dart';

class RecentSongs extends StatefulWidget {
  const RecentSongs({super.key});

  @override
  State<RecentSongs> createState() => _RecentSongsState();
}

class _RecentSongsState extends State<RecentSongs> {
  final PageController _pageController = PageController(viewportFraction: 0.9);
  int currentPage = 0;

  List<List<SongInfo>> _buildPages(List<SongInfo> songs) {
    const pageSize = 3;
    final List<List<SongInfo>> pages = [];

    for (int i = 0; i < songs.length; i += pageSize) {
      pages.add(
        songs.sublist(
          i,
          (i + pageSize < songs.length) ? i + pageSize : songs.length,
        ),
      );
    }
    return pages;
  }

  void _scrollLeft() {
    if (currentPage > 0) {
      _pageController.previousPage(
        duration: const Duration(milliseconds: 300),
        curve: Curves.easeInOut,
      );
    }
  }

  void _scrollRight(int pageCount) {
    if (currentPage < pageCount - 1) {
      _pageController.nextPage(
        duration: const Duration(milliseconds: 300),
        curve: Curves.easeInOut,
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    return BlocSelector<SongsBloc, SongsState, List<SongInfo>>(
      selector: (state) => state.recentlyPlayedSongs,
      builder: (context, songs) {
        if (songs.isEmpty) {
          return const SizedBox.shrink();
        }

        final pages = _buildPages(songs);

        return SizedBox(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Header
              Padding(
                padding: const EdgeInsets.symmetric(
                  horizontal: 16,
                  vertical: 0,
                ),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Text(
                      'Recently Played',
                      style: TextStyle(
                        color: MusicColors.textColor,
                        fontSize: 18,
                        height: 1.35,
                        fontWeight: FontWeight.w500,
                      ),
                    ),
                    Row(
                      children: [
                        IconButton(
                          icon: Image.asset(
                            MusicIcons.previousPageIcon,
                            height: 24,
                            width: 24,
                            color:
                                currentPage > 0
                                    ? MusicColors.primaryTextColor
                                    : MusicColors.disabledColor,
                          ),
                          iconSize: 40,
                          onPressed: currentPage > 0 ? _scrollLeft : null,
                        ),
                        IconButton(
                          icon: Image.asset(
                            MusicIcons.nextPageIcon,
                            height: 24,
                            width: 24,
                            color:
                                currentPage < pages.length - 1
                                    ? MusicColors.primaryTextColor
                                    : MusicColors.disabledColor,
                          ),
                          iconSize: 40,
                          onPressed: () => _scrollRight(pages.length),
                        ),
                      ],
                    ),
                  ],
                ),
              ),

              // Pages
              SizedBox(
                height: 240,
                child: PageView.builder(
                  controller: _pageController,
                  padEnds: false,
                  itemCount: pages.length,
                  onPageChanged: (index) {
                    setState(() => currentPage = index);
                  },
                  itemBuilder: (context, pageIndex) {
                    final pageSongs = pages[pageIndex];

                    return Padding(
                      padding: const EdgeInsets.only(left: 16, right: 8),
                      child: Column(
                        children:
                            pageSongs.map((song) {
                              return BlocSelector<SongsBloc, SongsState, bool>(
                                selector:
                                    (state) => state.currentSong?.id == song.id,
                                builder: (context, isCurrentSong) {
                                  return SongTile(
                                    song: song,
                                    isCurrentSong: isCurrentSong,
                                  );
                                },
                              );
                            }).toList(),
                      ),
                    );
                  },
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}
