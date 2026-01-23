import 'package:flutter/material.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/src/commons/constants.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/playlist_tab/playlist_view/add_music_sheet.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_sheet_modals/mechanix_bottom_sheet_theme.dart';

class PlaylistAddSong extends StatelessWidget {
  final PlaylistInfo playlistInfo;
  final bool isEditMode;
  const PlaylistAddSong({
    super.key,
    required this.playlistInfo,
    required this.isEditMode,
  });
  void _showAddMusicSheet(BuildContext context) {
    MechanixBottomSheet.show(
      topTabWidth: 370,
      topTabRightSideShiftLength: 40,
      context,
      theme: MechanixBottomSheetThemeData(
        decoration: BoxDecoration(color: context.surfaceContainerHigh),
        padding: const EdgeInsets.only(bottom: 0),
      ),
      child: AddMusicSheet(playlistInfo: playlistInfo),
    );
  }

  @override
  Widget build(BuildContext context) {
    return SliverToBoxAdapter(
      child: MouseRegion(
        cursor:
            isEditMode ? SystemMouseCursors.basic : SystemMouseCursors.click,
        child: GestureDetector(
          behavior: HitTestBehavior.translucent,
          onTap:
              isEditMode
                  ? null
                  : playlistInfo.songIds.length >= Constants.maxSongsPerPlaylist
                  ? null
                  : () {
                    _showAddMusicSheet(context);
                  },
          child: Row(
            spacing: 20,
            children: [
              IconButton(
                iconSize: 44,
                onPressed:
                    isEditMode
                        ? null
                        : playlistInfo.songIds.length >=
                            Constants.maxSongsPerPlaylist
                        ? null
                        : () {
                          _showAddMusicSheet(context);
                        },

                style: ButtonStyle(
                  shape: WidgetStatePropertyAll(
                    RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(8),
                    ),
                  ),
                  backgroundColor: WidgetStatePropertyAll(context.secondary),
                ),
                icon: IconWidget(
                  iconColor:
                      isEditMode
                          ? Theme.of(context).disabledColor
                          : context.primary,
                  iconPath: MusicIcons.plusIcon,
                  iconHeight: 24,
                  boxHeight: 24,
                  boxWidth: 24,
                  iconWidth: 24,
                ),
              ),
              Text(
                "Add a track",
                style: TextStyle(
                  fontSize: 18,
                  color: context.colorScheme.onSurfaceVariant,
                  height: 1.3,
                  // letterSpacing: -1.1,
                  fontWeight: FontWeight.w500,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
