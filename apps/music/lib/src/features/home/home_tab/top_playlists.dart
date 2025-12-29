import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_card.dart';

class TopPlaylists extends StatefulWidget {
  const TopPlaylists({super.key});

  @override
  State<TopPlaylists> createState() => _TopPlaylistsState();
}

class _TopPlaylistsState extends State<TopPlaylists> {
  final ScrollController _scrollController = ScrollController();
  bool canScrollLeft = false;
  bool canScrollRight = true;

  @override
  void initState() {
    super.initState();
    _scrollController.addListener(_updateScrollButtons);
  }

  @override
  void dispose() {
    _scrollController.removeListener(_updateScrollButtons);
    _scrollController.dispose();
    super.dispose();
  }

  void _updateScrollButtons() {
    if (!_scrollController.hasClients) return;

    setState(() {
      canScrollLeft = _scrollController.position.pixels > 0;
      canScrollRight =
          _scrollController.position.pixels <
          _scrollController.position.maxScrollExtent;
    });
  }

  void _scrollLeft() {
    if (canScrollLeft && _scrollController.hasClients) {
      final double targetPosition = (_scrollController.offset - 300).clamp(
        0.0,
        _scrollController.position.maxScrollExtent,
      );

      _scrollController.animateTo(
        targetPosition,
        duration: const Duration(milliseconds: 300),
        curve: Curves.easeInOut,
      );
    }
  }

  void _scrollRight() {
    if (canScrollRight && _scrollController.hasClients) {
      final double targetPosition = (_scrollController.offset + 300).clamp(
        0.0,
        _scrollController.position.maxScrollExtent,
      );

      _scrollController.animateTo(
        targetPosition,
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

        // Update scroll button states after build
        WidgetsBinding.instance.addPostFrameCallback((_) {
          if (_scrollController.hasClients) {
            _updateScrollButtons();
          }
        });

        return Container(
          margin: const EdgeInsets.only(bottom: 16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Header
              Padding(
                padding: const EdgeInsets.symmetric(
                  horizontal: 16,
                  vertical: 10,
                ),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Text(
                      'Playlists',
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
                                canScrollLeft
                                    ? MusicColors.primaryTextColor
                                    : MusicColors.disabledColor,
                          ),
                          iconSize: 40,
                          onPressed: canScrollLeft ? _scrollLeft : null,
                        ),
                        IconButton(
                          icon: Image.asset(
                            MusicIcons.nextPageIcon,
                            height: 24,
                            width: 24,
                            color:
                                canScrollRight
                                    ? MusicColors.primaryTextColor
                                    : MusicColors.disabledColor,
                          ),
                          iconSize: 40,
                          onPressed: canScrollRight ? _scrollRight : null,
                        ),
                      ],
                    ),
                  ],
                ),
              ),

              // Horizontal Scrollable List - Single Row
              SizedBox(
                height: 164, // Height for single row of cards
                child: SingleChildScrollView(
                  controller: _scrollController,
                  scrollDirection: Axis.horizontal,
                  physics:
                      const NeverScrollableScrollPhysics(), // Disable manual scroll
                  child: Padding(
                    padding: const EdgeInsets.symmetric(horizontal: 16),
                    child: Row(
                      children: [
                        for (int i = 0; i < playlists.length; i++) ...[
                          SizedBox(
                            width: 164, // Fixed width for each card
                            child: PlaylistCard(
                              onRenameClick: (value) {},
                              playlistInfo: playlists[i],
                              onPlaylistTap: () {
                                context.read<SongsBloc>().add(
                                  SelectedPlaylist(playlists[i].id),
                                );
                              },
                            ),
                          ),
                          if (i < playlists.length - 1)
                            const SizedBox(width: 8),
                        ],
                      ],
                    ),
                  ),
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}
