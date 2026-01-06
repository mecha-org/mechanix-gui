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
  final ValueChanged<String> onDraftChanged;
  final VoidCallback onDraftCleared;

  const PlaylistBottomSheetAdd({
    super.key,
    required this.onDraftChanged,
    required this.onDraftCleared,
  });

  @override
  State<PlaylistBottomSheetAdd> createState() => _PlaylistBottomSheetAddState();
}

enum ActiveMode { add, search, none }

class _PlaylistBottomSheetAddState extends State<PlaylistBottomSheetAdd> {
  ActiveMode _activeMode = ActiveMode.none;
  String textValue = '';

  void _reset() {
    widget.onDraftCleared();
    context.read<SongsBloc>().add(SearchPlaylist(''));
    setState(() {
      _activeMode = ActiveMode.none;
      textValue = '';
    });
  }

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Expanded(
          child: AnimatedSwitcher(
            duration: const Duration(milliseconds: 300),
            switchInCurve: Curves.easeOut,
            switchOutCurve: Curves.easeIn,
            transitionBuilder: (child, animation) {
              final isInput = child.key != const ValueKey('default');

              final slideAnimation = Tween<Offset>(
                begin:
                    isInput
                        ? const Offset(1.0, 0.0) // from right
                        : const Offset(-1.0, 0.0), // to right
                end: Offset.zero,
              ).animate(animation);

              return SlideTransition(
                position: slideAnimation,
                child: FadeTransition(opacity: animation, child: child),
              );
            },
            child: _buildContent(context),
          ),
        ),
      ],
    );
  }

  Widget _buildContent(BuildContext context) {
    // ADD MODE
    if (_activeMode == ActiveMode.add) {
      return Row(
        key: const ValueKey('add'),
        spacing: 16,
        children: [
          Expanded(
            child: MechanixTextInput.textInput(
              autofocus: true,
              onChanged: (value) {
                textValue = value;
                widget.onDraftChanged(value);
              },
              anchorWidget: Container(
                margin: EdgeInsets.only(left: 5),
                child: MusicIconButton(
                  onPressed:
                      textValue.trim().isEmpty
                          ? null
                          : () {
                            context.read<SongsBloc>().add(
                              CreateUpdatePlaylist(playlistName: textValue),
                            );
                            _reset();
                          },
                  icon: MusicIcons.checkIcon,
                  backgroundColor: Colors.transparent,
                ),
              ),
            ),
          ),
        ],
      );
    }

    // SEARCH MODE
    if (_activeMode == ActiveMode.search) {
      return Row(
        key: const ValueKey('search'),
        spacing: 16,
        children: [
          Expanded(
            child: MechanixTextInput.search(
              autofocus: true,
              onChanged:
                  (value) =>
                      context.read<SongsBloc>().add(SearchPlaylist(value)),
              onClear: _reset,
              anchorWidget: MusicIconButton(
                onPressed: _reset,
                icon: MusicIcons.checkIcon,
                backgroundColor: Colors.transparent,
              ),
            ),
          ),
        ],
      );
    }

    // DEFAULT MODE
    return Row(
      key: const ValueKey('default'),
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
                        : () => setState(() {
                          _activeMode = ActiveMode.add;
                        }),
                icon: Image.asset(
                  MusicIcons.addIcon,
                  width: 28,
                  height: 28,
                  color: isLimitReached ? Colors.grey : null,
                ),
              ),
        ),
        Container(
          margin: EdgeInsets.only(right: 5),
          child: IconButton(
            iconSize: 44,
            onPressed: () => setState(() => _activeMode = ActiveMode.search),
            icon: Image.asset(MusicIcons.searchIcon, width: 28, height: 28),
          ),
        ),
      ],
    );
  }
}
