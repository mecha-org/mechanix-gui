import 'dart:math' as math;
import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_state.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:mechanix_music/src/features/audio_player/circle.dart';
import 'package:mechanix_music/src/features/audio_player/tone_arm.dart';

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

  // Real duration variables from audio player
  Duration currentDuration = Duration.zero;
  Duration totalDuration = Duration.zero;

  StreamSubscription<Duration>? _positionSubscription;
  StreamSubscription<Duration?>? _durationSubscription;

  // Drag rotation control
  bool _isDraggingVinyl = false;
  double _dragRotationVelocity = 0.0;
  Timer? _velocityDecayTimer;

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

    // Subscribe to real audio player streams
    _subscribeToPlayerStreams();
  }

  void _subscribeToPlayerStreams() {
    final player = context.read<SongsBloc>().player;

    // Listen to position changes
    _positionSubscription = player.stream.position.listen((position) {
      if (!_isDraggingVinyl) {
        setState(() {
          currentDuration = position;
        });
      }
    });

    // Listen to duration changes
    // _durationSubscription = player.stream.duration.listen((duration) {
    //   print("duration: $duration");
    //   setState(() {
    //     totalDuration = duration;
    //   });
    // });
  }

  @override
  void didUpdateWidget(covariant PlayerDiscRotate oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (widget.isPlaying && !oldWidget.isPlaying && !_isDraggingVinyl) {
      controller.repeat(); // resume rotation
    } else if (!widget.isPlaying && oldWidget.isPlaying) {
      controller.stop(); // pause rotation, keep current angle
    }
  }

  void _handlePositionChange(Duration newPosition) {
    // Only update the UI during drag, don't seek yet
    setState(() {
      currentDuration = newPosition;
    });
  }

  void _handleDragStart() {
    setState(() {
      _isDraggingVinyl = true;
      _dragRotationVelocity = 0.0;
    });
    controller.stop(); // Stop automatic rotation
    _velocityDecayTimer?.cancel();
  }

  void _handleDragUpdate(double angularVelocity) {
    setState(() {
      _dragRotationVelocity = angularVelocity;
    });

    // Manually rotate the disc based on velocity
    // Positive velocity = clockwise, negative = counter-clockwise
    final currentValue = controller.value;
    final increment =
        angularVelocity * 0.008; // INCREASED scale factor for faster rotation
    controller.value = (currentValue - increment) % 1.0;
  }

  void _handleDragEnd() {
    // Seek the player to the final position when drag is released
    final player = context.read<SongsBloc>().player;
    player.seek(currentDuration);

    // Apply momentum/decay effect
    _startVelocityDecay();
  }

  void _startVelocityDecay() {
    _velocityDecayTimer?.cancel();

    _velocityDecayTimer = Timer.periodic(const Duration(milliseconds: 16), (
      timer,
    ) {
      if (_dragRotationVelocity.abs() < 0.1) {
        timer.cancel();
        _dragRotationVelocity = 0.0;

        // NOW it's safe to mark drag as complete and resume rotation
        setState(() {
          _isDraggingVinyl = false;
        });

        // Resume rotation immediately if playing
        if (mounted && widget.isPlaying) {
          controller.repeat();
        }
        return;
      }

      // Decay the velocity smoothly
      setState(() {
        _dragRotationVelocity *= 0.95;
      });

      // Continue rotating with decaying velocity
      final currentValue = controller.value;
      final increment =
          _dragRotationVelocity * 0.008; // Match the increased scale
      controller.value = (currentValue + increment) % 1.0;
    });
  }

  @override
  void dispose() {
    controller.dispose();
    _positionSubscription?.cancel();
    _durationSubscription?.cancel();
    _velocityDecayTimer?.cancel();
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
              angle: -controller.value * 2 * math.pi,
              child: SizedBox(
                width: 300,
                height: 300,
                child: ClipRRect(
                  borderRadius: BorderRadius.circular(300),
                  child: Stack(
                    fit: StackFit.expand,
                    children: [
                      /// Base artwork image
                      widget.songDetails.artworkPath != null
                          ? Image.asset(
                            widget.songDetails.artworkPath!,
                            fit: BoxFit.cover,
                            errorBuilder: (context, error, stackTrace) {
                              return Image.asset(
                                MusicIcons.audioImage,
                                fit: BoxFit.cover,
                              );
                            },
                          )
                          : Image.asset(
                            MusicIcons.audioImage,
                            fit: BoxFit.cover,
                          ),

                      ///  Transparent overlay image
                      if (widget.songDetails.artworkPath != null)
                        Opacity(
                          opacity: 0.35, // adjust transparency here
                          child: Image.asset(
                            MusicIcons.audioImage,
                            fit: BoxFit.cover,
                          ),
                        ),
                    ],
                  ),
                ),
              ),
            );
          },
        ),
        BlocSelector<SongsBloc, SongsState, bool>(
          selector: (state) => state.isPlaying,
          builder: (context, isPlaying) {
            return ToneArm(isPlaying: isPlaying);
          },
        ),

        Positioned(
          bottom: -30,
          left: -30,
          child: SemiCircularAudioProgress(
            currentDuration: currentDuration,
            totalDuration: context.read<SongsBloc>().player.state.duration,
            onPositionChange: _handlePositionChange,
            onDragStart: _handleDragStart,
            onDragUpdate: _handleDragUpdate,
            onDragEnd: _handleDragEnd,
          ),
        ),
      ],
    );
  }
}
