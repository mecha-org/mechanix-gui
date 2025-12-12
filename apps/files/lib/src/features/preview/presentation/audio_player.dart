import 'dart:async';
import 'package:flutter/material.dart';
import 'package:mechanix_files/src/commons/styles/file_theme_extenstions.dart';
import 'package:mechanix_files/src/features/files/presentation/files.dart';
import 'package:mechanix_files/src/services/media_kit_manager.dart';
import 'package:media_kit/media_kit.dart';
import 'package:path/path.dart' as p;
import 'package:widgets/constants.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottomBar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottomBar/mechanix_bottom_bar_theme.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';

import '../../../commons/constants.dart';

class AudioPlayerOverlay extends StatefulWidget {
  final BuildContext rootContext;
  String filePath;
  AudioPlayerOverlay(
      {super.key, required this.filePath, required this.rootContext});

  @override
  State<AudioPlayerOverlay> createState() => _AudioPlayerOverlayState();
}

class _AudioPlayerOverlayState extends State<AudioPlayerOverlay> {
  bool _playerReady = false;
  late final Player player;
  Duration _position = Duration.zero;
  Duration _duration = Duration.zero;
  bool _isPlaying = false;
  late final StreamSubscription<bool> _playingSub;
  late final StreamSubscription<Duration> _positionSub;
  late final StreamSubscription<Duration> _durationSub;
  bool _showTimeBubble = false;
  Timer? _bubbleTimer;
  double _lastVolume = 1.0; // default fallback

  bool isMenuOpen = false;
  String title = '';

  @override
  void initState() {
    super.initState();
    _initializePlayer();
  }

  Future<void> _initializePlayer() async {
    await MediaKitManager.init();
    player = Player();
    await player.open(Media(widget.filePath));

    // Get actual volume from the player
    _lastVolume = player.state.volume;

    _playingSub = player.stream.playing.listen((playing) {
      if (mounted) setState(() => _isPlaying = playing);
    });

    _positionSub = player.stream.position.listen((pos) {
      if (mounted) setState(() => _position = pos);
    });

    _durationSub = player.stream.duration.listen((dur) {
      if (mounted) setState(() => _duration = dur);
    });

    if (mounted) setState(() => _playerReady = true);
  }

  void _showBubble() {
    setState(() => _showTimeBubble = true);
    _bubbleTimer?.cancel();
    _bubbleTimer = Timer(const Duration(seconds: 1), () {
      if (mounted) setState(() => _showTimeBubble = false);
    });
  }

  @override
  void dispose() {
    _playingSub.cancel();
    _positionSub.cancel();
    _durationSub.cancel();
    player.dispose();
    _bubbleTimer?.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    title = p.basename(widget.filePath);
    return Scaffold(
      appBar: PreferredSize(
        preferredSize: const Size.fromHeight(60),
        child: Padding(
          padding: const EdgeInsets.only(top: 18, left: 16, right: 16),
          child: AppBar(
            automaticallyImplyLeading: false,
            scrolledUnderElevation: 0,
            title: Text(
              title,
              style: TextStyle(
                color: const Color(0xFFD2D2D2),
                fontSize: 20,
                fontWeight: FontWeight.w600,
                fontFamily: Theme.of(context)
                    .extension<FilesTheme>()!
                    .defaultFontFamily,
              ),
              overflow: TextOverflow.ellipsis,
            ),
            backgroundColor: Colors.transparent,
            elevation: 0,
          ),
        ),
      ),
      body: Center(
        child: Container(
          width: 200,
          height: 200,
          decoration: BoxDecoration(
            color: Colors.grey.shade900,
            borderRadius: BorderRadius.circular(16),
          ),
          padding: const EdgeInsets.all(60),
          child: Image.asset(
            Images.musicNote,
          ),
        ),
      ),
      bottomNavigationBar: _playerReady
          ? _buildBottomBar(context)
          : const Padding(
              padding: EdgeInsets.all(20),
              child: CircularProgressIndicator(color: Colors.white),
            ),
    );
  }

