import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/features/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/features/bloc/songs_event.dart';
import 'package:widgets/mechanix.dart';

class PlayerHeader extends StatelessWidget {
  final SongInfo songDetails;
  final bool isFavorited;
  const PlayerHeader({
    super.key,
    required this.songDetails,
    required this.isFavorited,
  });

  @override
  Widget build(BuildContext context) {
    void onBackPressed() {
      if (songDetails.isFavourite != isFavorited) {
        context.read<SongsBloc>().add(FavouriteToggle());
      }
      Navigator.pop(context);
    }

    return Container(
      padding: const EdgeInsets.only(left: 20,right: 20, bottom: 0,top:20),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          IconButton(
            onPressed: () => onBackPressed(),
            icon: Image.asset(
              MechanixIconImages.backIcon,
              height: 20,
              width: 20,
              package: 'widgets',
            ),
          ),
          Expanded(
            child: Column(
              children: [
                Text(
                  '${songDetails.title}${songDetails.album != null ? ' (${songDetails.album})' : ''}',
                  style: const TextStyle(
                    color: Colors.white,
                    fontSize: 18,
                    fontWeight: FontWeight.w500,
                  ),
                  textAlign: TextAlign.center,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                ),
                const SizedBox(height: 4),
                Text(
                  songDetails.artist,
                  style: const TextStyle(color: Colors.grey, fontSize: 14),
                  textAlign: TextAlign.center,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                ),
              ],
            ),
          ),
          // IconButton(
          //   onPressed: () {},
          //   icon: const Icon(Icons.menu, color: Colors.white, size: 24),
          // ),
        ],
      ),
    );
  }
}
