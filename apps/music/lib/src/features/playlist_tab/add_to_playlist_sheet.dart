import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
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
  String draftPlaylistName = '';

  @override
  void initState() {
    super.initState();
    context.read<SongsBloc>().add(SearchPlaylist(''));
  }

  void updateSelection(String id) {
    setState(() {
      selectedPlaylists.contains(id)
          ? selectedPlaylists.remove(id)
          : selectedPlaylists.add(id);
    });
  }

  void updateDraftName(String value) {
    setState(() {
      draftPlaylistName = value;
    });
  }

  void clearDraft() {
    setState(() {
      draftPlaylistName = '';
    });
  }

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        return Container(
          height: constraints.maxHeight * 0.98,
          width: double.infinity,
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Title
              Padding(
                padding: const EdgeInsets.symmetric(
                  horizontal: 16,
                  vertical: 16,
                ),
                child: Text(
                  "Playlists",
                  style: TextStyle(
                    fontSize: 24,
                    height: 1.25,
                    letterSpacing: -1.1,
                    fontWeight: FontWeight.w600,
                    color: context.colorScheme.onSurface,
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

                    final dataSource =
                        searchItem.isNotEmpty ? searchItem : playlists;

                    if (dataSource.isEmpty && draftPlaylistName.isEmpty) {
                      return Center(
                        child: Text(
                          "No playlists available",
                          style: TextStyle(
                            fontSize: 16,
                            color: context.onSecondaryFixedVariant,
                          ),
                        ),
                      );
                    }

                    return Column(
                      children: [
                        // Draft Live preview tile
                        if (draftPlaylistName.isNotEmpty)
                          PlaylistTile(
                            onRenameClick: (value) {},
                            playlistInfo: PlaylistInfo(
                              createdAt: DateTime.now(),
                              isShuffle: false,
                              updatedAt: DateTime.now(),
                              id: 'draft',
                              name: draftPlaylistName,
                              songIds: const [],
                              coverImagePath: null,
                            ),
                            isSelected: true,
                            isDisabled: false,
                            isMenuRequired: false,
                            onTap: null,
                          ),

                        Expanded(
                          child: ListView.builder(
                            itemCount: dataSource.length,
                            itemBuilder: (context, index) {
                              final playlist = dataSource[index];
                              final isAlreadyInPlaylist =
                                  playlist.songIds.length >=
                                      Constants.maxSongsPerPlaylist ||
                                  widget.song.playlistIds.contains(playlist.id);

                              return PlaylistTile(
                                onRenameClick: (value) {},
                                playlistInfo: playlist,
                                isSelected: selectedPlaylists.contains(
                                  playlist.id,
                                ),
                                isDisabled: isAlreadyInPlaylist,
                                isMenuRequired: false,
                                onTap:
                                    isAlreadyInPlaylist
                                        ? null
                                        : () => updateSelection(playlist.id),
                              );
                            },
                          ),
                        ),
                      ],
                    );
                  },
                ),
              ),

              // Bottom add/search row
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
                child: PlaylistBottomSheetAdd(
                  onDraftChanged: updateDraftName,
                  onDraftCleared: clearDraft,
                ),
              ),

              // Footer actions
              Container(
                color: context.secondaryContainer,
                height: 60,
                padding: const EdgeInsets.symmetric(horizontal: 16),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    if (selectedPlaylists.isNotEmpty)
                      RichText(
                        text: TextSpan(
                          style: DefaultTextStyle.of(
                            context,
                          ).style.copyWith(fontSize: 20, height: 1.3),
                          children: [
                            const TextSpan(text: 'Add track to '),
                            TextSpan(
                              text:
                                  '${selectedPlaylists.length} playlist${selectedPlaylists.length == 1 ? '' : 's'}',
                              style: const TextStyle(
                                fontSize: 20,
                                fontWeight: FontWeight.bold,
                              ),
                            ),
                          ],
                        ),
                      )
                    else
                      const Text(
                        'Add track to playlist',
                        style: TextStyle(fontSize: 20, height: 1.3),
                      ),

                    Row(
                      spacing: 12,
                      children: [
                        MechanixFilledButton(
                          theme: MechanixFilledButtonThemeData(
                            buttonSize: Size(100, 40),
                            buttonColor: context.surfaceContainerHighest,
                          ),
                          label: "Cancel",
                          onPressed: () {
                            Navigator.of(context).pop();
                          },
                        ),
                        MechanixFilledButton(
                          theme: MechanixFilledButtonThemeData(
                            buttonSize: const Size(75, 40),
                            buttonColor: context.primary,
                            pressedButtonColor: context.primaryContainer,
                          ),
                          label: "Add",
                          onPressed:
                              selectedPlaylists.isEmpty
                                  ? null
                                  : () {
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
