import 'dart:math' as math;
import 'package:flutter/material.dart';
import 'package:mechanix_music/models/song_info.dart';
import 'package:mechanix_music/src/commons/icons.dart';
import 'package:sleek_circular_slider/sleek_circular_slider.dart';

class PlayerVinyl extends StatefulWidget {
  final SongInfo songDetails;
  final double currentPosition;
  final ValueChanged<double> onPositionChange;
  final Duration currentDuration;
  final Duration totalDuration;
  final bool isPlaying; // <-- add this to control rotation

  const PlayerVinyl({
    super.key,
    required this.songDetails,
    required this.currentPosition,
    required this.onPositionChange,
    required this.currentDuration,
    required this.totalDuration,
    required this.isPlaying,
  });

  @override
  State<PlayerVinyl> createState() => _PlayerVinylState();
}

class _PlayerVinylState extends State<PlayerVinyl>
    with SingleTickerProviderStateMixin {
  late AnimationController controller;

  String _formatDuration(Duration d) {
    String twoDigits(int n) => n.toString().padLeft(2, "0");
    final minutes = twoDigits(d.inMinutes.remainder(60));
    final seconds = twoDigits(d.inSeconds.remainder(60));
    return "$minutes:$seconds";
  }

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
  }

  @override
  void didUpdateWidget(covariant PlayerVinyl oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (widget.isPlaying && !oldWidget.isPlaying) {
      controller.repeat(); // resume rotation
    } else if (!widget.isPlaying && oldWidget.isPlaying) {
      controller.stop(); // pause rotation, keep current angle
    }
  }

  @override
  void dispose() {
    controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Center(
      child: SizedBox(
        width: 380,
        height: 350,
        child: Stack(
          alignment: Alignment.center,
          children: [
            SizedBox(
              height: 350,
              width: 350,
              child: Stack(
                alignment: Alignment.center,
                children: [
                  AnimatedBuilder(
                    animation: controller,
                    builder: (context, child) {
                      return Transform.rotate(
                        angle: controller.value * 2 * math.pi,
                        child: SizedBox(
                          width: 300,
                          height: 300,
                          child: widget.songDetails.artwork != null
                              ? ClipRRect(
                                  borderRadius: BorderRadius.circular(300),
                                  child: Image.memory(
                                    widget.songDetails.artwork!,
                                    width: 300,
                                    height: 300,
                                    fit: BoxFit.cover,
                                  ),
                                )
                              : Image.asset(
                                  MusicIcons.audioImage,
                                  fit: BoxFit.cover,
                                ),
                        ),
                      );
                    },
                  ),
                  Positioned(
                    bottom: 0,
                    child: SizedBox(
                      width: 350,
                      height: 350,
                      child: SleekCircularSlider(
                        min: 0,
                        max: widget.totalDuration.inSeconds.toDouble(),
                        initialValue:
                            widget.currentDuration.inSeconds.toDouble(),
                        innerWidget: (_) => const SizedBox(),
                        appearance: CircularSliderAppearance(
                          counterClockwise: true,
                          startAngle: 180,
                          angleRange: 180,
                          size: 500,
                          customWidths: CustomSliderWidths(
                            progressBarWidth: 6,
                            trackWidth: 6,
                            handlerSize: 8,
                          ),
                          customColors: CustomSliderColors(
                            progressBarColor: Colors.blueAccent,
                            trackColor: Colors.grey.shade800,
                            dotColor: Colors.white,
                            hideShadow: true,
                          ),
                        ),
                        onChange: (v) {
                          final newDuration = Duration(seconds: v.round());
                          widget.onPositionChange(
                            newDuration.inSeconds /
                                widget.totalDuration.inSeconds,
                          );
                        },
                      ),
                    ),
                  ),
                ],
              ),
            ),
            // Start time
            Positioned(
              top: 140,
              left: 0,
              child: Text(
                _formatDuration(widget.currentDuration),
                style: const TextStyle(color: Colors.white),
              ),
            ),
            // End time
            Positioned(
              top: 140,
              right: -5,
              child: Text(
                _formatDuration(widget.totalDuration),
                style: const TextStyle(color: Colors.white),
              ),
            ),
            Positioned(
              right: 15,
              top: 5,
              child: Image.asset(MusicIcons.toneArm, height: 105, width: 97),
            ),
          ],
        ),
      ),
    );
  }
}
