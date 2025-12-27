import 'dart:math' as math;
import 'dart:async';

import 'package:flutter/material.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/audio_player/circle.dart';

class PlayerDiscRotate extends StatefulWidget {
  final SongInfo songDetails;
  final bool isPlaying;

  const PlayerDiscRotate({
    super.key,
    required this.songDetails,
    required this.isPlaying,
  });

  @override
  State<PlayerDiscRotate> createState() => _PlayerDiscRotateState();
}

class _PlayerDiscRotateState extends State<PlayerDiscRotate>
    with SingleTickerProviderStateMixin {
  late AnimationController controller;
  
  // Dummy duration variables for testing
  Duration currentDuration = Duration.zero;
  Duration totalDuration = const Duration(minutes: 3, seconds: 30); // 3:30 total
  Timer? _durationTimer;

  @override
  void initState() {
    super.initState();
    controller = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 20), // full rotation time
    )..repeat(); // keep rotating forever

    if (!widget.isPlaying) {
      controller.stop(); // start in paused state if not playing
    }

    // Start the duration timer
    _startDurationTimer();
  }

  void _startDurationTimer() {
    _durationTimer = Timer.periodic(const Duration(seconds: 1), (timer) {
      if (widget.isPlaying && currentDuration < totalDuration) {
        setState(() {
          currentDuration += const Duration(seconds: 1);
        });
      }
      
      // Loop back to start when finished
      if (currentDuration >= totalDuration) {
        setState(() {
          currentDuration = Duration.zero;
        });
      }
    });
  }

  @override
  void didUpdateWidget(covariant PlayerDiscRotate oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (widget.isPlaying && !oldWidget.isPlaying) {
      controller.repeat(); // resume rotation
    } else if (!widget.isPlaying && oldWidget.isPlaying) {
      controller.stop(); // pause rotation, keep current angle
    }
  }

  void _handlePositionChange(Duration newPosition) {
    setState(() {
      currentDuration = newPosition;
    });
    print('Position changed to: ${newPosition.inSeconds}s');
    // Here you would typically seek your audio player to this position
  }

  @override
  void dispose() {
    controller.dispose();
    _durationTimer?.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Stack(
      clipBehavior: Clip.none,
      children: [
        AnimatedBuilder(
          animation: controller,
          builder: (context, child) {
            return Transform.rotate(
              angle: controller.value * 2 * math.pi,
              child: SizedBox(
                width: 300,
                height: 300,
                child: widget.songDetails.artworkPath != null
                    ? ClipRRect(
                        borderRadius: BorderRadius.circular(300),
                        child: Image.asset(
                          widget.songDetails.artworkPath!,
                          width: 300,
                          height: 300,
                          fit: BoxFit.cover,
                        ),
                      )
                    : Image.asset(
                        MusicIcons.audioImage,
                        width: 300,
                        height: 300,
                        fit: BoxFit.cover,
                      ),
              ),
            );
          },
        ),
        Positioned(
          right: -25,
          top: -25,
          child: Image.asset(MusicIcons.toneArm, height: 105, width: 97),
        ),
        Positioned(
          bottom: -30,
          left: -30,
          child: SemiCircularAudioProgress(
            currentDuration: currentDuration,
            totalDuration: totalDuration,
            onPositionChange: _handlePositionChange,
          ),
        ),
      ],
    );
  }
}