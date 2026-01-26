Require cantrip
Require tmux

Output vhs.gif

Set WindowBar Colorful
Set Theme "GruvboxDark"
# Set FontFamily "JetBrains Mono"
Set FontSize 22
# Set LineHeight 1.2

Set Width 1600
Set Height 750
Set Padding 50
Set Margin 20
Set BorderRadius 10

Set TypingSpeed 50ms
Set PlaybackSpeed 1.0

Hide

Type "bash --noprofile --norc"
Enter
Type 'export PS1="\[\033[32;1m\]>\[\033[0m\] "'
Enter
Type "clear"
Enter

Type "tmux new-session -d -s cantrip_demo"
Enter

Type "tmux set-option -g default-command 'bash --noprofile --norc'"
Enter

Type "tmux set status off"
Enter
Type "tmux set pane-border-style fg='#444444'"
Enter
Type "tmux set pane-active-border-style fg='#444444'"
Enter

Type "tmux split-window -h -t cantrip_demo"
Enter

Type "tmux attach -t cantrip_demo"
Enter

Type 'export PS1="\[\033[1;32m\]>\[\033[0m\] "'
Enter
Type "clear"
Enter

Ctrl+B
Type "o"

Type 'export PS1="\[\033[1;32m\]>\[\033[0m\] "'
Enter
Type "clear"
Enter

Ctrl+B
Type "o"

Show

Sleep 1s

Type "cantrip -u DungeonMaster -r Strahd open"
Enter
Wait /password/
Sleep 500ms
Type "password"
Enter
Sleep 2s

Ctrl+B
Type "o"
Sleep 500ms

Type "cantrip -u Grog -r Strahd join "
Paste
Sleep 1s
Enter
Wait /password/
Sleep 500ms
Type "password"
Enter
Sleep 2s

Ctrl+B
Type "o"
Type "You find yourselves at the gates of Barovia."
Enter
Sleep 1.5s

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
Type "A dark shape looms near the gate..."
Enter
Sleep 3s

Type "/quit"
Enter
Sleep 500ms
Ctrl+B
Type "o"
Sleep 1s
Type "/quit"
Enter

Sleep 2s

Hide
Type "tmux kill-session -t cantrip_demo"
Enter
