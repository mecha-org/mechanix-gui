import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_camera/src/app_routes.dart';
import 'package:mechanix_camera/src/bloc/camera_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_state.dart';

class GalleryThumbnail extends StatefulWidget {
  const GalleryThumbnail({super.key});

  @override
  State<GalleryThumbnail> createState() => _GalleryThumbnailState();
}

class _GalleryThumbnailState extends State<GalleryThumbnail>
    with SingleTickerProviderStateMixin {
  late final AnimationController _controller;
  late final Animation<double> _scale;
  late final Animation<double> _opacity;

  String? _previousPath;

  @override
  void initState() {
    super.initState();

    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 300),
    );

    _scale = TweenSequence<double>([
      TweenSequenceItem(
        tween: Tween(
          begin: 0.4,
          end: 1.15,
        ).chain(CurveTween(curve: Curves.easeOut)),
        weight: 60,
      ),
      TweenSequenceItem(
        tween: Tween(
          begin: 1.15,
          end: 0.95,
        ).chain(CurveTween(curve: Curves.easeInOut)),
        weight: 20,
      ),
      TweenSequenceItem(
        tween: Tween(
          begin: 0.95,
          end: 1.0,
        ).chain(CurveTween(curve: Curves.easeInOut)),
        weight: 20,
      ),
    ]).animate(_controller);

    _opacity = Tween<double>(begin: 0.0, end: 1.0).animate(
      CurvedAnimation(
        parent: _controller,
        curve: const Interval(0.0, 0.4, curve: Curves.easeIn),
      ),
    );
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  void _playAnimation() {
    _controller.forward(from: 0.0);
  }

  @override
  Widget build(BuildContext context) {
    return BlocSelector<CameraBloc, CameraState, String?>(
      selector:
          (state) => state.mediaPath.isNotEmpty ? state.mediaPath.last : null,
      builder: (context, mediaPath) {
        if (mediaPath != null && mediaPath != _previousPath) {
          _previousPath = mediaPath;
          WidgetsBinding.instance.addPostFrameCallback((_) => _playAnimation());
        }

        return AnimatedBuilder(
          animation: _controller,
          builder: (context, child) {
            final isAnimating =
                _controller.isAnimating || _controller.value > 0;
            return Transform.scale(
              scale: isAnimating ? _scale.value : 1.0,
              child: Opacity(
                opacity: isAnimating ? _opacity.value.clamp(0.0, 1.0) : 1.0,
                child: child,
              ),
            );
          },
          child: GestureDetector(
            onTap: () {
              if (mediaPath == null || mediaPath.isEmpty) return;

              Navigator.pushNamed(
                context,
                AppRoutes.media,
                arguments: mediaPath,
              );
            },
            child: Container(
              width: 44,
              height: 44,
              decoration: BoxDecoration(borderRadius: BorderRadius.circular(8)),
              child: ClipRRect(
                borderRadius: BorderRadius.circular(6),
                child: _buildContent(mediaPath),
              ),
            ),
          ),
        );
      },
    );
  }

  Widget _buildContent(String? mediaPath) {
    if (mediaPath == null || mediaPath.isEmpty) return _buildEmpty();

    final file = File(mediaPath);
    if (!file.existsSync()) return _buildEmpty();

    final ext = mediaPath.split('.').last.toLowerCase();

    if (ext == 'jpg') return _buildImageThumbnail(file);
    if (ext == 'mp4') return _buildVideoPlaceholder();

    return _buildEmpty();
  }

  Widget _buildImageThumbnail(File file) {
    return Image.file(
      file,
      fit: BoxFit.cover,
      errorBuilder: (context, error, stackTrace) => _buildErrorPlaceholder(),
    );
  }

  Widget _buildVideoPlaceholder() {
    return Container(
      color: Colors.black87,
      child: const Stack(
        alignment: Alignment.center,
        children: [
          Icon(Icons.videocam, color: Colors.white, size: 20),
          Positioned(
            bottom: 3,
            right: 3,
            child: Icon(Icons.play_circle_fill, color: Colors.white, size: 12),
          ),
        ],
      ),
    );
  }

  Widget _buildEmpty() => Container(color: Colors.transparent);

  Widget _buildErrorPlaceholder() {
    return Container(
      color: Colors.grey[800],
      child: const Icon(Icons.broken_image, color: Colors.white54, size: 24),
    );
  }
}
