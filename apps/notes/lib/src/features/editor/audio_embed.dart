import 'dart:async';
import 'package:flutter/material.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:media_kit/media_kit.dart';

// Security utilities
// class AudioSecurity {
//   static const List<String> allowedExtensions = [
//     '.mp3',
//     '.wav',
//     '.aac',
//     '.m4a',
//     '.ogg',
//     '.flac',
//   ];

//   static const int maxFileSizeBytes = 50 * 1024 * 1024; // 50MB limit

//   static bool isValidAudioFile(String filePath) {
//     if (filePath.isEmpty) return false;

//     // Check file extension
//     final extension = path.extension(filePath).toLowerCase();
//     if (!allowedExtensions.contains(extension)) return false;

//     // Sanitize path to prevent directory traversal
//     final normalizedPath = path.normalize(filePath);
//     if (normalizedPath.contains('..') || normalizedPath.startsWith('/')) {
//       return false;
//     }

//     return true;
//   }

//   static Future<bool> validateAudioFile(String filePath) async {
//     try {
//       if (!isValidAudioFile(filePath)) return false;

//       final file = File(filePath);
//       if (!await file.exists()) return false;

//       // Check file size
//       final fileSize = await file.length();
//       if (fileSize > maxFileSizeBytes || fileSize == 0) return false;

//       return true;
//     } catch (e) {
//       debugPrint('Audio file validation error: $e');
//       return false;
//     }
//   }
// }

class AudioEmbed extends Embeddable {
  const AudioEmbed(String value) : super(audioType, value);

  static const String audioType = 'audio';

  static AudioEmbed fromPath(String audioPath) {
    // Always return an AudioEmbed - validation will happen in the widget
    return AudioEmbed(audioPath);
  }

  String get audioPath => data;
}

class AudioEmbedBuilder extends EmbedBuilder {
  @override
  String get key => 'audio';

  @override
  String toPlainText(Embed node) {
    return '[Audio Recording]';
  }

  @override
  Widget build(BuildContext context, EmbedContext embedContext) {
    final audioPath = embedContext.node.value.data as String;
    return AudioPlayerWidget(audioPath: audioPath);
  }
}

class AudioPlayerWidget extends StatefulWidget {
  final String audioPath;

  const AudioPlayerWidget({super.key, required this.audioPath});

  @override
  State<AudioPlayerWidget> createState() => _AudioPlayerWidgetState();
}

