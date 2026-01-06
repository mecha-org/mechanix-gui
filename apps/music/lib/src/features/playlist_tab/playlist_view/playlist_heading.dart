import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/features/playlist_tab/add_playlist_bar.dart';
import 'package:widgets/extensions/edge_insets.dart';

class PlaylistHeading extends StatefulWidget {
  final PlaylistInfo playlist;

  const PlaylistHeading({super.key, required this.playlist});

  @override
  State<PlaylistHeading> createState() => _PlaylistHeadingState();
}

class _PlaylistHeadingState extends State<PlaylistHeading> {
  BottomBarView? previousBottomBarView;
  bool isEditing = false;
  late String playlistName;

  @override
  void initState() {
    super.initState();
    playlistName = widget.playlist.name;
  }

  @override
  void didUpdateWidget(PlaylistHeading oldWidget) {
    super.didUpdateWidget(oldWidget);
    // Update name if playlist changes and not currently editing
    if (oldWidget.playlist.id != widget.playlist.id ||
        (!isEditing && oldWidget.playlist.name != widget.playlist.name)) {
      playlistName = widget.playlist.name;
    }
  }

  void _showEditPlaylistBottomSheet(BuildContext context) {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      backgroundColor: Colors.transparent,
      builder:
          (bottomSheetContext) => Container(
            decoration: BoxDecoration(
              color: Theme.of(context).scaffoldBackgroundColor,
              borderRadius: const BorderRadius.only(
                topLeft: Radius.circular(16),
                topRight: Radius.circular(16),
              ),
            ),
            child: AddPlaylistBar(
              initialValue: playlistName,
              playlistId: widget.playlist.id,
              key: const ValueKey('edit_playlist_heading'),
              onChanged: (value) {
                // Update local state with new name
                setState(() {
                  playlistName = value;
                });
              },
            ),
          ),
    ).whenComplete(() {
      if (mounted) {
        setState(() {
          isEditing = false;
        });
        if (mounted) {
          context.read<SongsBloc>().add(BottomBarToggle(BottomBarView.normal));
        }
        // Reset to original name if cancelled
        setState(() {
          playlistName = widget.playlist.name;
        });
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return BlocSelector<SongsBloc, SongsState, BottomBarView>(
      selector: (state) => state.bottomBarView,
      builder: (context, bottomBarView) {
        // Show bottom sheet when bottomBarView changes to add and it's for this playlist
        if (bottomBarView == BottomBarView.add &&
            previousBottomBarView != BottomBarView.add) {
          WidgetsBinding.instance.addPostFrameCallback((_) {
            _showEditPlaylistBottomSheet(context);
          });
        }
        previousBottomBarView = bottomBarView;

        return GestureDetector(
          onTap: () {
            setState(() {
              isEditing = true;
              playlistName = widget.playlist.name;
            });
            // Trigger bottom sheet
            context.read<SongsBloc>().add(BottomBarToggle(BottomBarView.add));
          },
          child: AnimatedContainer(
            duration: const Duration(milliseconds: 200),

            child: Text(
              playlistName,
              style: const TextStyle(
                color: MusicColors.primaryTextColor,
                fontWeight: FontWeight.w600,
                fontSize: 24,
                letterSpacing: -1.1,
                height: 1.25,
              ),
            ),
          ).padLeft(16),
        );
      },
    );
  }
}
