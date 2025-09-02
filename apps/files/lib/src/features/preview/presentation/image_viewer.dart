import 'dart:io';
import 'package:flutter/material.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_app_bar.dart';
import 'package:photo_view/photo_view.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:path/path.dart' as p;

/// A StatefulWidget that displays image files with zoom support.
/// Supports both raster formats (e.g. PNG, JPG) and vector (SVG).
class ImageViewerPage extends StatefulWidget {
  final String imagePath;

  const ImageViewerPage({super.key, required this.imagePath});

  @override
  State<ImageViewerPage> createState() => _ImageViewerPageState();
}

class _ImageViewerPageState extends State<ImageViewerPage> {
  late bool isSvg;

  @override
  void initState() {
    super.initState();
    // Check if the file is an SVG based on its extension
    isSvg = widget.imagePath.toLowerCase().endsWith('.svg');
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: CustomAppBar(
        title: p.basename(widget.imagePath),
        leftIcon: const Icon(Icons.arrow_back),
        leftIconOnTap: () => Navigator.pop(context), // Back navigation
      ),
      body: isSvg ? _buildSvgViewer() : _buildRasterViewer(),
    );
  }

  /// Builds the viewer for SVG (vector) images using flutter_svg and PhotoView
  Widget _buildSvgViewer() {
    return Center(
      child: PhotoView.customChild(
        backgroundDecoration: const BoxDecoration(color: Colors.black),
        minScale: PhotoViewComputedScale.contained,
        maxScale: PhotoViewComputedScale.covered * 2.0,
        child: SvgPicture.file(
          File(widget.imagePath),
          placeholderBuilder: (context) =>
              const Center(child: CircularProgressIndicator()),
          fit: BoxFit.contain,
        ),
      ),
    );
  }

  /// Builds the viewer for raster images (JPG, PNG) using FileImage
  Widget _buildRasterViewer() {
    return Center(
      child: PhotoView(
        imageProvider: FileImage(File(widget.imagePath)),
        backgroundDecoration: const BoxDecoration(color: Colors.black),
        minScale: PhotoViewComputedScale.contained,
        maxScale: PhotoViewComputedScale.covered * 2.0,
      ),
    );
  }
}