class _AudioPlayerWidgetState extends State<AudioPlayerWidget>
    with WidgetsBindingObserver {
  Player? _player;
  bool _isPlaying = false;
  // bool _isLoading = true;
  bool _hasError = false;
  String _errorMessage = '';

  Duration _duration = Duration.zero;
  Duration _position = Duration.zero;
  // bool _isMuted = false;

  // Stream subscriptions for proper cleanup
  StreamSubscription<Duration>? _durationSubscription;
  StreamSubscription<Duration>? _positionSubscription;
  StreamSubscription<bool>? _playingSubscription;
  // StreamSubscription<double>? _volumeSubscription;

  Timer? _positionUpdateTimer;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);
    _initializePlayer();
  }

  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    super.didChangeAppLifecycleState(state);
    // Pause playback when app goes to background
    if (state == AppLifecycleState.paused && _isPlaying) {
      _pauseAudio();
    }
  }

  Future<void> _initializePlayer() async {
    try {
      // Validate audio file before proceeding
      // if (!await AudioSecurity.validateAudioFile(widget.audioPath)) {
      //   setState(() {
      //     _hasError = true;
      //     _errorMessage = 'Invalid or corrupted audio file';
      //     _isLoading = false;
      //   });
      //   return;
      // }

      _player = Player();
      await _setupPlayerListeners();
      await _loadAudio();
    } catch (e) {
      debugPrint('Player initialization error: $e');
      setState(() {
        _hasError = true;
        _errorMessage = 'Failed to initialize audio player';
        // _isLoading = false;
      });
    }
  }

  Future<void> _setupPlayerListeners() async {
    if (_player == null) return;

    try {
      // Duration listener
      _durationSubscription = _player!.stream.duration.listen(
        (duration) {
          if (mounted) {
            setState(() {
              _duration = duration;
              // _isLoading = false;
            });
          }
        },
        onError: (error) {
          debugPrint('Duration stream error: $error');
        },
      );

      // Position listener
      _positionSubscription = _player!.stream.position.listen(
        (position) {
          if (mounted) {
            setState(() => _position = position);
          }
        },
        onError: (error) {
          debugPrint('Position stream error: $error');
        },
      );

      // Playing state listener
      _playingSubscription = _player!.stream.playing.listen(
        (playing) {
          if (mounted) {
            setState(() => _isPlaying = playing);
          }
        },
        onError: (error) {
          debugPrint('Playing stream error: $error');
        },
      );

      // Volume listener
      // _volumeSubscription = _player!.stream.volume.listen(
      //   (volume) {
      //     if (mounted) {
      //       setState(() {
      //         _isMuted = volume == 0;
      //       });
      //     }
      //   },
      //   onError: (error) {
      //     debugPrint('Volume stream error: $error');
      //   },
      // );
    } catch (e) {
      debugPrint('Error setting up player listeners: $e');
    }
  }

  Future<void> _loadAudio() async {
    if (_player == null) return;

    try {
      final media = Media(widget.audioPath);
      await _player!.open(media, play: false);
    } catch (e) {
      debugPrint('Audio loading error: $e');
      if (mounted) {
        setState(() {
          _hasError = true;
          _errorMessage = 'Failed to load audio file';
          // _isLoading = false;
        });
      }
    }
  }

  Future<void> _playPause() async {
    if (_player == null || _hasError) return;

    try {
      if (_isPlaying) {
        await _pauseAudio();
      } else {
        await _playAudio();
      }
    } catch (e) {
      debugPrint('Play/pause error: $e');
      _showErrorSnackBar('Failed to control playback');
    }
  }

  Future<void> _playAudio() async {
    if (_player == null) return;
    await _player!.play();
  }

  Future<void> _pauseAudio() async {
    if (_player == null) return;
    await _player!.pause();
  }

  // Future<void> _seek(Duration position) async {
  //   if (_player == null || _hasError) return;

  //   try {
  //     // Clamp position to valid range
  //     final clampedPosition = Duration(
  //       milliseconds: position.inMilliseconds.clamp(
  //         0,
  //         _duration.inMilliseconds,
  //       ),
  //     );
  //     await _player!.seek(clampedPosition);
  //   } catch (e) {
  //     debugPrint('Seek error: $e');
  //     _showErrorSnackBar('Failed to seek');
  //   }
  // }

  // Future<void> _toggleMute() async {
  //   if (_player == null || _hasError) return;

  //   try {
  //     final newVolume = _isMuted ? 100.0 : 0.0;
  //     await _player!.setVolume(newVolume);
  //   } catch (e) {
  //     debugPrint('Mute toggle error: $e');
  //   }
  // }

  void _showErrorSnackBar(String message) {
    if (!mounted) return;

    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(
        content: Text(message),
        backgroundColor: Colors.red,
        duration: const Duration(seconds: 3),
      ),
    );
  }

  String _formatDuration(Duration duration) {
    if (duration.inHours > 0) {
      return '${duration.inHours}:${(duration.inMinutes % 60).toString().padLeft(2, '0')}:${(duration.inSeconds % 60).toString().padLeft(2, '0')}';
    }
    return '${duration.inMinutes}:${(duration.inSeconds % 60).toString().padLeft(2, '0')}';
  }

  Widget _buildErrorWidget() {
    return Container(
      margin: const EdgeInsets.symmetric(vertical: 8),
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Colors.red.shade50,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: Colors.red.shade200),
      ),
      child: Row(
        children: [
          Icon(Icons.error_outline, color: Colors.red.shade600, size: 24),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisSize: MainAxisSize.min,
              children: [
                Text(
                  'Audio Error',
                  style: TextStyle(
                    fontWeight: FontWeight.w600,
                    color: Colors.red.shade800,
                  ),
                ),
                const SizedBox(height: 4),
                Text(
                  _errorMessage,
                  style: TextStyle(fontSize: 12, color: Colors.red.shade700),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  // Widget _buildLoadingWidget() {
  //   return Container(
  //     margin: const EdgeInsets.symmetric(vertical: 8),
  //     padding: const EdgeInsets.all(16),
  //     decoration: BoxDecoration(
  //       color:
  //           Theme.of(context).brightness == Brightness.dark
  //               ? Colors.grey.shade800
  //               : Colors.grey.shade200,
  //       borderRadius: BorderRadius.circular(12),
  //       border: Border.all(
  //         color:
  //             Theme.of(context).brightness == Brightness.dark
  //                 ? Colors.grey.shade600
  //                 : Colors.grey.shade300,
  //       ),
  //     ),
  //     child: const Row(
  //       children: [
  //         SizedBox(
  //           width: 24,
  //           height: 24,
  //           child: CircularProgressIndicator(strokeWidth: 2),
  //         ),
  //         SizedBox(width: 16),
  //         Text('Loading audio...'),
  //       ],
  //     ),
  //   );
  // }

  @override
  Widget build(BuildContext context) {
    if (_hasError) {
      return _buildErrorWidget();
    }

    // if (_isLoading) {
    //   return _buildLoadingWidget();
    // }

    final isDark = Theme.of(context).brightness == Brightness.dark;
    final backgroundColor =
        isDark ? Colors.grey.shade800 : Colors.grey.shade100;
    final borderColor = isDark ? Colors.grey.shade600 : Colors.grey.shade300;

    return Container(
      margin: const EdgeInsets.symmetric(vertical: 8),
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: backgroundColor,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: borderColor),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withOpacity(0.05),
            blurRadius: 8,
            offset: const Offset(0, 2),
          ),
        ],
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisSize: MainAxisSize.min,
        children: [
          // Header
          Row(
            children: [
              Icon(
                Icons.audiotrack,
                size: 20,
                color: isDark ? Colors.grey.shade400 : Colors.grey.shade600,
              ),
              const SizedBox(width: 8),
              const Expanded(
                child: Text(
                  'Audio Recording',
                  style: TextStyle(fontWeight: FontWeight.w600, fontSize: 14),
                ),
              ),
              // IconButton(
              //   // onPressed: _toggleMute,
              //   icon: Icon(_isMuted ? Icons.volume_off : Icons.volume_up),
              //   iconSize: 20,
              //   padding: const EdgeInsets.all(4),
              //   constraints: const BoxConstraints(minWidth: 32, minHeight: 32),
              // ),
            ],
          ),

          const SizedBox(height: 16),

          // Controls and Progress
          Row(
            children: [
              // Play/Pause button
              Container(
                decoration: BoxDecoration(
                  color: Colors.blue,
                  borderRadius: BorderRadius.circular(24),
                ),
                child: IconButton(
                  onPressed: _playPause,
                  icon: Icon(
                    _isPlaying ? Icons.pause : Icons.play_arrow,
                    color: Colors.white,
                  ),
                  iconSize: 24,
                ),
              ),

              const SizedBox(width: 16),

              // Progress and duration info
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    // Progress bar
                    GestureDetector(
                      onTapDown: (details) {
                        if (_duration.inMilliseconds > 0) {
                          // final RenderBox box =
                          //     context.findRenderObject() as RenderBox;
                          // final localPosition = box.globalToLocal(
                          //   details.globalPosition,
                          // );
                          // final progress =
                          //     (localPosition.dx - 16) /
                          //     (box.size.width -
                          //         96); // Account for padding and button
                          // // final seekPosition = Duration(
                          //   milliseconds:
                          //       (_duration.inMilliseconds *
                          //               progress.clamp(0.0, 1.0))
                          //           .round(),
                          // );
                          // // _seek(seekPosition);
                        }
                      },
                      child: Container(
                        height: 32,
                        alignment: Alignment.center,
                        child: LinearProgressIndicator(
                          value:
                              _duration.inMilliseconds > 0
                                  ? _position.inMilliseconds /
                                      _duration.inMilliseconds
                                  : 0.0,
                          backgroundColor:
                              isDark
                                  ? Colors.grey.shade700
                                  : Colors.grey.shade300,
                          valueColor: const AlwaysStoppedAnimation<Color>(
                            Colors.blue,
                          ),
                          minHeight: 4,
                        ),
                      ),
                    ),

                    const SizedBox(height: 8),

                    // Time display
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Text(
                          _formatDuration(_position),
                          style: TextStyle(
                            fontSize: 12,
                            color:
                                isDark
                                    ? Colors.grey.shade400
                                    : Colors.grey.shade600,
                            fontWeight: FontWeight.w500,
                          ),
                        ),
                        Text(
                          _formatDuration(_duration),
                          style: TextStyle(
                            fontSize: 12,
                            color:
                                isDark
                                    ? Colors.grey.shade400
                                    : Colors.grey.shade600,
                            fontWeight: FontWeight.w500,
                          ),
                        ),
                      ],
                    ),
                  ],
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }

  @override
  void dispose() {
    WidgetsBinding.instance.removeObserver(this);

    // Cancel all subscriptions
    _durationSubscription?.cancel();
    _positionSubscription?.cancel();
    _playingSubscription?.cancel();
    // _volumeSubscription?.cancel();
    _positionUpdateTimer?.cancel();

    // Dispose player
    _player?.dispose();

    super.dispose();
  }
}
