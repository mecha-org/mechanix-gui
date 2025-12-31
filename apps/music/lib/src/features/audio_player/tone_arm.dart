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
        return Positioned(
          right: -25,
          top: -10,
          child: AnimatedRotation(
            duration: const Duration(milliseconds: 400),
            curve: Curves.easeOutCubic,
            alignment: Alignment.topRight, // pivot point
            turns: isPlaying ? 0.0 : -0.1,
            child: Image.asset(MusicIcons.toneArm, height: 105, width: 97),
          ),
        );
      },
    );
  }
}
