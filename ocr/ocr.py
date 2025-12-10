import PIL.Image
from manga_ocr import MangaOcr

mocr = MangaOcr()

def ocr_image(image_path):
    """
    Perform OCR on the given image and return the recognized text.

    Args:
        image_path (str): The path to the image file.
    Returns:
        str: The recognized text from the image.
    """
    img = PIL.Image.open(image_path)
    result = mocr(img)
    return result