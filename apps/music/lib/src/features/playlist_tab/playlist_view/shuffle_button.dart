import 'package:flutter/material.dart';
import 'package:mechanix_music/src/commons/colors.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/home/common/music_icon_widget.dart';

class ShuffleButton extends StatelessWidget {
  final bool isShuffle;
  final VoidCallback onPressed;
  const ShuffleButton({
    super.key,
    required this.isShuffle,
    required this.onPressed,
  });
  @override
  Widget build(BuildContext context) {
    return MusicIconButton(
      onPressed: onPressed,
      icon: MusicIcons.shuffleIcon,
      iconColor: isShuffle ? MusicColors.titleColor : null,
    );
  }
}
