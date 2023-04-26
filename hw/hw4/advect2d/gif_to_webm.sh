ffmpeg -y -hide_banner -i advect2d_anim.gif -pix_fmt yuv420p -c:v libvpx-vp9 -crf 18 -b:v 0 advect2d_anim.webm
