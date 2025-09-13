class Note {
  final String id;
  final String title;
  final String content;
  final DateTime createdAt;
  final DateTime lastEditedAt;


  Note({
    required this.id,
    required this.title,
    required this.content,
    required this.createdAt,
    required this.lastEditedAt,
  });

  Note copyWith({
    String? title,
    String? content,
  }) {
    return Note(
      id: id,
      title: title ?? this.title,
      content: content ?? this.content,
      createdAt: createdAt,
      lastEditedAt: lastEditedAt
    );
  }
}
