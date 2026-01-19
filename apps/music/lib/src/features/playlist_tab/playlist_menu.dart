import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/presentation/songs_icon.dart';
import 'package:widgets/mechanix.dart';

class PlaylistMenu extends StatefulWidget {
  final PlaylistInfo playlistInfo;
  final ValueChanged<String> onRenameClick;
  final bool isDeletePlaylist;
  final bool isRenamePlaylist;
  final bool isLiked;
  final bool isBackgroundRequired;
  final Size iconSize;
  final bool enabled;
  final Color? backgroundColor;
  const PlaylistMenu({
    super.key,
    required this.playlistInfo,
    required this.onRenameClick,
    required this.isDeletePlaylist,
    required this.isRenamePlaylist,
    required this.isLiked,
    this.isBackgroundRequired = false,
    this.iconSize = const Size(40, 40),
    this.enabled = true,
    this.backgroundColor,
  });

  @override
  State<PlaylistMenu> createState() => _PlaylistMenuState();
}

class _PlaylistMenuState extends State<PlaylistMenu> {
  bool isMenuOpen = false;

  @override
  Widget build(BuildContext context) {
    return Theme(
      data: Theme.of(context).copyWith(
        splashColor: Colors.transparent,
        highlightColor: Colors.transparent,
        hoverColor: Colors.transparent,
      ),
      child: PopupMenuButton<_PlaylistMenuAction>(
        tooltip: '',
        padding: EdgeInsets.zero,
        enabled: widget.enabled,
        style: ButtonStyle(
          splashFactory: NoSplash.splashFactory,
          overlayColor: WidgetStatePropertyAll(Colors.transparent),
        ),
        offset: const Offset(-45, 10),
        color: context.surfaceContainerHighest,
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),

        // ✅ MENU OPEN / CLOSE TRACKING
        onOpened: () {
          setState(() => isMenuOpen = true);
        },
        onCanceled: () {
          setState(() => isMenuOpen = false);
        },
        onSelected: (action) {
          setState(() => isMenuOpen = false);

          switch (action) {
            case _PlaylistMenuAction.rename:
              widget.onRenameClick(widget.playlistInfo.id);
              break;
            case _PlaylistMenuAction.delete:
              context.read<SongsBloc>().add(
                DeletePlaylist(widget.playlistInfo.id),
              );
              break;
            // case _PlaylistMenuAction.liked:
            //   context.read<SongsBloc>().add(
            //     FavouriteToggle(
            //       isFavourite: isFavourite,
            //       songIds: playlistSongs.map((e) => e.id).toList(),
            //     ),
            //   );
            //   break;
            case _PlaylistMenuAction.queue:
              context.read<SongsBloc>().add(
                AddPlaylistToQueue(
                  playlistId: widget.playlistInfo.id,
                  playNext: false,
                ),
              );
              break;
            case _PlaylistMenuAction.playNext:
              context.read<SongsBloc>().add(
                AddPlaylistToQueue(
                  playlistId: widget.playlistInfo.id,
                  playNext: true,
                ),
              );
              break;
            case _PlaylistMenuAction.none:
              break;
          }
        },

        // ✅ CUSTOM BUTTON WITH SELECTED STATE
        icon: Container(
          height: widget.iconSize.height,
          width: widget.iconSize.width,
          decoration: BoxDecoration(
            color:
                widget.isBackgroundRequired
                    ? widget.backgroundColor ?? context.secondaryContainer
                    : isMenuOpen
                    ? widget.backgroundColor ?? context.secondaryContainer
                    : Colors.transparent,
            borderRadius: BorderRadius.circular(8),
          ),
          child: Center(
            child: SongsIcon(
              iconPath: MusicIcons.threeDotIcon,
              iconSize: 28,
              boxSize: 28,
              iconColor:
                  widget.enabled
                      ? isMenuOpen
                          ? context.primary
                          : null
                      : Theme.of(context).disabledColor,
            ),
          ),
        ),

        itemBuilder:
            (context) => [
              _menuItem(
                value: _PlaylistMenuAction.playNext,
                title: 'Play Next',
                icon: MusicIcons.playNextIcon,
              ),

              _menuItem(
                value: _PlaylistMenuAction.queue,
                title: 'Add to queue',
                icon: MusicIcons.queueIcon,
              ),
              _menuItem(
                value: _PlaylistMenuAction.none,
                title: 'Add to playlist',
                icon: MusicIcons.addSongIcon,
              ),
              if (widget.isLiked)
                _menuItem(
                  value: _PlaylistMenuAction.none,
                  title:
                      widget.playlistInfo.isLiked
                          ? 'Remove from Like'
                          : 'Add to liked',
                  color: widget.playlistInfo.isLiked ? context.primary : null,

                  icon:
                      widget.playlistInfo.isLiked
                          ? MusicIcons.filledFavouriteIcon
                          : MusicIcons.favouritesIcon,
                ),
              if (widget.isRenamePlaylist)
                _menuItem(
                  value: _PlaylistMenuAction.rename,
                  title: 'Rename',
                  icon: MusicIcons.renameIcon,
                ),
              if (widget.isDeletePlaylist)
                _menuItem(
                  value: _PlaylistMenuAction.delete,
                  title: 'Delete',
                  icon: MusicIcons.deleteIcon,
                  color: MusicColors.deleteColor,
                ),
            ],
      ),
    );
  }

  PopupMenuItem<_PlaylistMenuAction> _menuItem({
    required _PlaylistMenuAction value,
    required String title,
    required String icon,
    Color? color,
  }) {
    return PopupMenuItem<_PlaylistMenuAction>(
      value: value,
      height: 42,
      child: Row(
        children: [
          SongsIcon(
            iconPath: icon,
            boxSize: 20,
            iconSize: 20,
            iconColor: color,
          ),
          const SizedBox(width: 12),
          Text(
            title,
            style: TextStyle(
              color: color ?? context.colorScheme.onSurface,
              fontFamily: 'Overused Grotesk',
              fontSize: 18,
            ),
          ),
        ],
      ),
    );
  }
}

enum _PlaylistMenuAction {
  playNext,
  queue,
  // playlist,
  // liked,
  rename,
  delete,
  none,
}
