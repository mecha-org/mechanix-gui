import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottom_bar/mechanix_bottom_bar.dart';
import 'package:widgets/widgets/bottom_bar/mechanix_bottom_bar_theme.dart';

class AudioPlayerBottomBar extends StatelessWidget {
  final ValueChanged<bool> togglePlayer;
  final bool isUpcomingTrackWindow;
  const AudioPlayerBottomBar({
    super.key,
    required this.togglePlayer,
    required this.isUpcomingTrackWindow,
  });

  @override
  Widget build(BuildContext context) {
    return BlocSelector<SongsBloc, SongsState, bool>(
      selector: (state) => state.isPlaying,
      builder:
          (context, isPlaying) => MechanixBottomBar(
            theme: MechanixBottomBarThemeData(
              decoration: BoxDecoration(
                color: Color(0xFF2E2E2E),
                borderRadius: BorderRadius.only(
                  topLeft: Radius.circular(0),
                  topRight: Radius.circular(0),
                ),
              ),
            ),
            leadingWidget: [
              BottomBarButton(
                disabledColor: Theme.of(context).disabledColor,
                iconTheme: MechanixBottomBarIconThemeData(
                  buttonMargin: EdgeInsets.only(left: 10),
                  iconSize: Size(24, 24),
                  iconBoxSize: Size(44, 44),
                ),
                onPressed: () {
                  if (Navigator.canPop(context)) {
                    Navigator.pop(context);
                  }
                },

                iconPath: MusicIcons.backIcon,
              ),
            ],
            centerWidget: [
              BottomBarButton(
                onPressed: () {
                  // context.read<SongsBloc>().add(MusicTabSwitch(MusicTabs.home));
                },
                iconTheme: const MechanixBottomBarIconThemeData(
                  iconSize: Size(24, 24),
                  iconBoxSize: Size(44, 44),
                ),

                iconPath: MusicIcons.repeatIcon,
              ),
              BottomBarButton(
                onPressed: () {
                  context.read<SongsBloc>().add(PlayPrevious());
                },
                iconTheme: const MechanixBottomBarIconThemeData(
                  iconSize: Size(24, 24),
                  iconBoxSize: Size(44, 44),
                ),
                iconPath: MusicIcons.prevIcon,
              ),
              BottomBarButton(
                onPressed:
                    () => context.read<SongsBloc>().add(TogglePlayPause()),
                iconTheme: const MechanixBottomBarIconThemeData(
                  iconSize: Size(32, 32),
                  iconBoxSize: Size(44, 44),
                ),
                iconPath:
                    isPlaying ? MusicIcons.pauseIcon : MusicIcons.playIcon,
              ),
              BottomBarButton(
                onPressed: () => {context.read<SongsBloc>().add(PlayNext())},
                iconTheme: const MechanixBottomBarIconThemeData(
                  iconSize: Size(24, 24),
                  iconBoxSize: Size(44, 44),
                ),
                iconPath: MusicIcons.nextIcon,
              ),

              BottomBarButton(
                onPressed: () {
                  // context.read<SongsBloc>().add(
                  //   MusicTabSwitch(MusicTabs.favorites),
                  // );
                },
                iconTheme: const MechanixBottomBarIconThemeData(
                  iconSize: Size(24, 24),
                  iconBoxSize: Size(44, 44),
                ),
                iconPath: MusicIcons.shuffleIcon,
              ),
            ],
            anchorWidget: [
              BottomBarButton(
                isSelected: isUpcomingTrackWindow,
                onPressed: () => togglePlayer(!isUpcomingTrackWindow),
                iconTheme: const MechanixBottomBarIconThemeData(
                  iconSize: Size(28, 28),
                  iconBoxSize: Size(44, 44),
                  buttonMargin: EdgeInsets.only(right: 12),
                ),
                iconPath: MusicIcons.upcomingIcon,
              ),
            ],
          ),
    );
  }
}
