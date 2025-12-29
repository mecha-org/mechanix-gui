import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/icons.dart';

class ToneArm extends StatelessWidget {
  const ToneArm({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocSelector<SongsBloc, SongsState, bool>(
      selector: (state) => state.isPlaying,
      builder: (context, isPlaying) {
        return isPlaying
            ? Positioned(
              right: -25,
              top: -25,
              child: Image.asset(MusicIcons.toneArm, height: 105, width: 97),
            )
            : Positioned(
              right: -25,
              top: -25,
              child: Image.asset(
                MusicIcons.pauseToneArmIcon,
                height: 129,
                width: 46,
              ),
            );
      },
    );
  }
}
