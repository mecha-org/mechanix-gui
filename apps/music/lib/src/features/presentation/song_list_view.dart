import 'package:flutter/material.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/features/presentation/swipeable_list.dart';

class SongsListView extends StatelessWidget {
  final List<SongInfo> songs;
  final Function(SongInfo song)? onSongTap;

  const SongsListView({super.key, required this.songs, this.onSongTap});

  @override
  Widget build(BuildContext context) {
    return SliverPrototypeExtentList(
      prototypeItem: const SizedBox(height: 79),
      delegate: SliverChildBuilderDelegate(childCount: songs.length, (
        context,
        index,
      ) {
        final song = songs[index];

        return SwipeableList(song: song);
      }),
    );
  }
}
