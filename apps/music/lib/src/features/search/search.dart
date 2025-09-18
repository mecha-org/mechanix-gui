import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/src/features/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/features/bloc/songs_event.dart';
import 'package:mechanix_music/src/features/bloc/songs_state.dart';
import 'package:mechanix_music/src/features/presentation/song_list_view.dart';
import 'package:mechanix_music/src/features/search/music_search_bar.dart';
import 'package:widgets/mechanix.dart';

class SearchPage extends StatefulWidget {
  const SearchPage({super.key});

  @override
  State<SearchPage> createState() => _SearchPageState();
}

class _SearchPageState extends State<SearchPage> {
  @override
  void initState() {
    super.initState();
  }

  void onBackClick() {
    Navigator.pop(context);

    context.read<SongsBloc>().add(SearchSong(''));
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<SongsBloc, SongsState>(
      builder: (context, state) {
        return Scaffold(
          appBar: MechanixNavigationBar(
            title: "Search Song",
            titleStyle: const TextStyle(
              fontSize: 18,
              fontWeight: FontWeight.normal,
            ),
          ),
          body: Stack(
            children: [
              Positioned.fill(
                child: SingleChildScrollView(
                  padding: const EdgeInsets.all(16),
                  child: Column(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      SongsListView(
                        songs: state.searchedSongs,
                        onSongTap: (song) {},
                      ),

                      const SizedBox(height: 80),
                    ],
                  ),
                ),
              ),

              Positioned(
                left: 0,
                right: 0,
                bottom: 30,
                child: Center(child: MusicSearchBar()),
              ),
            ],
          ),
        );
      },
    );
  }
}
