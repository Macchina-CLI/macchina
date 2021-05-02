#!/usr/bin/env sh

# Will only work on macchina v0.7.3 or higher
# This script will download and run a video from youtube / any site supported by youtube-dl
# and display the video in macchina.
# The flow is 
# youtube-dl -> ffmpeg -> jp2a -> macchina
# First argument is video url.
# Second argument is frame wait time.

PID=$$
DIR="/tmp/ffmpeg_$PID"

if [ -n "$1" ];then
    URL="$1"
else
    URL=$(echo "aHR0cHM6Ly93d3cueW91dHViZS5jb20vd2F0Y2g/dj1kUXc0dzlXZ1hjUQo=" | base64 -d)
fi

if [ -n "$2" ];then
    FRAME_WAIT_TIME=$2
else 
    FRAME_WAIT_TIME=5
fi

required="youtube-dl ffmpeg base64 awk jp2a macchina"

for r in $required;do
    if ! [ -n "$(which $r 2> /dev/null)" ];then # need the quotes
        printf '\x1b[31m%s not found\x1b[0m\n' "$r"
        exit 1
    fi
done


# polling rate is .05 i.e. once every 50ms
WAIT=$(echo - | awk -v seconds="$FRAME_WAIT_TIME" '{print seconds/.05}')

trap_ctrlc() {

    printf '\x1b[?25h' # shows cursor
    if [ -n "$FFMPEG_PID" -a -d "/proc/$FFMPEG_PID" ];then
        kill -0 $FFMPEG_PID
        wait $FFMPEG_PID
    fi

    if [ -n "$DIR" -a -d "$DIR" ];then
        rm -rf "$DIR" 2> /dev/null
    fi

    exit
}

mkdir $DIR

# youtube-dl -f best $URL -o - | ffmpeg -i pipe: -r 10 -update 1 "$DIR/out_%d.png" > /dev/null 2>&1 &
youtube-dl -f best $URL -o - 2>/dev/null | ffmpeg -i pipe: -r 10 "$DIR/out_%d.png" > /dev/null 2>&1 &
FFMPEG_PID=$!

trap trap_ctrlc INT

printf '\x1b[?25l' # hides the cursor
for img in $(seq 1 999999);do # increasing this too much will break it
    count=0
    while ! [ -f "$DIR/out_$img.png" ];do 
        sleep .05
        count=$((count+1))
        if [ $count -ge $WAIT ];then break;fi
    done
    printf '\x1b[s' # saves cursor position
    macchina --custom-ascii <(jp2a --color --width=50 $DIR/out_$img.png)
    # jp2a --color --width=50 $DIR/out_$img.png # just display the video wihout macchina
    printf '\x1b[u'
    if [ -f  "$DIR/out_$img.png" ];then
        rm -f $DIR/out_$img.png
    fi
    sleep .02
done

printf '\x1b[?25h' # shows cursor


if [ -n "$FFMPEG_PID" -a -d "/proc/$FFMPEG_PID" ];then
    kill -0 $FFMPEG_PID
    wait $FFMPEG_PID
fi

if [ -n "$DIR" -a -d "$DIR" ];then
    rm -rf "$DIR" 2> /dev/null
fi

wait $FFMPEG_PID
