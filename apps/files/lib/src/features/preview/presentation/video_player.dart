import 'dart:async';
import 'package:flutter/material.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/commons/styles/file_theme_extenstions.dart';
import 'package:mechanix_files/src/features/files/presentation/files.dart';
import 'package:mechanix_files/src/services/media_kit_manager.dart';
import 'package:media_kit/media_kit.dart';
import 'package:media_kit_video/media_kit_video.dart';
import 'package:path/path.dart' as p;
import 'package:widgets/constants.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottomBar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottomBar/mechanix_bottom_bar_theme.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';

class VideoPlayer extends StatefulWidget {
  final BuildContext rootContext;
  String filePath;

  VideoPlayer({
    super.key,
    required this.filePath,
    required this.rootContext,
  });

  @override
  State<VideoPlayer> createState() => _VideoPlayerState();
}

class _VideoPlayerState extends State<VideoPlayer> {
  late final Player player;
  late final VideoController videoController;

  bool _playerReady = false;
  Duration _position = Duration.zero;
  Duration _duration = Duration.zero;
  bool _isPlaying = false;

  late final StreamSubscription<bool> _playingSub;
  late final StreamSubscription<Duration> _positionSub;
  late final StreamSubscription<Duration?> _durationSub;

  bool _showTimeBubble = false;
  Timer? _bubbleTimer;
  double _lastVolume = 1.0;

  bool isMenuOpen = false;
  String title = "";

  @override
  void initState() {
    super.initState();
    _initVideoPlayer();
  }

  Future<void> _initVideoPlayer() async {
    await MediaKitManager.init();
    player = Player();
    videoController = VideoController(player);

    await player.open(Media(widget.filePath));

    _lastVolume = player.state.volume;

    // Streams
    _playingSub = player.stream.playing.listen((v) {
      if (mounted) setState(() => _isPlaying = v);
    });

    _positionSub = player.stream.position.listen((pos) {
      if (mounted) setState(() => _position = pos);
    });

    _durationSub = player.stream.duration.listen((dur) {
      if (mounted) setState(() => _duration = dur ?? Duration.zero);
    });

    if (mounted) {
      setState(() => _playerReady = true);
      title = p.basename(widget.filePath);
    }
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
    return Scaffold(
      appBar: _buildAppBar(context),
      body: _playerReady
          ? Center(child: _buildVideo())
          : const Center(child: CircularProgressIndicator()),
      bottomNavigationBar:
          _playerReady ? _buildBottomBar(context) : const SizedBox(),
    );
  }

  PreferredSizeWidget _buildAppBar(BuildContext context) {
    return PreferredSize(
      preferredSize: const Size.fromHeight(60),
      child: Padding(
        padding: const EdgeInsets.only(top: 18, left: 16, right: 16),
        child: AppBar(
          automaticallyImplyLeading: false,
          scrolledUnderElevation: 0,
          title: Text(
            title,
            overflow: TextOverflow.ellipsis,
            style: TextStyle(
              color: const Color(0xFFD2D2D2),
              fontSize: 20,
              fontWeight: FontWeight.w600,
              fontFamily:
                  Theme.of(context).extension<FilesTheme>()!.defaultFontFamily,
            ),
          ),
          backgroundColor: Colors.transparent,
          elevation: 0,
        ),
      ),
    );
  }

  Widget _buildVideo() {
    return AspectRatio(
      aspectRatio: 16 / 9,
      child: ClipRRect(
        borderRadius: BorderRadius.circular(12),
        child: Video(controller: videoController, controls: NoVideoControls),
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
                    // Play / Pause
                    IconButton(
                      padding: EdgeInsets.zero,
                      icon: Icon(
                        _isPlaying ? Icons.pause : Icons.play_arrow,
                        color: Colors.white,
                        size: 28,
                      ),
                      onPressed: () =>
                          _isPlaying ? player.pause() : player.play(),
                    ),

                    const SizedBox(width: 6),

                    // Seek slider
                    Expanded(
                      child: SliderTheme(
                        data: SliderTheme.of(context).copyWith(
                          trackHeight: 6,
                          thumbShape: const RoundSliderThumbShape(
                              enabledThumbRadius: 8),
                          overlayShape:
                              const RoundSliderOverlayShape(overlayRadius: 0),
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
                          thumbColor: Colors.white,
                          onChanged: (v) {
                            player.seek(Duration(milliseconds: v.toInt()));
                            _showBubble();
                          },
                        ),
                      ),
                    ),

                    const SizedBox(width: 6),

                    // Volume mute/unmute
                    StreamBuilder<double>(
                      stream: player.stream.volume,
                      builder: (_, snap) {
                        final volume = snap.data ?? _lastVolume;
                        final muted = volume == 0;

                        if (!muted) _lastVolume = volume;

                        return IconButton(
                          padding: EdgeInsets.zero,
                          icon: Icon(
                            muted ? Icons.volume_off : Icons.volume_up,
                            color: Colors.white,
                            size: 24,
                          ),
                          onPressed: () =>
                              player.setVolume(muted ? _lastVolume : 0.0),
                        );
                      },
                    )
                  ],
                ),
              ),
              if (_showTimeBubble)
                Positioned(
                  top: -42,
                  left: 0,
                  right: 0,
                  child: Center(
                    child: _buildTimeBubble(),
                  ),
                ),
            ],
          ),

          // Bottom menu bar same as audio
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
        style: const TextStyle(color: Colors.white, fontSize: 16),
      ),
    );
  }

  String _formatHMS(Duration d) {
    final h = d.inHours;
    final m = d.inMinutes.remainder(60).toString().padLeft(2, '0');
    final s = d.inSeconds.remainder(60).toString().padLeft(2, '0');

    return h > 0 ? "$h:$m:$s" : "$m:$s";
  }
}
