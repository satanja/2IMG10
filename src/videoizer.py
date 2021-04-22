import os
import cv2


def create_video(folder_name: str, video_name: str, fps=30):
    """
    Creates a video from all the .png files located in the figures folder

    :param folder_name: the name of the folder that is inside the figures folder where the images are located
    :param video_name: the name of the video
    :param fps: frames per second, or in other words, how many figures per second to show
    """
    image_folder = f'../data/{folder_name}'
    video_name = f'{image_folder}/{video_name}.avi'

    images = [img for img in os.listdir(image_folder) if img.endswith(".png")]
    frame = cv2.imread(os.path.join(image_folder, images[0]))
    height, width, layers = frame.shape

    fourcc = cv2.VideoWriter_fourcc(*'MJPG')
    video = cv2.VideoWriter(video_name, fourcc, fps, (width, height))

    done = 0
    for image in images:
        video.write(cv2.imread(os.path.join(image_folder, image)))

        # progress bar
        done += 1
        percent_done = int((done * 100) / len(images))
        left = 100 - percent_done
        print('[' + '#' * percent_done + '-' * left + ']', end='\r')

    cv2.destroyAllWindows()
    video.release()
    print("done!")


create_video('2d-figures', '2d-movie')

