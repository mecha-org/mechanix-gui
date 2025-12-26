import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_event.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:widgets/mechanix.dart';

class AddPlaylistBar extends StatelessWidget {
  const AddPlaylistBar({super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.end,
      mainAxisSize: MainAxisSize.min,

      children: [
        MechanixTextInput.textInput(
          autofocus: true,
          getCurrentValue:
              (value) => {
                if (value.trim().isNotEmpty)
                  context.read<SongsBloc>().add(CreatePlaylist(value)),
              },
          anchorWidget: IconWidget(
            boxHeight: 44,
            boxWidth: 44,
            iconHeight: 24,
            iconWidth: 24,
            iconPath: MusicIcons.checkIcon,
          ),
        ),
      ],
    );
  }
}
