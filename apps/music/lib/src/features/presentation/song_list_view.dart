import 'dart:typed_data';
import 'package:flutter/material.dart';
import 'package:mechanix_music/models/song_info.dart';

class SongsListView extends StatelessWidget {
  final List<SongInfo> songs;
  final Function(SongInfo song)? onSongTap;

  const SongsListView({super.key, required this.songs, this.onSongTap});

  @override
  Widget build(BuildContext context) {
    // if (songs.isEmpty) {
    //   return const Center(child: Text("No songs available"));
    // }

    return ListView.builder(
      shrinkWrap: true,
      itemCount: songs.length,
      itemBuilder: (context, i) {
        final song = songs[i];
        return ListTile(
          
          tileColor: Colors.transparent,
          hoverColor: Colors.transparent,
          focusColor: Colors.transparent,
          splashColor: Colors.transparent,
            selectedTileColor: Colors.transparent,
          leading: _buildArtworkOrIcon(
            artwork: song.artwork,
            fallbackIcon: Icons.music_note,
          ),
          title: Text(song.title),
          subtitle: Text(
            "${song.artist}${song.album != null && song.album!.isNotEmpty ? ' | ${song.album}' : ''}",
          ),
          onTap: () => onSongTap?.call(song),
        );
      },
    );
  }

  Widget _buildArtworkOrIcon({
    required Uint8List? artwork,
    required IconData fallbackIcon,
    double size = 40,
  }) {
    if (artwork != null) {
      return ClipRRect(
        borderRadius: BorderRadius.circular(50),
        child: Image.memory(
          artwork,
          width: size,
          height: size,
          fit: BoxFit.cover,
        ),
      );
    } else {
      return Container(
        width: size,
        height: size,
        decoration: BoxDecoration(
          color: Colors.grey.shade200,
          borderRadius: BorderRadius.circular(50),
        ),
        child: Icon(fallbackIcon, color: Colors.grey.shade600),
      );
    }
  }
}
