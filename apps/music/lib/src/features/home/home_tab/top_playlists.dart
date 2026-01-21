import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_card.dart';
import 'package:mechanix_music/src/features/presentation/songs_icon.dart';
import 'package:widgets/extensions/build_context.dart';
import 'package:widgets/extensions/color.dart';

class TopPlaylists extends StatefulWidget {
  const TopPlaylists({super.key});

  @override
  State<TopPlaylists> createState() => _TopPlaylistsState();
}

class _TopPlaylistsState extends State<TopPlaylists> {
  final PageController _pageController = PageController(viewportFraction: 1);
  int currentPage = 0;
  static const int _itemsPerPage = 3;
  static const double _cardHeight = 164;
  static const double _cardWidth = 164;
  static const double _horizontalSpacing = 8;

  List<List<PlaylistInfo>> _buildPages(List<PlaylistInfo> playlists) {
    final List<List<PlaylistInfo>> pages = [];

    for (int i = 0; i < playlists.length; i += _itemsPerPage) {
      pages.add(
        playlists.sublist(
          i,
          (i + _itemsPerPage < playlists.length)
              ? i + _itemsPerPage
              : playlists.length,
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
    return BlocSelector<SongsBloc, SongsState, List<PlaylistInfo>>(
      selector: (state) => state.playlists,
      builder: (context, playlists) {
        if (playlists.isEmpty) {
          return const SizedBox.shrink();
        }

        final pages = _buildPages(playlists);

        return Container(
          margin: const EdgeInsets.only(bottom: 36),

          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            spacing: 8,
            children: [
              // Header
              Padding(
                padding: const EdgeInsets.only(left: 16, right: 10),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Text(
                      'Playlists',
                      style: TextStyle(
                        color: context.onSurfaceVariant,
                        fontSize: 18,
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
                            iconSize: 24,
                            boxSize: 24,
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
                            iconSize: 24,
                            boxSize: 24,
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

              // Horizontal PageView - Single Row
              SizedBox(
                height: _cardHeight,
                child: PageView.builder(
                  controller: _pageController,
                  padEnds: false,
                  scrollDirection: Axis.horizontal,
                  itemCount: pages.length,
                  onPageChanged: (index) {
                    setState(() => currentPage = index);
                  },
                  itemBuilder: (context, pageIndex) {
                    final pagePlaylists = pages[pageIndex];

                    return Padding(
                      padding: const EdgeInsets.only(left: 16, right: 8),
                      child: Row(
                        children: [
                          for (int i = 0; i < pagePlaylists.length; i++) ...[
                            SizedBox(
                              width: _cardWidth,
                              height: _cardHeight,
                              child: BlocSelector<SongsBloc, SongsState, bool>(
                                selector:
                                    (state) =>
                                        state.currentPlaylist.playlistId ==
                                        pagePlaylists[i].id,
                                builder: (context, isCurrentPlaylist) {
                                  return PlaylistCard(
                                    isDeletePlaylist: false,
                                    isLiked: true,
                                    isRenamePlaylist: false,
                                    isActive: isCurrentPlaylist,
                                    onRenameClick: (value) {},
                                    playlistInfo: pagePlaylists[i],
                                    onPlaylistTap: () {
                                      context.read<SongsBloc>().add(
                                        SelectedPlaylist(pagePlaylists[i].id),
                                      );
                                    },
                                  );
                                },
                              ),
                            ),
                            if (i < pagePlaylists.length - 1)
                              const SizedBox(width: _horizontalSpacing),
                          ],
                        ],
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

  @override
  void dispose() {
    _pageController.dispose();
    super.dispose();
  }
}
