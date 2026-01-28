import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/models.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:widgets/mechanix.dart';

class EmptyPlaylistView extends StatelessWidget {
  final PlaylistViewEnum playlistView;
  const EmptyPlaylistView({super.key, required this.playlistView});

  @override
  Widget build(BuildContext context) {
    if (playlistView == PlaylistViewEnum.list) {
      return SliverPadding(
        padding: const EdgeInsets.all(16),
        sliver: SliverToBoxAdapter(
          child: MouseRegion(
            cursor: SystemMouseCursors.click,
            child: GestureDetector(
              behavior: HitTestBehavior.translucent,
              onTap: () {
                context.read<SongsBloc>().add(
                  BottomBarToggle(BottomBarView.add),
                );
              },
              child: Row(
                children: [
                  IconButton(
                    iconSize: 44,
                    onPressed: () {
                      context.read<SongsBloc>().add(
                        BottomBarToggle(BottomBarView.add),
                      );
                    },
                    style: ButtonStyle(
                      shape: WidgetStatePropertyAll(
                        RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(8),
                        ),
                      ),
                      backgroundColor: WidgetStatePropertyAll(
                        context.secondary,
                      ),
                    ),
                    icon: IconWidget(
                      iconColor: context.primary,
                      iconPath: MusicIcons.plusIcon,
                      iconHeight: 24,
                      boxHeight: 24,
                      boxWidth: 24,
                      iconWidth: 24,
                    ),
                  ),
                  const SizedBox(width: 20),
                   Text(
                    "Add a new playlist",
                    style: TextStyle(
                      fontSize: 20,
                      color: context.colorScheme.onSurfaceVariant,
                      fontWeight: FontWeight.w500,
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      );
    }

    ///  GRID VIEW
    return SliverPadding(
      padding: const EdgeInsets.all(16),
      sliver: SliverGrid(
        gridDelegate: const SliverGridDelegateWithMaxCrossAxisExtent(
          maxCrossAxisExtent: 164,
          mainAxisSpacing: 16,
          crossAxisSpacing: 16,
          childAspectRatio: 1, // 164 × 164
        ),
        delegate: SliverChildListDelegate([
          _EmptyGridCard(
            onTap: () {
              context.read<SongsBloc>().add(BottomBarToggle(BottomBarView.add));
            },
          ),
        ]),
      ),
    );
  }
}

class _EmptyGridCard extends StatelessWidget {
  final VoidCallback onTap;

  const _EmptyGridCard({required this.onTap});

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
      cursor: SystemMouseCursors.click,
      child: GestureDetector(
        onTap: onTap,
        child: Container(
          height: 164,
          decoration: BoxDecoration(
            color: context.secondary,
            borderRadius: BorderRadius.circular(12),
          ),
          child: Center(
            child: IconWidget(
              iconColor: context.primary,
              iconPath: MusicIcons.plusIcon,
              iconHeight: 24,
              boxHeight: 24,
              boxWidth: 24,
              iconWidth: 24,
            ),
          ),
        ),
      ),
    );
  }
}
