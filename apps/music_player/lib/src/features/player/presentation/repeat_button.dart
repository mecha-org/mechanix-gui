import 'package:flutter/material.dart';
import 'package:flutter_styled_toast/flutter_styled_toast.dart';
import 'package:media_kit/media_kit.dart';

class RepeatButton extends StatefulWidget {
  final Player player;
  const RepeatButton({super.key, required this.player});

  @override
  State<RepeatButton> createState() => _RepeatButtonState();
}

class _RepeatButtonState extends State<RepeatButton> {
  PlaylistMode _mode = PlaylistMode.none;

  @override
  void initState() {
    super.initState();
    _mode = widget.player.state.playlistMode;
  }

  Future<void> _toggleMode() async {
    String message = "";

    if (_mode == PlaylistMode.none) {
      await widget.player.setPlaylistMode(PlaylistMode.loop);
      setState(() => _mode = PlaylistMode.loop);
      message = "Current playlist is looped";
    } else if (_mode == PlaylistMode.loop) {
      await widget.player.setPlaylistMode(PlaylistMode.single);
      setState(() => _mode = PlaylistMode.single);
      message = "Single loop mode";
    } else {
      await widget.player.setPlaylistMode(PlaylistMode.none);
      setState(() => _mode = PlaylistMode.none);
      message = "Normal play mode";
    }

    showToast(
      message,
      context: context,
      animation: StyledToastAnimation.slideFromBottom,
      reverseAnimation: StyledToastAnimation.fade,
      position: StyledToastPosition.bottom,
      duration: const Duration(seconds: 2),
    );
  }

  @override
  Widget build(BuildContext context) {
    IconData icon;
    if (_mode == PlaylistMode.none) {
      icon = Icons.shuffle; // Random play
    } else if (_mode == PlaylistMode.loop) {
      icon = Icons.repeat; // Playlist loop
    } else {
      icon = Icons.repeat_one; // Single loop
    }

    return IconButton(
      icon: Icon(icon, color: Colors.white),
      onPressed: _toggleMode,
    );
  }
}
