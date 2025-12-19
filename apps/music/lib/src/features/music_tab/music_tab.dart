import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/features/home/widgets/title_widget.dart';
import 'package:mechanix_music/src/features/presentation/song_list_view.dart';

class MusicTab extends StatefulWidget {
  const MusicTab({super.key});

  @override
  State<MusicTab> createState() => _MusicTabState();
}

class _MusicTabState extends State<MusicTab> {
  final scrollController = ScrollController();
  @override
  Widget build(BuildContext context) {
    return Scrollbar(
      controller: scrollController,
      child: ScrollConfiguration(
        behavior: const ScrollBehavior().copyWith(
          overscroll: false,
          scrollbars:
              false, // Disable default scrollbar since we're using Scrollbar widget
          dragDevices: {PointerDeviceKind.touch, PointerDeviceKind.mouse},
        ),
        child: Container(
          padding: EdgeInsets.only(left: 16, right: 16, top: 12, bottom: 6),
          child: CustomScrollView(
            controller: scrollController,
            slivers: [
              // Title widget is now part of the scrollable content
              const SliverToBoxAdapter(child: TitleWidget(title: "My Music")),

              BlocSelector<SongsBloc, SongsState, List<SongInfo>>(
                selector: (state) => state.songs,
                builder: (context, state) => SongsListView(songs: state),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
