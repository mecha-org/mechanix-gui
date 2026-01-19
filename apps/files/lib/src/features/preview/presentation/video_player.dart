import 'dart:async';
import 'dart:io' show FileSystemEntity, File;
import 'package:flutter/material.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/commons/customWidgets/middle_ellipsis_text.dart';
import 'package:mechanix_files/src/commons/customWidgets/pressable_icon.dart';
import 'package:mechanix_files/src/controllers/file_manager_controller.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:mechanix_files/src/features/files/presentation/files.dart';
import 'package:mechanix_files/src/services/media_kit_manager.dart';
import 'package:media_kit/media_kit.dart';
import 'package:media_kit_video/media_kit_video.dart';
import 'package:path/path.dart' as p;
import 'package:widgets/constants.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottom_bar/mechanix_bottom_bar_theme.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';

class VideoPlayer extends StatefulWidget {
  final BuildContext rootContext;
  String filePath;
  FileExplorerPageState? state;

  VideoPlayer({
    super.key,
    required this.filePath,
    required this.rootContext,
    this.state,
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

  double _lastVolume = 1.0;

  bool isMenuOpen = false;

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
    }
  }

  @override
  void dispose() {
    _playingSub.cancel();
    _positionSub.cancel();
    _durationSub.cancel();
    player.dispose();
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
    final explorerState = widget.state;

    final controller = explorerState?.controller;

    return PreferredSize(
      preferredSize: const Size.fromHeight(60),
      child: Padding(
        padding: const EdgeInsets.only(top: 6, left: 16, right: 16, bottom: 12),
        child: AppBar(
          automaticallyImplyLeading: false,
          scrolledUnderElevation: 0,
          title: _buildTitle(controller),
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

  Widget _buildTitle(FileManagerController? controller) {
    if (controller == null) {
      return MiddleEllipsisText(
        p.basename(widget.filePath),
        style: previewTitleStyle(context),
      );
    }

    return ValueListenableBuilder<List<FileSystemEntity>>(
      valueListenable: controller.paginatedEntities,
      builder: (_, __, ___) {
        final title = controller.getDisplayName(File(widget.filePath));
        return MiddleEllipsisText(
          title,
          style: previewTitleStyle(context),
        );
      },
    );
  }

  Widget _buildBottomBar(BuildContext context) {
    final state = widget.state;

    return Container(
      decoration: BoxDecoration(
        color: context.colorScheme.secondary,
        borderRadius: const BorderRadius.only(
            topLeft: Radius.circular(8), topRight: Radius.circular(8)),
      ),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Stack(
            clipBehavior: Clip.none,
            children: [
              Padding(
                padding:
                    const EdgeInsets.symmetric(horizontal: 12, vertical: 12),
                child: Row(
                  children: [
                    // Play / Pause
                    PressableIcon(
                      iconPath: _isPlaying ? Images.pause : Images.play,
                      onTap: () {
                        setState(() {
                          _isPlaying ? player.pause() : player.play();
                          _isPlaying = !_isPlaying;
                        });
                      },
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
                          activeColor: context.colorScheme.primaryContainer,
                          inactiveColor: context.colorScheme.surfaceContainer,
                          thumbColor: context.colorScheme.onSurface,
                          onChanged: (v) {
                            player.seek(Duration(milliseconds: v.toInt()));
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

                        return PressableIcon(
                          iconPath: muted ? Images.mute : Images.volume,
                          onTap: () {
                            setState(() {
                              if (muted) {
                                player.setVolume(_lastVolume); // restore
                              } else {
                                player.setVolume(0.0); // mute
                              }
                            });
                          },
                        );
                      },
                    )
                  ],
                ),
              ),
              Positioned(
                top: -36,
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
            theme: MechanixBottomBarThemeData(
                decoration: BoxDecoration(
              color: context.colorScheme.secondaryContainer,
              borderRadius: null,
            )),
            leadingWidget: [
              BottomBarButton.widget(
                  widget: Padding(
                padding: const EdgeInsets.only(left: 4),
                child: DecoratedPressableIcon(
                  iconPath: Images.back,
                  onTap: () => Navigator.pop(context),
                ),
              )),
            ],
            centerWidgetSpacing: 30,
            centerWidget: [
              BottomBarButton.widget(
                widget: DecoratedPressableIcon(
                  iconPath: Images.copy,
                  onTap: () {
                    state?.selectedPaths = {widget.filePath};
                    state?.handleCopy();
                  },
                ),
              ),
              BottomBarButton.widget(
                widget: DecoratedPressableIcon(
                  iconPath: Images.move,
                  onTap: () {
                    Navigator.pop(context);
                    state?.selectedPaths = {widget.filePath};
                    state?.handleMove();
                  },
                ),
              ),
              const BottomBarButton.widget(
                widget: DecoratedPressableIcon(
                  iconPath: Images.share,
                  isDisabled: true, // TODO: add share functionality
                  onTap: null,
                ),
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
    final state = widget.state;

    return MechanixMenu(
      offset: offset,
      dropdownPosition: DropdownPosition.topRight,
      animationDuration: const Duration(milliseconds: 300),
      buttonIcon: IconWidget(
          iconPath: Images.dots,
          iconHeight: 28,
          iconWidth: 28,
          iconColor: isMenuOpen
              ? context.colorScheme.primaryContainer
              : context.colorScheme.onSurface),
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
            color: context.colorScheme.onSurface,
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

            // Also update widget.filePath for correct behavior
            widget.filePath = newPath;
          },
        ),
        MechanixMenuItemsType(
          title: "Properties",
          leading: Image.asset(
            Images.info,
            color: context.colorScheme.onSurface,
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
            color: context.colorScheme.onSurface,
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
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      decoration: BoxDecoration(
        color: context.colorScheme.surfaceContainer.withOpacity(0.6),
        borderRadius: BorderRadius.circular(6),
      ),
      child: Text(
        "${_formatHMS(_position)} / ${_formatHMS(_duration)}",
        style: TextStyle(
            color: context.colorScheme.onSecondaryFixedVariant, fontSize: 16),
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
