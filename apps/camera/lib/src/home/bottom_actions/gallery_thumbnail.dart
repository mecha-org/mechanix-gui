import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_state.dart';

class GalleryThumbnail extends StatelessWidget {
  const GalleryThumbnail({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocSelector<CameraBloc, CameraState, String?>(
      selector:
          (state) => state.mediaPath.isNotEmpty ? state.mediaPath.last : null,
      builder: (context, mediaPath) {
        return Container(
          width: 44,
          height: 44,
          decoration: BoxDecoration(borderRadius: BorderRadius.circular(8)),
          child: ClipRRect(
            borderRadius: BorderRadius.circular(6),
            child: _buildContent(mediaPath),
          ),
        );
      },
    );
  }

  Widget _buildContent(String? mediaPath) {
    if (mediaPath == null || mediaPath.isEmpty) {
      return _buildEmpty();
    }

    final file = File(mediaPath);

    if (!file.existsSync()) {
      return _buildEmpty();
    }

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

  Widget _buildEmpty() {
    return Container(color: Colors.transparent);
  }

  Widget _buildErrorPlaceholder() {
    return Container(
      color: Colors.grey[800],
      child: const Icon(Icons.broken_image, color: Colors.white54, size: 24),
    );
  }
}
