import 'dart:ui';

class Images {
  static const String back = 'assets/images/back.png';
  static const String add = 'assets/images/add.png';
  static const String arrow = 'assets/images/arrow.png';
  static const String dots = 'assets/images/dots_three_vertical.png';
  static const String file = 'assets/images/file.png';
  static const String folder = 'assets/images/fold.png';
  static const String imageFile = 'assets/images/image_file.png';
  static const String pdfFile = 'assets/images/pdf.png';
  static const String unfoldDir = 'assets/images/unfold_dir.png';
  static const String textFile = 'assets/images/text_file.png';
  static const String audioFile = 'assets/images/audio_file.png';
  static const String videoFile = 'assets/images/video_file.png';
  static const String list = 'assets/images/list.png';
  static const String grid = 'assets/images/grid.png';
  static const String listChecks = 'assets/images/list_checks.png';
  static const String listChecksAll = 'assets/images/list_checks_all.png';
  static const String excelFile = 'assets/images/excel_file.png';
  static const String csvFile = 'assets/images/csv.png';
  static const String archiveFile = 'assets/images/archive_file.png';
  static const String sortAscending = 'assets/images/sort_ascending.png';
  static const String sortDescending = 'assets/images/sort_descending.png';
  static const String recent = 'assets/images/recent.png';
  static const String compress = 'assets/images/compress.png';
  static const String copy = 'assets/images/copy.png';
  static const String createFolder = 'assets/images/create_folder.png';
  static const String delete = 'assets/images/delete.png';
  static const String extract = 'assets/images/extract.png';
  static const String eyeSlash = 'assets/images/eye_slash.png';
  static const String eye = 'assets/images/eye.png';
  static const String hardDrive = 'assets/images/hard_drive.png';
  static const String homeDocuments = 'assets/images/home_documents.png';
  static const String home = 'assets/images/home.png';
  static const String info = 'assets/images/info.png';
  static const String move = 'assets/images/move.png';
  static const String rename = 'assets/images/rename.png';
  static const String search = 'assets/images/search.png';
  static const String usb = 'assets/images/usb.png';
  static const String xCircle = 'assets/images/x_circle.png';
  static const String paste = 'assets/images/paste.png';
  static const String arrowUp = 'assets/images/arrow_up.png';
  static const String unfoldMore = 'assets/images/unfold_more.png';
  static const String downloads = 'assets/images/downloads.png';
  static const String terminal = 'assets/images/terminal.png';
  static const String refresh = 'assets/images/refresh.png';
  static const String codeFile = 'assets/images/code.png';
  static const String checkCircle = 'assets/images/check_circle.png';
  static const String share = 'assets/images/share.png';
  static const String duplicate = 'assets/images/duplicate.png';
  static const String musicNote = 'assets/images/music_note.png';
  static const String play = 'assets/images/play.png';
  static const String pause = 'assets/images/pause.png';
  static const String volume = 'assets/images/volume.png';
  static const String mute = 'assets/images/mute.png';
  static const String crop = 'assets/images/crop.png';
  static const String rotateRight = 'assets/images/rotate_right.png';
  static const String mirrorVertical = 'assets/images/mirror_vertical.png';
  static const String mirrorHorizontal = 'assets/images/mirror_horizontal.png';
  static const String redo = 'assets/images/redo.png';
  static const String undo = 'assets/images/undo.png';
  static const String check = 'assets/images/check.png';
  static const String cut = 'assets/images/cut.png';
  static const String lock = 'assets/images/lock.png';
  static const String edit = 'assets/images/edit.png';
}

class HiveTables {
  static const String appSettingsTable = "appSettingsTable";
  static const String recentFilesTable = "recentFilesTable";
}

class AppPaths {
  static const String homeDir = '/home/mecha';
  static const String downloadsDir = '/home/mecha/Downloads';
  static const String documentsDir = '/home/mecha/Documents';

  /// Virtual recent page (not filesystem path)
  static const String recentDir = '/recent';

  /// PDFium native library
  static const String pdfiumModulePath = '/usr/lib64/libpdfium.so';
}

class AppLimits {
  /// Max recent files stored
  static const int recentFilesCount = 50;
}
