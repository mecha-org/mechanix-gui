import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/constants.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/home/common/music_icon_widget.dart';
import 'package:widgets/mechanix.dart';

class PlaylistBottomSheetAdd extends StatefulWidget {
  const PlaylistBottomSheetAdd({super.key});

  @override
  State<PlaylistBottomSheetAdd> createState() => _PlaylistBottomSheetAddState();
}

enum ActiveMode { add, search, none }

class _PlaylistBottomSheetAddState extends State<PlaylistBottomSheetAdd> {
  ActiveMode _activeMode = ActiveMode.none;
  String textValue = '';
  @override
  void dispose() {
    super.dispose();
  }

  void _handlePress() {
    if (_activeMode == ActiveMode.search) {
      context.read<SongsBloc>().add(SearchPlaylist(''));
    } else {
      context.read<SongsBloc>().add(CreatePlaylist(textValue));
    }
    setState(() {
      _activeMode = ActiveMode.none;
      textValue = '';
    });
  }

  @override
  Widget build(BuildContext context) {
    if (_activeMode == ActiveMode.add) {
      return Row(
        crossAxisAlignment: CrossAxisAlignment.center,
        spacing: 16,
        children: [
          Expanded(
            child: MechanixTextInput.textInput(
              autofocus: true,
              onChanged:
                  (value) => setState(() {
                    textValue = value;
                  }),
              anchorWidget: MusicIconButton(
                onPressed: () => _handlePress(),
                icon: MusicIcons.checkIcon,
                backgroundColor: Colors.transparent,
              ),
            ),
          ),
        ],
      );
    }
    if (_activeMode == ActiveMode.search) {
      return Row(
        crossAxisAlignment: CrossAxisAlignment.center,
        spacing: 16,
        children: [
          Expanded(
            child: MechanixTextInput.search(
              autofocus: true,
              onChanged:
                  (value) =>
                      context.read<SongsBloc>().add(SearchPlaylist(value)),

              // (value) => setState(() {
              //   textValue = value;
              // }),
              onClear: () => _handlePress(),
              anchorWidget: MusicIconButton(
                onPressed: () => _handlePress(),
                icon: MusicIcons.checkIcon,
                backgroundColor: Colors.transparent,
              ),
            ),
          ),
        ],
      );
    }

    // Show buttons by default
    return Row(
      crossAxisAlignment: CrossAxisAlignment.center,
      spacing: 16,
      mainAxisAlignment: MainAxisAlignment.end,
      children: [
        BlocSelector<SongsBloc, SongsState, bool>(
          selector:
              (state) => state.playlists.length >= Constants.playlistLimit,
          builder:
              (context, isLimitReached) => IconButton(
                iconSize: 44,
                onPressed:
                    isLimitReached
                        ? null
                        : () {
                          setState(() {
                            _activeMode = ActiveMode.add;
                          });
                        },
                icon: Image.asset(
                  MusicIcons.addIcon,
                  width: 28,
                  height: 28,
                  color: isLimitReached ? Colors.grey : null,
                ),
              ),
        ),
        IconButton(
          iconSize: 44,
          onPressed: () {
            setState(() {
              context.read<SongsBloc>().add(SearchPlaylist(''));
              _activeMode = ActiveMode.search;
            });
          },
          icon: Image.asset(MusicIcons.searchIcon, width: 28, height: 28),
        ),
      ],
    );
  }
}
