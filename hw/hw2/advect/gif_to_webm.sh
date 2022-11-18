ffmpeg -y -hide_banner -i advect_anim.gif -pix_fmt yuv420p -c:v libvpx-vp9 -crf 18 -b:v 0 advect_anim.webm
