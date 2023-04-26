ffmpeg -y -hide_banner -i advect2d_anim.gif -pix_fmt yuv420p -c:v libx264 -preset slow -crf 20 -movflags +faststart advect2d_anim.mp4