  Widget _buildBottomBar(BuildContext context) {
    final state =
        widget.rootContext.findAncestorStateOfType<FileExplorerPageState>();

    return Container(
      color: Colors.grey.shade900,
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Stack(
            clipBehavior: Clip.none,
            children: [
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 12),
                child: Row(
                  children: [
                    IconButton(
                      padding: EdgeInsets.zero,
                      icon: Image.asset(
                        _isPlaying ? Images.pause : Images.play,
                        width: 24,
                        height: 24,
                      ),
                      onPressed: () {
                        _isPlaying ? player.pause() : player.play();
                      },
                    ),
                    const SizedBox(width: 6),
                    Expanded(
                        child: SliderTheme(
                      data: SliderTheme.of(context).copyWith(
                        trackHeight: 6,
                        thumbShape: const RoundSliderThumbShape(
                          enabledThumbRadius: 8,
                        ),
                        overlayShape: const RoundSliderOverlayShape(
                          overlayRadius: 0,
                        ),
                      ),
                      child: Slider(
                        min: 0,
                        max: _duration.inMilliseconds
                            .toDouble()
                            .clamp(1, double.infinity),
                        value: _position.inMilliseconds
                            .toDouble()
                            .clamp(0, _duration.inMilliseconds.toDouble()),
                        activeColor: Theme.of(context)
                            .extension<FilesTheme>()!
                            .primaryColor,
                        inactiveColor: const Color(0xFF464646),
                        thumbColor: const Color(0xFFD2D2D2),
                        onChanged: (v) {
                          player.seek(Duration(milliseconds: v.toInt()));
                          _showBubble();
                        },
                      ),
                    )),
                    const SizedBox(width: 6),
                    StreamBuilder<double>(
                      stream: player.stream.volume,
                      builder: (_, snapshot) {
                        final volume = snapshot.data ?? _lastVolume;
                        final isMuted = volume == 0.0;

                        if (!isMuted) {
                          _lastVolume = volume; // store last non-zero volume
                        }

                        return IconButton(
                          onPressed: () {
                            if (isMuted) {
                              player.setVolume(_lastVolume); // restore
                            } else {
                              player.setVolume(0.0); // mute
                            }
                          },
                          icon: Image.asset(
                            isMuted ? Images.mute : Images.volume,
                            width: 24,
                            height: 24,
                          ),
                        );
                      },
                    )
                  ],
                ),
              ),
              if (_showTimeBubble)
                Positioned(
                  top: -42, // float upward
                  left: 0,
                  right: 0,
                  child: Center(
                    child: _buildTimeBubble(),
                  ),
                ),
            ],
          ),
          MechanixBottomBar(
            leadingWidget: [
              BottomBarButton(
                iconTheme: const MechanixBottomBarIconThemeData(
                    padding: EdgeInsets.only(left: 12)),
                iconPath: Images.back,
                onPressed: () => Navigator.pop(context),
              ),
            ],
            centerWidgetSpacing: 30,
            centerWidget: [
              BottomBarButton(
                iconPath: Images.copy,
                onPressed: () {
                  state?.selectedPaths = {widget.filePath};
                  state?.handleCopy();
                },
              ),
              BottomBarButton(
                iconPath: Images.move,
                onPressed: () {
                  Navigator.pop(context);
                  state?.selectedPaths = {widget.filePath};
                  state?.handleMove();
                },
              ),
              BottomBarButton(
                iconWidget: IconWidget(
                  iconPath: Images.share,
                  iconColor: Colors.grey.shade600,
                ),
                onPressed: () {},
                isDisabled: true, //TODO : add share functionality
              ),
            ],
            anchorWidget: [
              BottomBarButton.widget(widget: buildActionsMenu(context)),
            ],
          ),
        ],
      ),
    );
  }

  Widget buildActionsMenu(BuildContext context) {
    final offset = const Offset(-8, -14);
    final state =
        widget.rootContext.findAncestorStateOfType<FileExplorerPageState>();

    return MechanixMenu(
      offset: offset,
      dropdownPosition: DropdownPosition.topRight,
      animationDuration: const Duration(milliseconds: 300),
      buttonIcon: IconWidget(
          iconPath: Images.dots,
          iconColor: isMenuOpen
              ? Theme.of(context).extension<FilesTheme>()!.primaryColor
              : Colors.white70),
      openMenu: () {
        setState(() => isMenuOpen = true);
      },
      closeMenu: () {
        setState(() => isMenuOpen = false);
      },
      items: [
        MechanixMenuItemsType(
          leading: Image.asset(
            Images.rename,
            color: Colors.white70,
            height: mechanixIconSize,
          ),
          title: 'Rename',
          onTap: () async {
            final oldPath = widget.filePath;

            // Wait for rename result
            final newPath = await state?.showRenameSheet(
              initialName: p.basename(oldPath),
            );

            // If user canceled : do nothing
            if (newPath == null) return;

            // If rename succeeded : update title + filepath
            setState(() {
              title = p.basename(newPath);
            });

            // Also update widget.filePath for correct behavior
            widget.filePath = newPath;
          },
        ),
        MechanixMenuItemsType(
          title: "Properties",
          leading: Image.asset(
            Images.info,
            color: Colors.white70,
            height: mechanixIconSize,
          ),
          onTap: () {
            state?.showDetailsDialog(widget.rootContext, widget.filePath);
          },
        ),
        MechanixMenuItemsType(
          title: "Delete",
          leading: Image.asset(
            Images.delete,
            color: Colors.white70,
            height: mechanixIconSize,
          ),
          onTap: () {
            Navigator.pop(context);
            state?.confirmDelete(widget.rootContext, {widget.filePath});
          },
        ),
      ],
    ).padRight(8);
  }

  Widget _buildTimeBubble() {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 6),
      decoration: BoxDecoration(
        color: const Color.fromRGBO(70, 70, 70, 0.6),
        borderRadius: BorderRadius.circular(6),
      ),
      child: Text(
        "${_formatHMS(_position)} / ${_formatHMS(_duration)}",
        style: TextStyle(
          fontSize: 16,
          color: const Color(0xFFFFFFFF),
          fontWeight: FontWeight.w500,
          fontFamily:
              Theme.of(context).extension<FilesTheme>()!.defaultFontFamily,
        ),
      ),
    );
  }

  String _formatHMS(Duration d) {
    final hours = d.inHours;
    final minutes = d.inMinutes.remainder(60).toString().padLeft(2, '0');
    final seconds = d.inSeconds.remainder(60).toString().padLeft(2, '0');

    if (hours > 0) {
      return "${hours.toString().padLeft(2, '0')}:$minutes:$seconds";
    } else {
      return "$minutes:$seconds";
    }
  }
}
