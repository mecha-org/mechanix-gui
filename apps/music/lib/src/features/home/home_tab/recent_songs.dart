import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/presentation/song_tile.dart';
import 'package:mechanix_music/src/features/presentation/songs_icon.dart';
import 'package:widgets/extension.dart';

class RecentSong extends StatefulWidget {
  const RecentSong({super.key});

  @override
  State<RecentSong> createState() => _RecentSongState();
}

class _RecentSongState extends State<RecentSong> {
  final PageController _pageController = PageController(viewportFraction: 0.9);
  int currentPage = 0;
  static const double _songTileHeight = 81;
  static const double _maxHeight = 243;

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

  double _calculatePageHeight(int itemCount) {
    final height = itemCount * _songTileHeight;
    return height.clamp(0, _maxHeight);
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

        return Container(
          margin: const EdgeInsets.only(bottom: 36),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Header
              Padding(
                padding: const EdgeInsets.only(left: 16, right: 10),
                child: Row(
                  spacing: 12,
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Text(
                      'Recently Played',
                      style: TextStyle(
                        color: context.onSurfaceVariant,
                        fontSize: 20,
                        height: 1.35,
                        fontWeight: FontWeight.w500,
                      ),
                    ),
                    Row(
                      spacing: 12,
                      children: [
                        IconButton(
                          icon: SongsIcon(
                            iconPath: MusicIcons.previousPageIcon,
                            iconColor:
                                currentPage > 0
                                    ? context.colorScheme.onSurface
                                    : context.colorScheme.onSurfaceVariant,
                          ),
                          iconSize: 40,
                          onPressed: currentPage > 0 ? _scrollLeft : null,
                        ),
                        IconButton(
                          icon: SongsIcon(
                            iconPath: MusicIcons.nextPageIcon,
                            iconColor:
                                currentPage < pages.length - 1
                                    ? context.colorScheme.onSurface
                                    : context.colorScheme.onSurfaceVariant,
                          ),
                          iconSize: 40,
                          onPressed:
                              currentPage < pages.length - 1
                                  ? () => _scrollRight(pages.length)
                                  : null,
                        ),
                      ],
                    ),
                  ],
                ),
              ),

              // Pages
              SizedBox(
                height: _calculatePageHeight(pages[0].length),
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
