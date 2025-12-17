import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/commons/styles/file_theme_extenstions.dart';
import 'package:mechanix_files/src/features/files/presentation/files.dart';
import 'package:mechanix_files/src/features/preview/presentation/image_editor.dart';
import 'package:path/path.dart' as p;
import 'package:photo_view/photo_view.dart';
import 'package:widgets/constants.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottomBar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottomBar/mechanix_bottom_bar_theme.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';

/// A StatefulWidget that displays image files with zoom support.
/// Supports both raster formats (e.g. PNG, JPG) and vector (SVG).
class ImageViewerPage extends StatefulWidget {
  final BuildContext rootContext;
  String filePath;

  ImageViewerPage(
      {super.key, required this.rootContext, required this.filePath});

  @override
  State<ImageViewerPage> createState() => _ImageViewerPageState();
}

class _ImageViewerPageState extends State<ImageViewerPage> {
  bool get isSvg => widget.filePath.toLowerCase().endsWith('.svg');
  bool isMenuOpen = false;
  String title = '';
  bool _isEditing = false;

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
      body: isSvg
          ? _SvgViewer(imagePath: widget.filePath)
          : (_isEditing
              ? ImageEditorPage(
                  rootContext: widget.rootContext,
                  imagePath: widget.filePath,
                  onClose: () {
                    setState(() {
                      _isEditing = false;
                    });
                  },
                )
              : _RasterViewer(
                  imagePath: widget.filePath,
                )),
      bottomNavigationBar: _buildBottomBar(context),
    );
  }

  Widget _buildBottomBar(BuildContext context) {
    final state =
        widget.rootContext.findAncestorStateOfType<FileExplorerPageState>();

    return MechanixBottomBar(
      leadingWidget: [
        BottomBarButton(
          iconTheme: const MechanixBottomBarIconThemeData(
              padding: EdgeInsets.only(left: 12), iconSize: Size(28, 28)),
          iconPath: Images.back,
          onPressed: () => Navigator.pop(context),
        ),
      ],
      centerWidgetSpacing: 30,
      centerWidget: [
        BottomBarButton(
          iconTheme:
              const MechanixBottomBarIconThemeData(iconSize: Size(28, 28)),
          iconPath: Images.copy,
          onPressed: () {
            state?.selectedPaths = {widget.filePath};
            state?.handleCopy();
          },
        ),
        BottomBarButton(
          iconTheme: const MechanixBottomBarIconThemeData(),
          isDisabled: isSvg,
          isSelected: _isEditing,
          iconWidget: IconWidget(
            iconPath: Images.crop,
            iconColor: isSvg
                ? Colors.grey
                : _isEditing
                    ? Theme.of(context).extension<FilesTheme>()!.primaryColor
                    : Colors.white70,
            iconHeight: 28.0,
            iconWidth: 28.0,
          ),
          onPressed: isSvg
              ? null
              : () {
                  setState(() {
                    _isEditing = !_isEditing;
                  });
                },
        ),
        BottomBarButton(
          iconPath: Images.move,
          iconTheme:
              const MechanixBottomBarIconThemeData(iconSize: Size(28, 28)),
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
            iconHeight: 28.0,
            iconWidth: 28.0,
          ),
          isDisabled: true,
          onPressed: () {}, // TODO : share functionality
        ),
      ],
      anchorWidget: [
        BottomBarButton.widget(
          widget: buildActionsMenu(context),
        ),
      ],
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
          iconHeight: 28,
          iconWidth: 28,
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
}

class _RasterViewer extends StatelessWidget {
  final String imagePath;

  const _RasterViewer({required this.imagePath});

  @override
  Widget build(BuildContext context) {
    return Center(
      child: PhotoView(
        imageProvider: FileImage(File(imagePath)),
        backgroundDecoration: const BoxDecoration(color: Colors.black),
        minScale: PhotoViewComputedScale.contained,
        maxScale: PhotoViewComputedScale.covered * 2,
      ),
    );
  }
}

class _SvgViewer extends StatelessWidget {
  final String imagePath;

  const _SvgViewer({required this.imagePath});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: PhotoView.customChild(
          backgroundDecoration: const BoxDecoration(color: Colors.black),
          minScale: PhotoViewComputedScale.contained,
          maxScale: PhotoViewComputedScale.covered * 2,
          child: SvgPicture.file(
            File(imagePath),
            fit: BoxFit.contain,
            placeholderBuilder: (_) => const CircularProgressIndicator(),
          ),
        ),
      ),
    );
  }
}
