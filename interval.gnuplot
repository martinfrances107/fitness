# dark mode colors.
set terminal png transparent truecolor
set border lc rgb 'white'
set xlabel 'tick' tc rgb 'white'
set ylabel 'interval (hrs)' tc rgb 'white'
set title "Time between events (hrs)" tc rgb 'white'
set key off
plot [1:] "/dev/stdin" using ($0+1):1 with linespoints