import 'package:flutter/material.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/features/presentation/song_tile.dart';

class SongsListView extends StatelessWidget {
  final List<SongInfo> songs;
  final Function(SongInfo song)? onSongTap;

  const SongsListView({super.key, required this.songs, this.onSongTap});

  @override
  Widget build(BuildContext context) {
    return SliverList(
      delegate: SliverChildBuilderDelegate((context, index) {
        final song = songs[index];
        return SongTile(song: song);
      }, childCount: songs.length),
    );
  }
}
