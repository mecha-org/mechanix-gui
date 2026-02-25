import 'dart:io';

import 'package:ellipsized_text/ellipsized_text.dart';
import 'package:flutter/material.dart';
import 'package:mechanix_camera/src/media/widgets/image_editor.dart';
import 'package:mechanix_camera/src/media/widgets/image_properties_menu.dart';
import 'package:mechanix_camera/utils/icons/icon.dart';
import 'package:path/path.dart' as p;
import 'package:photo_view/photo_view.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottom_bar/mechanix_bottom_bar_theme.dart';

class ImageView extends StatefulWidget {
  final String imagePath;

  const ImageView({super.key, required this.imagePath});

  @override
  State<ImageView> createState() => ImageViewState();
}

class ImageViewState extends State<ImageView> {
  bool _isEditing = false;

  void _onEditorSaved() {
    setState(() {
      _isEditing = false;
    });
  }

  void _exitEditing() {
    setState(() => _isEditing = false);
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: PreferredSize(
        preferredSize: const Size.fromHeight(60),
        child: Padding(
          padding: const EdgeInsets.only(
            top: 6,
            left: 16,
            right: 16,
            bottom: 12,
          ),
          child: AppBar(
            automaticallyImplyLeading: false,
            scrolledUnderElevation: 0,
            backgroundColor: Colors.transparent,
            elevation: 0,
            title: EllipsizedText(
              type: EllipsisType.middle,
              p.basename(widget.imagePath),
              style: TextStyle(
                color: context.colorScheme.onSurface,
                fontSize: 22,
                fontWeight: FontWeight.w600,
              ),
            ),
          ),
        ),
      ),
      body:
          _isEditing
              ? ImageEditorPage(
                key: const ValueKey('editor'),
                imagePath: widget.imagePath,
                onClose: _exitEditing,
                onSaved: _onEditorSaved,
              )
              : ImageViewer(imagePath: widget.imagePath),

      bottomNavigationBar: _buildBottomBar(context),
    );
  }

  Widget _buildBottomBar(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        MechanixBottomBar(
          centerWidgetSpacing: 16,
          theme: MechanixBottomBarThemeData(
            decoration: BoxDecoration(
              color: context.colorScheme.secondaryContainer,
              borderRadius:
                  _isEditing
                      ? const BorderRadius.only(
                        topLeft: Radius.circular(0),
                        topRight: Radius.circular(0),
                      )
                      : null,
            ),
          ),
          leadingWidget: [
            BottomBarButton.widget(
              widget: Padding(
                padding: const EdgeInsets.only(left: 8),
                child: IconButton(
                  onPressed: () {
                    if (_isEditing) {
                      _exitEditing();
                    } else {
                      Navigator.pop(context);
                    }
                  },
                  icon: IconWidget(
                    iconHeight: 28,
                    iconWidth: 28,
                    iconPath: CameraIcons.backIcon,
                  ),
                ),
              ),
            ),
          ],
          centerWidget: [
            // Copy
            BottomBarButton.widget(
              widget: IconButton(
                onPressed: () {
                  // TODO: handle copy
                },
                icon: IconWidget(
                  iconHeight: 28,
                  iconWidth: 28,
                  iconPath: CameraIcons.copyIcon,
                ),
              ),
            ),

            // Edit / Crop
            BottomBarButton.widget(
              widget: IconButton(
                onPressed: () {
                  setState(() {
                    _isEditing = !_isEditing;
                  });
                },
                icon: IconWidget(
                  iconHeight: 28,
                  iconWidth: 28,
                  iconPath: CameraIcons.cropIcon,
                  isActive: _isEditing,
                ),
              ),
            ),

            // Move
            BottomBarButton.widget(
              widget: IconButton(
                onPressed: () {
                  // TODO: handle move
                },
                icon: IconWidget(
                  iconHeight: 28,
                  iconWidth: 28,
                  iconPath: CameraIcons.folderIcon,
                ),
              ),
            ),

            // Share (disabled)
            BottomBarButton.widget(
              widget: IconButton(
                onPressed: null,
                icon: IconWidget(
                  iconHeight: 28,
                  iconWidth: 28,
                  iconPath: CameraIcons.shareIcon,
                  iconColor: context.colorScheme.outline,
                ),
              ),
            ),
          ],
          anchorWidget: [
            BottomBarButton.widget(
              widget: ImagePropertiesMenu(
                imagePath: widget.imagePath,
              ).padRight(12),
            ),
          ],
        ),
      ],
    );
  }
}

class ImageViewer extends StatelessWidget {
  final String imagePath;

  const ImageViewer({super.key, required this.imagePath});

  @override
  Widget build(BuildContext context) {
    return Center(
      child: PhotoView(
        imageProvider: FileImage(File(imagePath)),
        backgroundDecoration: BoxDecoration(color: context.colorScheme.surface),
        minScale: PhotoViewComputedScale.contained,
        maxScale: PhotoViewComputedScale.covered * 2,
      ),
    );
  }
}
