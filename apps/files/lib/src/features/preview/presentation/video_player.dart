import 'dart:async';
import 'package:flutter/material.dart';
import 'package:mechanix_files/src/services/media_kit_manager.dart';
import 'package:media_kit/media_kit.dart';
import 'package:media_kit_video/media_kit_video.dart';
import 'package:widgets/mechanix.dart';

enum ZoomMode { stretch, original }

class VideoPlayer extends StatefulWidget {
  final String filePath;

  const VideoPlayer({super.key, required this.filePath});

  @override
  State<VideoPlayer> createState() => _VideoPlayerState();
}

class _VideoPlayerState extends State<VideoPlayer> {
  late final Player player;
  late final VideoController videoController;
  final ValueNotifier<bool> isLooping = ValueNotifier(false);
  double _lastVolume = 50; // Store user's volume before muting

  bool _controlsVisible = true;
  Timer? _hideControlsTimer;

  final bool _showOptions = false;
  double _playbackRate = 1.0;
  ZoomMode _zoomMode = ZoomMode.original;

  Duration _currentPosition = Duration.zero;
  Duration _totalDuration = Duration.zero;

  late final StreamSubscription<Duration> _positionSub;
  late final StreamSubscription<Duration?> _durationSub;

  bool _isPlaying = false;
  late final StreamSubscription<bool> _playingSub;

  double _localVolume = 50; // Default volume at 50%
  bool _isPlayerDisposed = false;
  bool _isInitialized = false;

  @override
  void initState() {
    super.initState();
    _initializePlayer();
  }

  Future<void> _initializePlayer() async {
    await MediaKitManager.init();
    player = Player();
    videoController = VideoController(player);
    await player.open(Media(widget.filePath));

    _positionSub = player.stream.position.listen((position) {
      if (!mounted) return;
      setState(() => _currentPosition = position);
    });

    _durationSub = player.stream.duration.listen((duration) {
      if (!mounted) return;
      setState(() => _totalDuration = duration ?? Duration.zero);
    });

    _playingSub = player.stream.playing.listen((playing) {
      if (!mounted) return;
      setState(() => _isPlaying = playing);
    });

    if (!mounted) return;
    setState(() => _isInitialized = true);
  }

  void _startHideTimer() {
    _hideControlsTimer?.cancel();
    _hideControlsTimer = Timer(const Duration(seconds: 3), () {
      if (!mounted) return;
      setState(() {
        _controlsVisible = false;
      });
    });
  }

  void _handleGesture() {
    setState(() {
      _controlsVisible = true;
    });
    _startHideTimer(); // Restart the timer to auto-hide again
  }

  @override
  void dispose() {
    _positionSub.cancel();
    _durationSub.cancel();
    _playingSub.cancel();

    // Don't call player.dispose() here if already done in _backNavigation
    if (!_isPlayerDisposed) {
      player.dispose();
    }

    _hideControlsTimer?.cancel();
    super.dispose();
  }

  void _backNavigation() async {
    if (!_isPlayerDisposed) {
      await player.pause();
      await player.dispose();
      _isPlayerDisposed = true;
    }
    if (mounted) Navigator.pop(context);
  }

