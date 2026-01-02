import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/playlist_tab/add_to_playlist_sheet.dart';
import 'package:widgets/widgets/bottom_sheet_modals/mechanix_bottom_sheet.dart';

class SongMenu extends StatefulWidget {
  final SongInfo song;
  final VoidCallback? onToggleFavourite;
  final bool isDeleteMenu;

  const SongMenu({
    super.key,
    required this.song,
    this.onToggleFavourite,
    required this.isDeleteMenu,
  });

  @override
  State<SongMenu> createState() => _SongMenuState();
}

class _SongMenuState extends State<SongMenu> {
  bool isMenuOpen = false;

  void _showAddToPlaylistSheet(BuildContext context) {
    context.read<SongsBloc>().add(LoadPlaylist());

    MechanixBottomSheet.show(
      topTabWidth: 370,
      topTabRightSideShiftLength: 40,
      context,
      child: AddToPlaylistSheet(song: widget.song),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Theme(
      data: Theme.of(context).copyWith(
        splashColor: Colors.transparent,
        highlightColor: Colors.transparent,
        hoverColor: Colors.transparent,
      ),
      child: PopupMenuButton<_SongMenuAction>(
        tooltip: '',
        padding: EdgeInsets.zero,
        // offset: const Offset(0, 36),
        offset: const Offset(-45, 10),

        color: MusicColors.tapColor,
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
        style: ButtonStyle(
          splashFactory: NoSplash.splashFactory,
          overlayColor: WidgetStatePropertyAll(Colors.transparent),
        ),
        // ✅ track open / close
        onOpened: () => setState(() => isMenuOpen = true),
        onCanceled: () => setState(() => isMenuOpen = false),
        onSelected: (action) {
          setState(() => isMenuOpen = false);

          switch (action) {
            case _SongMenuAction.playNext:
              context.read<SongsBloc>().add(
                AddToQueue(widget.song, playNext: true),
              );
              break;

            case _SongMenuAction.addToQueue:
              context.read<SongsBloc>().add(AddToQueue(widget.song));
              break;

            case _SongMenuAction.addToPlaylist:
              _showAddToPlaylistSheet(context);
              break;

            case _SongMenuAction.toggleFavourite:
              widget.onToggleFavourite?.call();
              break;

            case _SongMenuAction.delete:
              context.read<SongsBloc>().add(DeleteSong(widget.song));
              break;
          }
        },

        // ✅ SAME pressed UI as PlaylistMenu
        icon: Container(
          height: 40,
          width: 40,
          decoration: BoxDecoration(
            color:
                isMenuOpen ? MusicColors.backgroundColor : Colors.transparent,
            borderRadius: BorderRadius.circular(8),
          ),
          child: Center(
            child: Image.asset(
              MusicIcons.threeDotIcon,
              width: 28,
              height: 28,
              color: isMenuOpen ? MusicColors.borderColor : Colors.white,
            ),
          ),
        ),

        itemBuilder:
            (context) => [
              _menuItem(
                value: _SongMenuAction.playNext,
                title: 'Play Next',
                icon: MusicIcons.playNextIcon,
              ),
              _menuItem(
                value: _SongMenuAction.addToQueue,
                title: 'Add to queue',
                icon: MusicIcons.queueIcon,
              ),
              _menuItem(
                value: _SongMenuAction.addToPlaylist,
                title: 'Add to playlist',
                icon: MusicIcons.addSongIcon,
              ),
              _menuItem(
                value: _SongMenuAction.toggleFavourite,
                title:
                    widget.song.isFavourite ? 'Unlike Track' : 'Add to liked',
                icon:
                    widget.song.isFavourite
                        ? MusicIcons.filledFavouriteIcon
                        : MusicIcons.favouritesIcon,
                color:
                    widget.song.isFavourite
                        ? MusicColors.borderColor
                        : MusicColors.primaryTextColor,
              ),
              if (widget.isDeleteMenu)
                _menuItem(
                  value: _SongMenuAction.delete,
                  title: 'Delete',
                  icon: MusicIcons.deleteIcon,
                  color: MusicColors.deleteColor,
                ),
            ],
      ),
    );
  }

  PopupMenuItem<_SongMenuAction> _menuItem({
    required _SongMenuAction value,
    required String title,
    required String icon,
    Color color = MusicColors.primaryTextColor,
  }) {
    return PopupMenuItem<_SongMenuAction>(
      value: value,
      height: 42,
      child: Row(
        children: [
          Image.asset(icon, width: 20, height: 20, color: color),
          const SizedBox(width: 12),
          Text(
            title,
            style: TextStyle(
              color: color,
              fontFamily: 'Overused Grotesk',
              fontSize: 18,
            ),
          ),
        ],
      ),
    );
  }
}

enum _SongMenuAction {
  playNext,
  addToQueue,
  addToPlaylist,
  toggleFavourite,
  delete,
}
