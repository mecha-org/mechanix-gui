import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_music/src/bloc/songs_bloc.dart';
import 'package:widgets/extensions/color.dart';

class UpcomingSongProgressBar extends StatefulWidget {
  const UpcomingSongProgressBar({super.key});

  @override
  State<UpcomingSongProgressBar> createState() =>
      _UpcomingSongProgressBarState();
}

class _UpcomingSongProgressBarState extends State<UpcomingSongProgressBar> {
  double? _dragValue;
  bool _isDragging = false;

  String _format(Duration d) {
    final minutes = d.inMinutes.remainder(60).toString().padLeft(2, '0');
    final seconds = d.inSeconds.remainder(60).toString().padLeft(2, '0');
    return '$minutes:$seconds';
  }

  @override
  Widget build(BuildContext context) {
    final player = context.read<SongsBloc>().player;

    return StreamBuilder<Duration>(
      stream: player.stream.position,
      initialData: Duration.zero,
      builder: (context, snapshot) {
        final position = snapshot.data ?? Duration.zero;
        final total = player.state.duration;

        final progress =
            total.inMilliseconds == 0
                ? 0.0
                : position.inMilliseconds / total.inMilliseconds;

        final sliderValue = _isDragging ? (_dragValue ?? progress) : progress;

        return Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            /// SLIDER
            Expanded(
              child: SliderTheme(
                data: SliderThemeData(
                  padding: const EdgeInsets.symmetric(
                    vertical: 6,
                    horizontal: 4,
                  ),
                  trackHeight: 4,
                  thumbShape: const RoundSliderThumbShape(
                    enabledThumbRadius: 5,
                  ),
                  overlayColor: Colors.transparent,
                  overlayShape: const RoundSliderOverlayShape(
                    overlayRadius: 15,
                  ),
                  activeTrackColor: context.onSurface,
                  inactiveTrackColor: context.surfaceContainer,

                  thumbColor: context.onSecondaryFixedVariant,
                ),
                child: Slider(
                  value: sliderValue.clamp(0.0, 1.0),
                  min: 0,
                  max: 1,
                  onChangeStart: (value) {
                    setState(() {
                      _isDragging = true;
                      _dragValue = value;
                    });
                  },
                  onChanged: (value) {
                    setState(() {
                      _dragValue = value;
                    });
                  },
                  onChangeEnd: (value) {
                    player.seek(total * value);
                    setState(() {
                      _isDragging = false;
                      _dragValue = null;
                    });
                  },
                ),
              ),
            ),

            /// TIME TEXT (fixed width = stable spacing)
            SizedBox(
              width: 48,
              child: Text(
                _format(_isDragging ? total * sliderValue : position),
                textAlign: TextAlign.right,
                style: TextStyle(
                  color: context.onSecondaryFixedVariant,
                  fontSize: 16,
                  fontWeight: FontWeight.w500,
                ),
              ),
            ),
          ],
        );
      },
    );
  }
}
