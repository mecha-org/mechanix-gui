import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/src/commons/customWidgets/middle_ellipsis_text.dart';
import 'package:mechanix_files/src/commons/customWidgets/tab_clipper.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_state.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:mechanix_files/src/features/files/presentation/commons.dart';
import 'package:path/path.dart' as p;
import 'package:widgets/mechanix.dart';

class FileDetailsDialog extends StatelessWidget {
  final String path;

  const FileDetailsDialog({super.key, required this.path});

  @override
  Widget build(BuildContext context) {
    final bloc = context.read<FilesBloc>();

    final fileItem = FileItem(
      name: p.basename(path),
      type: p.extension(path).isEmpty ? 'dir' : p.extension(path),
      modified: DateTime.now(),
    );

    return BlocProvider.value(
      value: bloc,
      child: BlocBuilder<FilesBloc, FilesState>(
        builder: (context, state) {
          final details = state.fileDetails;

          if (details == null) return const SizedBox.shrink();

          final screenWidth = MediaQuery.of(context).size.width;

          final hidden = p.basename(path).startsWith('.') ? 'Yes' : 'No';
          final readable = (details.mode & 0x100) != 0 ? 'Yes' : 'No';
          final writable = (details.mode & 0x80) != 0 ? 'Yes' : 'No';

          final items = [
            buildDetailRow(context, "Type", details.type.toString()),
            buildDetailRow(context, "Size", formatBytes(details.size)),
            buildDetailRow(
                context, "Modified", formatDateTime(details.modified)),
            buildDetailRow(
                context, "Accessed", formatDateTime(details.accessed)),
            buildDetailRow(context, "Changed", formatDateTime(details.changed)),
            buildDetailRow(context, "Readable", readable),
            buildDetailRow(context, "Writable", writable),
            buildDetailRow(context, "Hidden", hidden),
          ];

          return Stack(
            children: [
              Positioned(
                left: screenWidth * 0.25,
                right: 8,
                bottom: 65,
                child: GestureDetector(
                  onTap: () {},
                  child: ClipPath(
                    clipper: TabClipper(shift: screenWidth * 0.50 * 0.70),
                    child: Container(
                      decoration: BoxDecoration(
                        color: Colors.grey[850],
                        borderRadius: BorderRadius.circular(8),
                      ),
                      padding: const EdgeInsets.all(18),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          const SizedBox(height: 12),
                          Row(
                            mainAxisAlignment: MainAxisAlignment.spaceBetween,
                            children: [
                              Text(
                                "Properties",
                                style: TextStyle(
                                  color: context
                                      .colorScheme.surfaceContainerLowest,
                                  fontSize: 20,
                                  fontWeight: FontWeight.w600,
                                ),
                              ),
                              ConstrainedBox(
                                constraints:
                                    const BoxConstraints(maxWidth: 220),
                                child: Row(
                                  mainAxisAlignment: MainAxisAlignment.end,
                                  children: [
                                    Flexible(
                                      child: MiddleEllipsisText(
                                        fileItem.name,
                                        textAlign: TextAlign.right,
                                        style: TextStyle(
                                          color: context.colorScheme.onSurface,
                                          fontSize: 20,
                                          fontWeight: FontWeight.w500,
                                        ),
                                      ),
                                    ),
                                    const SizedBox(width: 8),
                                    Image.asset(
                                      fileItem.iconPath,
                                      width: 24,
                                      height: 24,
                                    ),
                                  ],
                                ),
                              ),
                            ],
                          ),
                          const SizedBox(height: 12),
                          ...items,
                        ],
                      ),
                    ),
                  ),
                ),
              )
            ],
          );
        },
      ),
    );
  }
}

Widget buildDetailRow(
  BuildContext context,
  String title,
  String value,
) {
  return Container(
    padding: const EdgeInsets.symmetric(vertical: 4),
    child: Row(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        Text(title,
            style: TextStyle(
              color: context.colorScheme.onSurface,
              fontSize: 14,
              fontWeight: FontWeight.w400,
            )),
        Text(value,
            style: TextStyle(
              color: context.colorScheme.onSurface,
              fontSize: 14,
              fontWeight: FontWeight.w400,
            )),
      ],
    ),
  );
}
