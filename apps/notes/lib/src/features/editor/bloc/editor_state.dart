import 'package:equatable/equatable.dart';
import 'package:flutter/material.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/features/home/models/toolbar_model.dart';

class EditorBlocState extends Equatable {
  final bool isEditing;
  final bool isUndo;
  final bool isRedo;
  final bool isPinned;
  final bool toolbarToggle;
  final LayerLink linkLayer;
  final LayerLink optionsLayer;
  final ToolbarEnum? selectedToolbar;

  const EditorBlocState({
    required this.isEditing,
    this.isUndo = false,
    this.isRedo = false,
    this.isPinned = false,
    this.toolbarToggle = false,
    required this.linkLayer,
    required this.optionsLayer,
    this.selectedToolbar,
  });

  EditorBlocState copyWith({
    bool? isEditing,
    bool? isUndo,
    bool? isRedo,
    bool? isPinned,
    bool? toolbarToggle,
    LayerLink? linkLayer,
    LayerLink? optionsLayer,
    ToolbarEnum? selectedToolbar,
    QuillController? controller,
  }) {
    return EditorBlocState(
      linkLayer: linkLayer ?? this.linkLayer,
      optionsLayer: optionsLayer ?? this.linkLayer,
      isEditing: isEditing ?? this.isEditing,
      isUndo: isUndo ?? this.isUndo,
      isRedo: isRedo ?? this.isRedo,
      isPinned: isPinned ?? this.isPinned,
      toolbarToggle: toolbarToggle ?? this.toolbarToggle,
      selectedToolbar: selectedToolbar,
    );
  }

  @override
  List<Object?> get props => [
    isEditing,
    isUndo,
    isRedo,
    isPinned,
    toolbarToggle,
    linkLayer,
    optionsLayer,
    selectedToolbar,
  ];
}
