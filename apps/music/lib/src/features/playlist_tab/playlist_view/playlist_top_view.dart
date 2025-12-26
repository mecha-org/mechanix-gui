import 'package:flutter/material.dart';
import 'package:mechanix_music/models/playlist_info.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';

class PlaylistTopView extends StatelessWidget {
  final PlaylistInfo playlistInfo;
  const PlaylistTopView({
    super.key,
    required this.playlistInfo,
  });

  @override
  Widget build(BuildContext context) {
    final hasCoverImage =
        playlistInfo.coverImagePath != null &&
        playlistInfo.coverImagePath!.isNotEmpty;
    return SliverPadding(
      padding: EdgeInsets.symmetric(horizontal: 12),
      sliver: SliverToBoxAdapter(
        child: Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            Expanded(
              child: Stack(
                clipBehavior: Clip.none,
                children: [
                  Image.asset(MusicIcons.audioImage, width: 152, height: 160),
                  Positioned(
                    left: 72,
                    child: Container(
                      width: 160,
                      height: 160,
                      decoration: BoxDecoration(
                        borderRadius: BorderRadius.circular(8),
                        color: hasCoverImage ? null : MusicColors.albumColor,
                        image:
                            hasCoverImage
                                ? DecorationImage(
                                  image: AssetImage(
                                    playlistInfo.coverImagePath!,
                                  ),
                                  fit: BoxFit.cover,
                                )
                                : null,
                      ),
                      child: Center(
                        child: Image.asset(
                          MusicIcons.plusIcon,
                          width: 24,
                          height: 24,
                          color: Colors.white,
                        ),
                      ),
                    ),
                  ),
                ],
              ),
            ),
            Expanded(
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [Text("0 tracks, 0 min"), Text("0 tracks, 0 min")],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
