from PIL import Image, ImageDraw

def create_icon(size, filename):
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    # Dégradé bleu -> violet (couleurs Initium)
    for i in range(size):
        r = int(102 + (118 - 102) * (i / size))  # 667eea -> 764ba2
        g = int(126 + (75 - 126) * (i / size))
        b = int(234 + (162 - 234) * (i / size))
        draw.line([(0, i), (size, i)], fill=(r, g, b, 255))
    
    # Cercle blanc au centre
    center = size // 2
    radius = int(size * 0.3)
    draw.ellipse(
        [center - radius, center - radius, center + radius, center + radius],
        fill=(255, 255, 255, 200)
    )
    
    img.save(filename)
    print(f"✅ {filename} created ({size}x{size})")

# Créer les différentes résolutions
create_icon(32, 'icons/icon.png')
create_icon(128, 'icons/icon-128x128.png')
create_icon(256, 'icons/icon-256x256.png')
create_icon(512, 'icons/icon-512x512.png')

# Pour Windows (ICO format)
icon_png = Image.open('icons/icon.png')
icon_png.save('icons/icon.ico')
print("✅ icons/icon.ico created")