  void _showBottomOptions(BuildContext context) {
    showModalBottomSheet(
      context: context,
      backgroundColor: Colors.black.withOpacity(0.6),
      builder: (_) {
        return StatefulBuilder(
          builder: (_, setModalState) {
            return Padding(
              padding: const EdgeInsets.all(16),
              child: SingleChildScrollView(
                child: _buildOptionsPanel(setModalState),
              ),
            );
          },
        );
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    if (!_isInitialized) {
      return const Scaffold(
        body: Center(child: CircularProgressIndicator()),
      );
    }

    return Scaffold(
      body: Listener(
        behavior: HitTestBehavior.opaque, // ensures all taps are detected
        onPointerDown: (_) => _handleGesture(),
        onPointerMove: (_) => _handleGesture(),
        child: Stack(
          children: [
            // Video content with optional zoom styling
            Center(
              child: _buildZoomedVideo(),
            ),

            // Back button & title (only visible when _controlsVisible)
            if (_controlsVisible)
              Positioned(
                top: 30,
                left: 16,
                right: 16,
                child: Row(
                  children: [
                    IconButton(
                      icon: const Icon(Icons.arrow_back_ios,
                          color: Colors.blue, size: 16),
                      onPressed: _backNavigation,
                    ),
                    const SizedBox(width: 8),
                    Expanded(
                      child: Text(
                        widget.filePath.split('/').last,
                        style: context.textTheme.bodySmall,
                        overflow: TextOverflow.ellipsis,
                      ),
                    ),
                  ],
                ),
              ),

            // Right-side control buttons (only visible when _controlsVisible || _showOptions)
            if (_controlsVisible || _showOptions)
              Positioned(
                right: 16,
                top: MediaQuery.of(context).size.height * 0.3,
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.end,
                  children: [
                    StreamBuilder<double>(
                      stream: player.stream.volume,
                      builder: (_, snapshot) {
                        final volume = snapshot.data ?? _lastVolume;
                        final isMuted = volume == 0;

                        // Update _lastVolume only when not muted
                        if (!isMuted) {
                          _lastVolume = volume;
                        }

                        return _circularIconButton(
                          icon: isMuted ? Icons.volume_off : Icons.volume_up,
                          onPressed: () {
                            if (isMuted) {
                              player.setVolume(_lastVolume);
                            } else {
                              player.setVolume(0);
                            }
                          },
                        );
                      },
                    ),
                    const SizedBox(height: 8),
                    if (_showOptions) _buildOptionsPanel((_) {}),
                    _circularIconButton(
                      icon: Icons.more_horiz,
                      onPressed: () => _showBottomOptions(context),
                    ),
                  ],
                ),
              ),

            if (_controlsVisible && !_showOptions)
              Center(
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    _circularIconButton(
                      icon: Icons.replay_10,
                      onPressed: () {
                        final newPosition =
                            _currentPosition - const Duration(seconds: 10);
                        player.seek(newPosition > Duration.zero
                            ? newPosition
                            : Duration.zero);
                      },
                    ),
                    const SizedBox(width: 16),
                    _circularIconButton(
                      icon: _isPlaying ? Icons.pause : Icons.play_arrow,
                      onPressed: () {
                        if (_isPlaying) {
                          player.pause();
                        } else {
                          player.play();
                        }
                      },
                    ),
                    const SizedBox(width: 16),
                    _circularIconButton(
                      icon: Icons.forward_10,
                      onPressed: () {
                        final newPosition =
                            _currentPosition + const Duration(seconds: 10);
                        player.seek(newPosition < _totalDuration
                            ? newPosition
                            : _totalDuration);
                      },
                    ),
                  ],
                ),
              ),
          ],
        ),
      ),
    );
  }

  Widget _buildOptionsPanel(void Function(void Function()) setModalState) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      children: [
        const Text('Playback speed', style: TextStyle(color: Colors.white70)),
        const SizedBox(height: 8),
        Wrap(
          spacing: 8,
          runSpacing: 8,
          children: [0.5, 1.0, 1.25, 1.5, 2.0].map((rate) {
            return ChoiceChip(
              label: Text(
                rate.toStringAsFixed(2).replaceAll(RegExp(r'\.0+$'), ''),
                style: const TextStyle(color: Colors.white),
              ),
              selected: _playbackRate == rate,
              onSelected: (_) {
                player.setRate(rate);
                setState(() => _playbackRate = rate);
                setModalState(() {}); // Triggers UI update inside bottom sheet
              },
              selectedColor: Colors.blue,
              backgroundColor: Colors.white24,
            );
          }).toList(),
        ),
        const SizedBox(height: 16),
        const Text('Zoom', style: TextStyle(color: Colors.white70)),
        const SizedBox(height: 8),
        Wrap(
          spacing: 8,
          children: ZoomMode.values.map((mode) {
            IconData icon = mode == ZoomMode.stretch
                ? Icons.open_in_full
                : Icons.fullscreen;
            return ChoiceChip(
              label: Icon(icon, color: Colors.white),
              selected: _zoomMode == mode,
              onSelected: (_) {
                setState(() => _zoomMode = mode);
                setModalState(() {});
              },
              selectedColor: Colors.blue,
              backgroundColor: Colors.white24,
            );
          }).toList(),
        ),
        const SizedBox(height: 16),
        const Text('Volume', style: TextStyle(color: Colors.white70)),
        const SizedBox(height: 8),
        StreamBuilder<double>(
          stream: player.stream.volume,
          builder: (_, snapshot) {
            final volume = snapshot.data ?? _lastVolume;
            return Slider(
              value: volume.clamp(0, 100),
              min: 0,
              max: 100,
              divisions: 20,
              //label: '${volume.round()}%',
              onChanged: (value) {
                // Only update UI during dragging
                setModalState(() {
                  _localVolume = value;
                });
              },
              onChangeEnd: (value) {
                // Apply volume to player once user finishes sliding
                player.setVolume(value);
                setState(() {
                  _lastVolume = value;
                });
              },
              activeColor: Colors.blue,
              inactiveColor: Colors.white24,
            );
          },
        ),
      ],
    );
  }

  Widget _buildZoomedVideo() {
    final video = Video(
      controller: videoController,
    );

    switch (_zoomMode) {
      case ZoomMode.original:
        return SizedBox.expand(child: video);

      case ZoomMode.stretch:
        return Center(
          child: Transform.scale(
            scale: 1.2,
            child: video,
          ),
        );
    }
  }

  Widget _circularIconButton({
    required IconData icon,
    required VoidCallback onPressed,
  }) {
    return Container(
      decoration: BoxDecoration(
        color: Colors.black.withOpacity(0.6),
        shape: BoxShape.circle,
      ),
      child: IconButton(
        icon: Icon(icon, size: 30, color: Colors.white),
        onPressed: onPressed,
      ),
    );
  }
}
