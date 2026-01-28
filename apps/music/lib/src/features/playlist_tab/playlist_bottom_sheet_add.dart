import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/constants.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/home/common/music_icon_widget.dart';
import 'package:mechanix_music/src/features/presentation/songs_icon.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/text_input/mechanix_text_input_theme.dart';

class PlaylistBottomSheetAdd extends StatefulWidget {
  final ValueChanged<String> onDraftChanged;
  final VoidCallback onDraftCleared;
  final FocusNode focusNode;
  final bool isFocused;

  const PlaylistBottomSheetAdd({
    super.key,
    required this.onDraftChanged,
    required this.onDraftCleared,
    required this.focusNode,
    required this.isFocused,
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
              theme: MechanixTextInputThemeData(
                widgetHeight: 66,
                widgetDecoration: BoxDecoration(
                  color: context.surfaceContainerHigh,
                ),
              ),
              focusNode: widget.focusNode,
              anchorWidget: Container(
                margin: EdgeInsets.only(left: 5),
                child: MusicIconButton(
                  onPressed: () {
                    if (textValue.trim().isNotEmpty) {
                      context.read<SongsBloc>().add(
                        CreateUpdatePlaylist(playlistName: textValue),
                      );
                    }

                    _reset();
                  },
                  iconColor: context.onSurface,
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
              theme: MechanixTextInputThemeData(
                widgetDecoration: BoxDecoration(
                  color: context.surfaceContainerHigh,
                ),
              ),
              focusNode: widget.focusNode,
              isClearButtonRequired: false,
              anchorWidget: MusicIconButton(
                onPressed: _reset,
                icon: MusicIcons.closeIcon,
                iconColor: context.onSurface,
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
                style: ButtonStyle(
                  overlayColor: WidgetStateProperty.all(Colors.transparent),
                  backgroundColor: WidgetStateProperty.resolveWith((states) {
                    if (states.contains(WidgetState.pressed)) {
                      return context.secondaryContainer;
                    }
                    if (states.contains(WidgetState.hovered)) {
                      return Colors.transparent;
                    }
                    return Colors.transparent;
                  }),
                  fixedSize: WidgetStateProperty.all(Size(40, 40)),
                  animationDuration: const Duration(milliseconds: 300),
                  tapTargetSize: MaterialTapTargetSize.padded,
                  shape: WidgetStateProperty.all(
                    RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(8),
                    ),
                  ),
                  iconColor: WidgetStateProperty.resolveWith((states) {
                    if (states.contains(WidgetState.pressed)) {
                      return context.primary;
                    }

                    return context.onSurface;
                  }),
                ),
                icon: SongsIcon(
                  iconPath: MusicIcons.addIcon,
                  boxSize: 28,
                  iconSize: 28,
                  iconColor:
                      isLimitReached
                          ? Theme.of(context).disabledColor
                          : context.onSurface,
                ),
              ),
        ),
        Container(
          margin: EdgeInsets.only(right: 5),
          child: IconButton(
            iconSize: 44,
            onPressed: () => setState(() => _activeMode = ActiveMode.search),
            icon: SongsIcon(
              iconPath: MusicIcons.searchIcon,
              boxSize: 28,
              iconSize: 28,
              iconColor: context.onSurface,
            ),
          ),
        ),
      ],
    );
  }
}
