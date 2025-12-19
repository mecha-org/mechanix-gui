import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';

class MusicSearchBar extends StatefulWidget {
  const MusicSearchBar({super.key});

  @override
  State<MusicSearchBar> createState() => _MusicSearchBarState();
}

class _MusicSearchBarState extends State<MusicSearchBar> {
  final TextEditingController _titleController = TextEditingController();

  void searchSongs() {
    context.read<SongsBloc>().add(SearchSong(_titleController.value.text));
  }

  @override
  void dispose() {
    _titleController.dispose();
    super.dispose();
  }

  void _handleBackwardPress() {
    final text = _titleController.text;
    if (text.isNotEmpty) {
      _titleController.text = text.substring(0, text.length - 1);
      _titleController.selection = TextSelection.fromPosition(
        TextPosition(offset: _titleController.text.length),
      );
      searchSongs();
    }
  }

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 56,
      width: 508,
      // child: MechanixSearchBar(
      //   controller: _titleController,
      //   onChanged: (value) => searchSongs(),
      //   autoFocus: true,
      //   hintText: "Song Name",
      //   onBackwardIconPress: _handleBackwardPress,
      //   onCloseIconPress: () => _titleController.clear(),
      // ),
    );
  }
}
