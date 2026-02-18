import 'package:flutter/material.dart';
import 'package:mechanix_camera/utils/icons/icon.dart';

/// Displays a thumbnail preview of the last captured photo/video
class GalleryThumbnail extends StatelessWidget {
  const GalleryThumbnail({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 44,
      height: 44,
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(8),
      ),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(6),
        child: Image.asset(
          CameraIcons.demoImage,
          fit: BoxFit.cover,
          errorBuilder: (context, error, stackTrace) {
            return _buildErrorPlaceholder();
          },
        ),
      ),
    );
  }

  Widget _buildErrorPlaceholder() {
    return Container(
      color: Colors.orange[300],
      child: const Icon(
        Icons.image,
        color: Colors.white,
        size: 24,
      ),
    );
  }
}