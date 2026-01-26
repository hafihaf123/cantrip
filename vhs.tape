Require cantrip
Require tmux

Output vhs.gif

Set Width 1600
Set Height 800
Set FontSize 16
Set Padding 20

# 1. Setup Tmux (Hide this part so the viewer just sees the app start)
Hide
Type "tmux new-session -d -s cantrip_demo"
Enter
Type "tmux split-window -h -t cantrip_demo"
Enter
# Resize panes if necessary (optional)
# Type "tmux resize-pane -L 10"
# Enter
Type "tmux attach -t cantrip_demo"
Enter
Show

Type "cantrip -u DungeonMaster -r Strahd open"
Enter
Wait /password/
Type "password"
Enter
Sleep 2s

Ctrl+B
Type "o"
Sleep 1s

Type "cantrip -u Player -r Strahd join"
Enter
Wait /ticket/
Paste
Enter
Wait /password/
Type "password"
Enter
Sleep 2s

Ctrl+B
Type "o"

Type "You find yourselves at the gates of Barovia. The mist is thick."
Enter
Sleep 2s

Ctrl+B
Type "o"
Type "I look around for threats."
Enter
Sleep 1s
Type "/roll 1d20+3"
Enter
Sleep 2s

Ctrl+B
Type "o"
Type "You see a dark shape looming near the gate..."
Enter
Sleep 3s

Ctrl+B
Type "o"
Type "/quit"
Enter
Sleep 3s


Ctrl+B
Type "o"
Type "/quit"
Enter
Sleep 3s

Hide
Type "tmux kill-session -t cantrip_demo"
Enter
