import 'package:flutter/widgets.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_bloc.dart';
import 'package:mechanix_camera/src/bloc/camera_state.dart';
import 'package:mechanix_camera/src/media/widgets/image_view.dart';

class MediaView extends StatelessWidget {
  const MediaView({super.key});

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      child: BlocSelector<CameraBloc, CameraState, List<String>>(
        selector: (state) => state.mediaPath,
        builder: (context, state) => ImageView(imagePath: state.first),
      ),
    );
  }
}
