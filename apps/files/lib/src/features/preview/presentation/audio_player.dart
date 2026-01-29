import 'dart:async';
import 'dart:io';
import 'package:flutter/material.dart';
import 'package:mechanix_files/src/commons/customWidgets/middle_ellipsis_text.dart';
import 'package:mechanix_files/src/commons/customWidgets/pressable_icon.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:mechanix_files/src/features/files/presentation/files.dart';
import 'package:mechanix_files/src/services/media_kit_manager.dart';
import 'package:media_kit/media_kit.dart';
import 'package:path/path.dart' as p;
import 'package:widgets/constants.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottom_bar/mechanix_bottom_bar_theme.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';

import '../../../commons/constants.dart';

class AudioPlayerOverlay extends StatefulWidget {
  final BuildContext rootContext;
  String filePath;
  FileExplorerPageState? state;

  AudioPlayerOverlay(
      {super.key,
      required this.filePath,
      required this.rootContext,
      this.state});

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
  double _lastVolume = 1.0; // default fallback

  bool isMenuOpen = false;

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
    final explorerState = widget.state;

    final controller = explorerState?.controller;

    final title = controller != null
        ? controller.getDisplayName(File(widget.filePath))
        : p.basename(widget.filePath);

    return Scaffold(
      appBar: PreferredSize(
        preferredSize: const Size.fromHeight(60),
        child: Padding(
          padding:
              const EdgeInsets.only(top: 6, left: 16, right: 16, bottom: 12),
          child: AppBar(
            automaticallyImplyLeading: false,
            scrolledUnderElevation: 0,
            title: MiddleEllipsisText(title, style: previewTitleStyle(context)),
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
            color: context.colorScheme.secondary,
            borderRadius: BorderRadius.circular(16),
          ),
          padding: const EdgeInsets.all(60),
          child: Image.asset(
            Images.musicNote,
            color: context.colorScheme.onSurfaceVariant,
          ),
        ),
      ),
      bottomNavigationBar: _playerReady
          ? _buildBottomBar(context)
          : Padding(
              padding: const EdgeInsets.all(20),
              child: CircularProgressIndicator(
                  color: context.colorScheme.primaryContainer),
            ),
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
                    PressableIcon(
                      iconPath: _isPlaying ? Images.pause : Images.play,
                      onTap: () {
                        setState(() {
                          _isPlaying ? player.pause() : player.play();
                          _isPlaying = !_isPlaying;
                        });
                      },
                    ),
                    const SizedBox(width: 14),
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
                        activeColor: context.colorScheme.primaryContainer,
                        inactiveColor: context.colorScheme.surfaceContainer,
                        thumbColor: context.colorScheme.onSurface,
                        onChanged: (v) {
                          player.seek(Duration(milliseconds: v.toInt()));
                        },
                      ),
                    )),
                    const SizedBox(width: 14),
                    StreamBuilder<double>(
                      stream: player.stream.volume,
                      builder: (_, snapshot) {
                        final volume = snapshot.data ?? _lastVolume;
                        final isMuted = volume == 0.0;

                        if (!isMuted) {
                          _lastVolume = volume; // store last non-zero volume
                        }

                        return PressableIcon(
                          iconPath: isMuted ? Images.mute : Images.volume,
                          onTap: () {
                            setState(() {
                              if (isMuted) {
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
                top: -36, // float upward
                left: 0,
                right: 0,
                child: Center(
                  child: _buildTimeBubble(),
                ),
              ),
            ],
          ),
          MechanixBottomBar(
            theme: MechanixBottomBarThemeData(
                decoration: BoxDecoration(
              color: context.colorScheme.secondaryContainer,
              borderRadius: null,
            )),
            leadingWidget: [
              BottomBarButton.widget(
                  widget: Padding(
                padding: const EdgeInsets.only(left: 8),
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
          iconWidth: 28,
          iconHeight: 28,
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
          fontSize: 16,
          color: context.colorScheme.onSecondaryFixedVariant,
          fontWeight: FontWeight.w500,
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
