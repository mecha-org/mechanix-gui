import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_view/add_music_input.dart';
import 'package:mechanix_music/src/features/presentation/song_tile.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/filled_button/mechanix_filled_button_theme.dart';

class AddMusicSheet extends StatefulWidget {
  final PlaylistInfo playlistInfo;

  const AddMusicSheet({super.key, required this.playlistInfo});

  @override
  State<AddMusicSheet> createState() => _AddMusicSheetState();
}

class _AddMusicSheetState extends State<AddMusicSheet> {
  final List<String> selectedMusic = [];

  void updateSelection(String id) {
    if (selectedMusic.contains(id)) {
      setState(() {
        selectedMusic.remove(id);
      });
    } else {
      setState(() {
        selectedMusic.add(id);
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
                child: BlocSelector<SongsBloc, SongsState, List<SongInfo>>(
                  selector: (state) => state.songs,
                  builder: (context, songs) {
                    if (songs.isEmpty) {
                      return Center(
                        child: Text(
                          "No songs available",
                          style: TextStyle(
                            fontSize: 16,
                            color: MusicColors.secondaryTextColor,
                          ),
                        ),
                      );
                    }

                    return ListView.builder(
                      itemCount: songs.length,
                      itemBuilder: (context, index) {
                        final song = songs[index];
                        final isAlreadyInPlaylist = widget.playlistInfo.songIds
                            .contains(song.id);

                        return SongTile(
                          song: song,
                          isPaddingRequired: true,
                          isDisabled: isAlreadyInPlaylist,
                          isSelected: selectedMusic.contains(song.id),
                          onTap:
                              isAlreadyInPlaylist
                                  ? null
                                  : () => updateSelection(song.id),
                        );
                      },
                    );
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
                child: AddMusicInput(),
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
                                songIds: selectedMusic,
                                playlistIds: [widget.playlistInfo.id],
                                isMusicList: true,
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
