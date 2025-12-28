import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/constants.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_bottom_sheet_add.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_tile.dart';
import 'package:tuple/tuple.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/filled_button/mechanix_filled_button_theme.dart';

class AddToPlaylistSheet extends StatefulWidget {
  final SongInfo song;
  const AddToPlaylistSheet({super.key, required this.song});

  @override
  State<AddToPlaylistSheet> createState() => _AddToPlaylistSheetState();
}

class _AddToPlaylistSheetState extends State<AddToPlaylistSheet> {
  final List<String> selectedPlaylists = [];

  @override
  void initState() {
    super.initState();
    context.read<SongsBloc>().add(SearchPlaylist(''));
  }

  void updateSelection(String id) {
    if (selectedPlaylists.contains(id)) {
      setState(() {
        selectedPlaylists.remove(id);
      });
    } else {
      setState(() {
        selectedPlaylists.add(id);
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        return Container(
          height: constraints.maxHeight,
          width: double.infinity,
          decoration: BoxDecoration(
            color: context.secondary,
            borderRadius: BorderRadius.only(
              topLeft: Radius.circular(16),
              topRight: Radius.circular(16),
            ),
          ),

          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Title
              Padding(
                padding: EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                child: Text(
                  "Playlists",
                  style: TextStyle(
                    fontSize: 24,
                    height: 1.25,
                    letterSpacing: -1.1,
                    fontWeight: FontWeight.w600,
                    color: MusicColors.primaryTextColor,
                  ),
                ),
              ),

              // Playlist list
              Expanded(
                child: BlocSelector<
                  SongsBloc,
                  SongsState,
                  Tuple2<List<PlaylistInfo>, List<PlaylistInfo>>
                >(
                  selector:
                      (state) =>
                          Tuple2(state.playlists, state.searchedPlaylist),
                  builder: (context, state) {
                    final playlists = state.item1;
                    final searchItem = state.item2;
                    if (playlists.isEmpty) {
                      return Center(
                        child: Text(
                          "No playlists available",
                          style: TextStyle(
                            fontSize: 16,
                            color: MusicColors.secondaryTextColor,
                          ),
                        ),
                      );
                    }
                    if (searchItem.isNotEmpty) {
                      return ListView.builder(
                        itemCount: searchItem.length,
                        itemBuilder: (context, index) {
                          final playlist = searchItem[index];
                          final isAlreadyInPlaylist =
                              playlist.songIds.length >=
                                  Constants.maxSongsPerPlaylist ||
                              widget.song.playlistIds.contains(playlist.id);

                          return PlaylistTile(
                            playlistInfo: playlist,
                            isSelected: selectedPlaylists.contains(playlist.id),
                            isDisabled: isAlreadyInPlaylist,
                            isMenuRequired: false,
                            onTap:
                                isAlreadyInPlaylist
                                    ? null
                                    : () => updateSelection(playlist.id),
                          );
                        },
                      );
                    } else {
                      return ListView.builder(
                        itemCount: playlists.length,
                        itemBuilder: (context, index) {
                          final playlist = playlists[index];
                          final isAlreadyInPlaylist =
                              playlist.songIds.length >=
                                  Constants.maxSongsPerPlaylist ||
                              widget.song.playlistIds.contains(playlist.id);

                          return PlaylistTile(
                            playlistInfo: playlist,
                            isSelected: selectedPlaylists.contains(playlist.id),
                            isDisabled: isAlreadyInPlaylist,
                            isMenuRequired: false,
                            onTap:
                                isAlreadyInPlaylist
                                    ? null
                                    : () => updateSelection(playlist.id),
                          );
                        },
                      );
                    }
                  },
                ),
              ),
              Container(
                height: 64,
                decoration: BoxDecoration(
                  border: Border(
                    top: BorderSide(
                      color: context.colorScheme.surfaceContainerLow,
                      width: 1,
                    ),
                  ),
                ),
                child: PlaylistBottomSheetAdd(),
              ),

              Container(
                color: MusicColors.backgroundColor,
                height: 60,
                padding: EdgeInsets.symmetric(horizontal: 16),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Text(
                      "Add track to playlist",
                      style: TextStyle(fontSize: 20, height: 1.3),
                    ),
                    Row(
                      spacing: 12,
                      children: [
                        MechanixFilledButton(
                          theme: MechanixFilledButtonThemeData(
                            buttonSize: Size(94, 40),
                          ),
                          label: "Cancel",
                          onPressed: () {
                            Navigator.of(context).pop();
                          },
                        ),
                        MechanixFilledButton(
                          theme: MechanixFilledButtonThemeData(
                            buttonSize: Size(72, 40),
                            decoration: BoxDecoration(
                              borderRadius: BorderRadius.circular(8),
                              color: MusicColors.titleColor,
                            ),
                          ),
                          label: "Add",
                          onPressed: () {
                            context.read<SongsBloc>().add(
                              AddToPlaylist(
                                songIds: [widget.song.id],
                                playlistIds: selectedPlaylists,
                              ),
                            );
                            Navigator.of(context).pop();
                          },
                        ),
                      ],
                    ),
                  ],
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}
