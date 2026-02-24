import 'dart:io';

import 'package:ellipsized_text/ellipsized_text.dart';
import 'package:flutter/material.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/menu/constants/menu_positions.dart';
import 'package:widgets/widgets/menu/mechanix_menu_theme.dart';
import 'package:widgets/widgets/menu/models/mechanix_menu_item.dart';
import 'package:path/path.dart' as path;

class ImagePropertiesMenu extends StatefulWidget {
  final String imagePath;

  const ImagePropertiesMenu({super.key, required this.imagePath});

  @override
  State<ImagePropertiesMenu> createState() => _ImagePropertiesMenuState();
}

class _ImagePropertiesMenuState extends State<ImagePropertiesMenu> {
  FileStat? _stat;
  bool _loading = true;

  @override
  void initState() {
    super.initState();
    _loadProperties();
  }

  Future<void> _loadProperties() async {
    try {
      final stat = await FileStat.stat(widget.imagePath);
      setState(() {
        _stat = stat;
        _loading = false;
      });
    } catch (_) {
      setState(() => _loading = false);
    }
  }

  String _formatDateTime(DateTime? dt) {
    if (dt == null) return '—';
    final day = _p(dt.day);
    final month = _p(dt.month);
    final year = dt.year;
    final hour = dt.hour % 12 == 0 ? 12 : dt.hour % 12;
    final minute = _p(dt.minute);
    final period = dt.hour < 12 ? 'am' : 'pm';
    return '$day-$month-$year, $hour.$minute$period';
  }

  String _p(int v) => v.toString().padLeft(2, '0');

  String _formatSize(int bytes) {
    if (bytes < 1024) return '$bytes B';
    if (bytes < 1024 * 1024) return '${(bytes / 1024).round()} KB';
    return '${(bytes / (1024 * 1024)).round()} MB';
  }

  String _getType(String path) {
    final ext = path.split('.').last.toUpperCase();
    return ext.isNotEmpty ? '$ext Image' : 'Unknown';
  }

  String _modeReadable(int mode) {
    // Check owner read bit (0400)
    return (mode & 0x100) != 0 ? 'Yes' : 'No';
  }

  String _modeWritable(int mode) {
    // Check owner write bit (0200)
    return (mode & 0x80) != 0 ? 'Yes' : 'No';
  }

  String _isHidden(String path) {
    final name = path.split(Platform.pathSeparator).last;
    return name.startsWith('.') ? 'Yes' : 'No';
  }

  @override
  Widget build(BuildContext context) {
    if (_loading) {
      return const Center(child: CircularProgressIndicator());
    }

    final stat = _stat;
    final textStyle = TextStyle(
      color: context.colorScheme.onSurface,
      fontSize: 18,
      fontWeight: FontWeight.w400,
    );

    List<({String label, String value})> properties = [
      (label: 'Type', value: _getType(widget.imagePath)),
      (label: 'Type', value: _getType(widget.imagePath)),
      (label: 'Size', value: stat != null ? _formatSize(stat.size) : '—'),
      (label: 'Modified', value: _formatDateTime(stat?.modified)),
      (label: 'Accessed', value: _formatDateTime(stat?.accessed)),
      (label: 'Changed', value: _formatDateTime(stat?.changed)),
      (label: 'Readable', value: stat != null ? _modeReadable(stat.mode) : '—'),
      (label: 'Writable', value: stat != null ? _modeWritable(stat.mode) : '—'),
      (label: 'Hidden', value: _isHidden(widget.imagePath)),
    ];

    return MechanixMenu(
      animationDuration: const Duration(milliseconds: 100),
      theme: const MechanixMenuThemeData(
        itemPadding: EdgeInsets.symmetric(vertical: 8, horizontal: 8),
      ),
      dropdownSize: const Size(405, 400),
      dropdownPosition: MenuDropdownPosition.topEnd,
      items: [
        MechanixMenuItemsType(
          title: 'Properties',
          trailing: _topHeader(textStyle),
        ),
        ...properties.map(
          (p) => MechanixMenuItemsType(
            title: '',
            leading: Text(p.label, style: textStyle),
            trailing: Text(p.value, style: textStyle),
          ),
        ),
      ],
    );
  }

  Widget _topHeader(TextStyle style) {
    final fileName = path.basename(widget.imagePath);

    return Row(
      children: [
        ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 180),
          child: EllipsizedText(
            type: EllipsisType.middle,
            fileName,
            style: TextStyle(
              color: context.colorScheme.onSurface,
              fontSize: 20,
            ),
          ),
        ),
        const SizedBox(width: 8),
        ClipRRect(
          borderRadius: BorderRadius.circular(4),
          child: Image.file(
            File(widget.imagePath),
            width: 20,
            height: 20,
            fit: BoxFit.cover,
          ),
        ),
      ],
    );
  }
}
