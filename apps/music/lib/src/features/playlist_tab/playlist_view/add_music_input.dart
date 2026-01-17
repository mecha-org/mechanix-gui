import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/presentation/songs_icon.dart';
import 'package:widgets/widgets/text_input/mechanix_text_input.dart';

class AddMusicInput extends StatefulWidget {
  final ValueChanged<String> onSearch;

  const AddMusicInput({super.key, required this.onSearch});

  @override
  State<AddMusicInput> createState() => _AddMusicInputState();
}

class _AddMusicInputState extends State<AddMusicInput> {
  bool _isSearchActive = false;

  void _closeSearch() {
    context.read<SongsBloc>().add(SearchedSong(''));
    widget.onSearch('');
    setState(() => _isSearchActive = false);
  }

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.end,
      crossAxisAlignment: CrossAxisAlignment.center,
      children: [
        Expanded(
          child: AnimatedSwitcher(
            duration: const Duration(milliseconds: 300),
            switchInCurve: Curves.easeOut,
            switchOutCurve: Curves.easeIn,
            transitionBuilder: (child, animation) {
              final isSearch = child.key == const ValueKey('search');

              final offsetAnimation = Tween<Offset>(
                begin:
                    isSearch
                        ? const Offset(1.0, 0.0) // from right
                        : const Offset(-1.0, 0.0), // from left
                end: Offset.zero,
              ).animate(animation);

              return SlideTransition(
                position: offsetAnimation,
                child: FadeTransition(opacity: animation, child: child),
              );
            },
            child:
                _isSearchActive
                    ? MechanixTextInput.search(
                      key: const ValueKey('search'),
                      autofocus: true,
                      onChanged: (value) {
                        context.read<SongsBloc>().add(SearchedSong(value));
                        widget.onSearch(value);
                      },
                      onClear: _closeSearch,
                    )
                    : Align(
                      key: const ValueKey('icon'),
                      alignment: Alignment.centerRight,
                      child: IconButton(
                        iconSize: 44,
                        onPressed: () {
                          setState(() => _isSearchActive = true);
                        },
                        icon: SongsIcon(
                          iconPath: MusicIcons.searchIcon,
                          boxSize: 28,
                          iconSize: 28,
                        ),
                      ),
                    ),
          ),
        ),
      ],
    );
  }
}
