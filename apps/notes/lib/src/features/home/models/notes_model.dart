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

  Note copyWith({String? title, String? content}) {
    return Note(
      id: id,
      title: title ?? this.title,
      content: content ?? this.content,
      createdAt: createdAt,
      lastEditedAt: lastEditedAt,
    );
  }
}

class GroupedNotes {
  final String label; // e.g. "This Month", "August 2025"
  final List<NoteMetaData> notes;
  final double height;
  GroupedNotes({
    required this.label,
    required this.notes,
    required this.height,
  });
}

class NoteMetaData {
  final String id;
  final String title;
  final DateTime createdAt;
  final DateTime updatedAt;
  final List<NoteLine> preview;
  final double height;

  const NoteMetaData({
    required this.id,
    required this.title,
    required this.createdAt,
    required this.updatedAt,
    required this.preview,
    required this.height,
  });
}

class SearchMetaData {
  final String id;
  final String text;
  final DateTime updatedAt;
  final bool isTitle;
  final int availableCount;
  final int titleMatchCount;
  final int contentMatchCount;

  SearchMetaData({
    required this.id,
    required this.text,
    required this.updatedAt,
    required this.isTitle,
    required this.availableCount,
    required this.titleMatchCount,
    required this.contentMatchCount,
  });
}

class NoteLine {
  final String type;
  final List<NoteSpan> spans;
  final bool? checked;

  NoteLine({required this.type, required this.spans, this.checked});

  Map<String, dynamic> toJson() => {
    'type': type,
    'spans': spans.map((e) => e.toJson()).toList(),
    if (checked != null) 'checked': checked,
  };

  factory NoteLine.fromJson(Map<String, dynamic> json) => NoteLine(
    type: json['type'],
    spans: (json['spans'] as List).map((e) => NoteSpan.fromJson(e)).toList(),
    checked: json['checked'],
  );
}

class NoteSpan {
  String text;
  bool bold;
  bool italic;
  bool underline;
  bool strike;
  String? color;
  String? background;

  NoteSpan({
    required this.text,
    this.bold = false,
    this.italic = false,
    this.underline = false,
    this.strike = false,
    this.color,
    this.background,
  });

  Map<String, dynamic> toJson() => {
    "text": text,
    "bold": bold,
    "italic": italic,
    "underline": underline,
    "strike": strike,
    "color": color,
    "background": background,
  };

  factory NoteSpan.fromJson(Map<String, dynamic> json) => NoteSpan(
    text: json["text"],
    bold: json["bold"] ?? false,
    italic: json["italic"] ?? false,
    underline: json["underline"] ?? false,
    strike: json["strike"] ?? false,
    color: json["color"],
    background: json["background"],
  );
}

class SectionInfo {
  final String label;
  final double offset;
  final double height;

  SectionInfo({
    required this.label,
    required this.offset,
    required this.height,
  });
}
