cd ~/rb/a11y/src/launcher || exit 1
B=$CARGO_TARGET_DIR/debug/anyplay-launcher
mkdir -p ~/rb/a11y-after && rm -f ~/rb/a11y-after/*.png
export DISPLAY=:177
Xvfb :177 -screen 0 1920x1080x24 -nolisten tcp >/dev/null 2>&1 &
X=$!; sleep 2
for s in notifications notifications-long settings toolkit library library-longname friends-activity-error friends-activity-empty post-listing controllers-test home-empty home-nofolder captures-clip captures-many home-shelves trophies-all about-update; do
  home=$(mktemp -d)
  env -u WAYLAND_DISPLAY HOME=$home XDG_DATA_HOME=$home/data XDG_CONFIG_HOME=$home/config VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json timeout 120 $B --gallery=$s --screenshot=$HOME/rb/a11y-after/$s.png --size=1920x1080 --frames=100 >$HOME/rb/a11y-after/$s.log 2>&1 || echo FAIL $s
  rm -rf $home
done
kill $X
ls ~/rb/a11y-after/*.png | wc -l
