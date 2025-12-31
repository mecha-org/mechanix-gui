import 'package:flutter/material.dart';
import 'package:mechanix_music/src/features/home/home_tab/recent_songs.dart';
import 'package:mechanix_music/src/features/home/widgets/title_widget.dart';
import 'package:widgets/mechanix.dart';

class HomeTab extends StatefulWidget {
  const HomeTab({super.key});

  @override
  State<HomeTab> createState() => _HomeTabState();
}

class _HomeTabState extends State<HomeTab> {
  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        spacing: 0,
        children: [
          TitleWidget(
            title: "Music",
            textStyle: TextStyle(fontSize: 32, fontWeight: FontWeight.w600),
          ).padOnly(left: 16, top: 12),
          RecentSongs(),
        ],
      ),
    );
  }
}
