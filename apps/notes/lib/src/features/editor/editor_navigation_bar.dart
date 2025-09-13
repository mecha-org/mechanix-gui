// import 'package:flutter/material.dart';
// import 'package:mechanix_notes/src/commons/icons.dart';
// import 'package:mechanix_notes/src/commons/styles/colors.dart';
// import 'package:widgets/mechanix.dart';

// class EditorNavigationBar extends StatelessWidget
//     implements PreferredSizeWidget {
//   final bool isUndo;
//   final bool isRedo;
//   final bool toolbarToggle;
//   final TextEditingController titleController;
//   final VoidCallback saveNotes;
//   final VoidCallback undoCall;
//   final VoidCallback redoCall;
//   final VoidCallback enableToolbar;
//   final VoidCallback showOptions;
//   final LayerLink? optionsLayer;

//   const EditorNavigationBar({
//     super.key,
//     required this.isUndo,
//     required this.isRedo,
//     required this.toolbarToggle,
//     required this.titleController,
//     required this.saveNotes,
//     required this.undoCall,
//     required this.redoCall,
//     required this.enableToolbar,
//     this.optionsLayer,
//     required this.showOptions,
//   });

//   @override
//   Widget build(BuildContext context) {
//     return MechanixNavigationBar(
//       leadingWidth: 320,
//       leadingWidget: Row(
//         children: [
//           IconButton(
//             icon: Image.asset(NotesIcon.backIcon, height: 20, width: 20),
//             onPressed: saveNotes,
//             padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 8),
//           ),
//           Expanded(
//             child: TextField(
//               controller: titleController,
//               maxLength: 25,
//               style: TextStyle(color: NotesColors.titleTextColor),
//               cursorColor: NotesColors.editorTextColor,
//               decoration: InputDecoration(
//                 counterText: "",
//                 hintText: "New Note",
//                 hintStyle: TextStyle(color: NotesColors.titleTextColor),
//                 border: InputBorder.none,
//                 isCollapsed: true,
//               ),
//             ),
//           ),
//         ],
//       ),

//       actionsIconTheme: IconThemeData(size: 20),
//       actionWidgets: [
//         if (!toolbarToggle)
//           IconButton(
//             onPressed: isUndo ? undoCall : null,
//             icon: SizedBox(
//               height: 20,
//               width: 20,
//               child: Image.asset(
//                 NotesIcon.undoIcon,
//                 color: isUndo ? Colors.white : Theme.of(context).disabledColor,
//               ),
//             ),
//           ).padRight(10),
//         if (!toolbarToggle) ...[
//           IconButton(
//             onPressed: isRedo ? redoCall : null,
//             icon: SizedBox(
//               height: 20,
//               width: 20,
//               child: Image.asset(
//                 NotesIcon.redoIcon,
//                 color: isRedo ? Colors.white : Theme.of(context).disabledColor,
//               ),
//             ),
//           ).padRight(10),
//         ],
//         IconButton(
//           onPressed: () {
//             enableToolbar();
//           },
//           icon: SizedBox(
//             height: 20,
//             width: 20,
//             child: Image.asset(
//               toolbarToggle
//                   ? NotesIcon.toolbarEnableIcon
//                   : NotesIcon.toolbarDisableIcon,
//             ),
//           ),
//         ),
//         CompositedTransformTarget(
//           link: optionsLayer!,
//           child: IconButton(
//             onPressed: () {
//               showOptions();
//             },
//             icon: SizedBox(
//               height: 20,
//               width: 20,
//               child: Image.asset(NotesIcon.threeDotIcon),
//             ),
//           ),
//         ),
//       ],
//     );
//   }

//   @override
//   Size get preferredSize => const Size.fromHeight(50);
// }
