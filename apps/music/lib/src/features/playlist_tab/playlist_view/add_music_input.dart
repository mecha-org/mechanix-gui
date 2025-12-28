import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:widgets/widgets/text_input/mechanix_text_input.dart';

class AddMusicInput extends StatefulWidget {
  const AddMusicInput({super.key});

  @override
  State<AddMusicInput> createState() => _AddMusicInputState();
}

class _AddMusicInputState extends State<AddMusicInput> {
  bool _isSearchActive = false;

  @override
  void dispose() {
    super.dispose();
  }

  void _handlePress() {
    context.read<SongsBloc>().add(SearchedSong(''));
    setState(() {
      _isSearchActive = false;
    });
  }

  @override
  Widget build(BuildContext context) {
    // Show buttons by default
    return Row(
      crossAxisAlignment: CrossAxisAlignment.center,
      spacing: 16,
      mainAxisAlignment: MainAxisAlignment.end,
      children: [
        if (_isSearchActive)
          Expanded(
            child: MechanixTextInput.search(
              autofocus: true,
              onChanged:
                  (value) => context.read<SongsBloc>().add(SearchedSong(value)),
              onClear: () => _handlePress(),
            ),
          )
        else
          IconButton(
            iconSize: 44,
            onPressed: () {
              setState(() {
                _isSearchActive = true;
              });
            },
            icon: Image.asset(MusicIcons.searchIcon, width: 28, height: 28),
          ),
      ],
    );
  }
}
