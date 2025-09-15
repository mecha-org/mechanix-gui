import 'package:equatable/equatable.dart';
import 'package:mechanix_notes/models/note_hive.dart';

class NoteItem extends Equatable {
  final String id;
  final String title;
  final String content;
  final DateTime createdAt;
  final DateTime updatedAt;
  final String plainText;
  final bool isPinned;
  final String tag;

  const NoteItem({
    required this.id,
    required this.title,
    required this.content,
    required this.createdAt,
    required this.updatedAt,
    required this.plainText,
    this.isPinned = false,
    this.tag = 'none',
  });

  // Factory constructor to create NoteItem from NoteHive
  factory NoteItem.fromNoteHive(NoteHive noteHive) {
    return NoteItem(
      id: noteHive.id,
      title: noteHive.title,
      content: noteHive.content,
      createdAt: noteHive.createdAt,
      updatedAt: noteHive.updatedAt,
      plainText: noteHive.plainText,
      isPinned: noteHive.isPinned,
      tag: noteHive.tag,
    );
  }

  // Method to convert back to NoteHive
  NoteHive toNoteHive() {
    return NoteHive(
      id: id,
      title: title,
      content: content,
      createdAt: createdAt,
      updatedAt: updatedAt,
      plainText: plainText,
      isPinned: isPinned,
      tag: tag,
    );
  }

  // CopyWith method for easy updates
  NoteItem copyWith({
    String? id,
    String? title,
    String? content,
    DateTime? createdAt,
    DateTime? updatedAt,
    String? plainText,
    bool? isPinned,
    String? tag,
  }) {
    return NoteItem(
      id: id ?? this.id,
      title: title ?? this.title,
      content: content ?? this.content,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
      plainText: plainText ?? this.plainText,
      isPinned: isPinned ?? this.isPinned,
      tag: tag ?? this.tag,
    );
  }

  @override
  List<Object?> get props => [
    id,
    title,
    content,
    createdAt,
    updatedAt,
    plainText,
    isPinned,
    tag,
  ];
}
