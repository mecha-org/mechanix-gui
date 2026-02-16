import 'dart:io';

import 'package:ellipsized_text/ellipsized_text.dart';
import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/controllers/file_manager_controller.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:mechanix_files/src/features/files/presentation/files.dart';
import 'package:mechanix_files/src/features/preview/presentation/image_editor.dart';
import 'package:path/path.dart' as p;
import 'package:photo_view/photo_view.dart';
import 'package:widgets/constants.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottom_bar/mechanix_bottom_bar_theme.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';

/// A StatefulWidget that displays image files with zoom support.
/// Supports both raster formats (e.g. PNG, JPG) and vector (SVG).
class ImageViewerPage extends StatefulWidget {
  final BuildContext rootContext;
  String filePath;
  FileExplorerPageState? state;

  ImageViewerPage({
    super.key,
    required this.rootContext,
    required this.filePath,
    this.state,
  });

  @override
  State<ImageViewerPage> createState() => _ImageViewerPageState();
}

class _ImageViewerPageState extends State<ImageViewerPage> {
  bool get isSvg => widget.filePath.toLowerCase().endsWith('.svg');
  bool isMenuOpen = false;
  bool _isEditing = false;

  @override
  Widget build(BuildContext context) {
    final explorerState = widget.state;

    final controller = explorerState?.controller;

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
            title: _buildTitle(controller),
            backgroundColor: Colors.transparent,
            elevation: 0,
          ),
        ),
      ),
      body: isSvg
          ? _SvgViewer(imagePath: widget.filePath)
          : (_isEditing
              ? ImageEditorPage(
                  state: widget.state,
                  imagePath: widget.filePath,
                  onClose: () {
                    setState(() {
                      _isEditing = false;
                    });
                  },
                )
              : _RasterViewer(imagePath: widget.filePath)),
      bottomNavigationBar: _buildBottomBar(context),
    );
  }

  Widget _buildTitle(FileManagerController? controller) {
    if (controller == null) {
      return EllipsizedText(
        type: EllipsisType.middle,
        p.basename(widget.filePath),
        style: previewTitleStyle(context),
      );
    }

    return ValueListenableBuilder<List<FileSystemEntity>>(
      valueListenable: controller.paginatedEntities,
      builder: (_, __, ___) {
        final title = controller.getDisplayName(File(widget.filePath));
        return EllipsizedText(
          type: EllipsisType.middle,
          title,
          style: previewTitleStyle(context),
        );
      },
    );
  }

  Widget _buildBottomBar(BuildContext context) {
    final state = widget.state;

    return MechanixBottomBar(
      theme: MechanixBottomBarThemeData(
        decoration: BoxDecoration(
          color: context.colorScheme.secondaryContainer,
          borderRadius: !_isEditing
              ? const BorderRadius.only(
                  topLeft: Radius.circular(8),
                  topRight: Radius.circular(8),
                )
              : null,
        ),
      ),
      leadingWidget: [
        BottomBarButton.widget(
          widget: Padding(
            padding: const EdgeInsets.only(left: 8),
            child: IconButton(
              onPressed: () => Navigator.pop(context),
              icon: const IconWidget(
                iconHeight: 28,
                iconWidth: 28,
                iconPath: Images.back,
              ),
            ),
          ),
        ),
      ],
      centerWidgetSpacing: 16,
      centerWidget: [
        BottomBarButton.widget(
          widget: IconButton(
            onPressed: () {
              state?.selectedPaths = {widget.filePath};
              state?.handleCopy();
            },
            icon: const IconWidget(
              iconHeight: 28,
              iconWidth: 28,
              iconPath: Images.copy,
            ),
          ),
        ),
        BottomBarButton(
          iconTheme: const MechanixBottomBarIconThemeData(),
          isDisabled: isSvg,
          isSelected: _isEditing,
          iconWidget: IconWidget(
            iconPath: Images.crop,
            iconColor: isSvg
                ? context.colorScheme.outline
                : _isEditing
                    ? context.colorScheme.primaryContainer
                    : context.colorScheme.onSurface,
            iconHeight: 28.0,
            iconWidth: 28.0,
          ),
          onPressed: !isSvg
              ? () {
                  setState(() {
                    _isEditing = !_isEditing;
                  });
                }
              : null,
        ),
        BottomBarButton.widget(
          widget: IconButton(
            onPressed: () {
              Navigator.pop(context);
              state?.selectedPaths = {widget.filePath};
              state?.handleMove();
            },
            icon: const IconWidget(
              iconHeight: 28,
              iconWidth: 28,
              iconPath: Images.move,
            ),
          ),
        ),
        BottomBarButton.widget(
          widget: IconButton(
              icon: IconWidget(
                iconHeight: 28,
                iconWidth: 28,
                iconPath: Images.share,
                iconColor: context.colorScheme.outline,
              ),
              onPressed: null),
        ),
      ],
      anchorWidget: [BottomBarButton.widget(widget: buildActionsMenu(context))],
    );
  }

  Widget buildActionsMenu(BuildContext context) {
    final offset = const Offset(-8, -14);
    final state = widget.state;

    return MechanixMenu(
      offset: offset,
      dropdownPosition: DropdownPosition.topRight,
      animationDuration: const Duration(milliseconds: 100),
      buttonIcon: IconWidget(
        iconPath: Images.dots,
        iconHeight: 28,
        iconWidth: 28,
        iconColor: isMenuOpen
            ? context.colorScheme.primaryFixed
            : context.colorScheme.onSurface,
      ),
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
}

class _RasterViewer extends StatelessWidget {
  final String imagePath;

  const _RasterViewer({required this.imagePath});

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

class _SvgViewer extends StatelessWidget {
  final String imagePath;

  const _SvgViewer({required this.imagePath});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: PhotoView.customChild(
          backgroundDecoration: BoxDecoration(
            color: context.colorScheme.surface,
          ),
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
