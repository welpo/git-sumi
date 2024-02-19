# git-sumi demo tape

####################################################
# vhs config: https://github.com/charmbracelet/vhs #
#  create the video with: vhs git-sumi_demo.tape   #
####################################################
Set Shell "bash"
Set FontSize 32
Set Width 1450
Set Height 800
Set FontFamily "Monolisa Script"
# Set Theme "Catppuccin Frappe"
Set CursorBlink false
Set TypingSpeed 85ms
Output git-sumi_demo.mp4
# Output frames/ # a directory of frames as a PNG sequence
Require git-sumi

#####################
# Prepare the stage #
#####################
Hide
Type "tmpdir=$(mktemp -d)" Enter
Type 'cd "$tmpdir"' Enter
Type 'git init' Enter
Type 'clear' Enter
Show

####################
# Actual recording #
####################
Sleep 1.2s
Type 'git sumi "fixes a bug where users could bypass authentication by providing an empty password"'
Sleep 500ms
Enter
Enter
Sleep 2.3s
Type "# Enable 3 rules: max header length of 50 (H 50),"
Enter
Type "# imperative mood (I), and uppercase description (E upper)."
Sleep 200ms
Enter
Sleep 3s
Type 'git sumi -H 50 -I -E upper "fixes a bug where users could bypass authentication by providing an empty password"'
Sleep 1.2s
Enter
Enter
Sleep 8s
Type "# Let's fix the errors."
Enter
Sleep 1.2s
Type@102ms 'git sumi -H 50 -I -E upper '
Type@72ms '"Fix empty password authentication bypass bug"'
Sleep 0.8s
Enter
Enter
Sleep 1.8s
Type "# Nice! "
Sleep 500ms
Type "Let's create a config for these rules."
Sleep 1.5s
Enter
Type "# First, we initialize the default config."
Enter
Sleep 1s
Type "git sumi --init"
Enter
Enter
Sleep 1s
Type "# Now we edit the file to enable the rules."
Enter
Sleep 500ms
Type "$EDITOR sumi.toml"
Sleep 800ms
Enter
Sleep 1.5s
Type "/description_case" Enter
Type '$BC"upper"' Escape
Sleep 800ms
Type "jjjjciwtrue" Escape
Sleep 720ms
Type "/0" Enter
Type "ns50" Escape
Sleep 1.1s
Type ":wq"
Enter
Enter
Sleep 2s
Type "# git-sumi will read 'sumi.toml' from the current directory."
Enter
Sleep 200ms
Type 'git sumi "Fix empty password authentication bypass bug"'
Enter
Sleep 20ms
Enter
Sleep 1s
Type "# Let's set up a Git hook that validates all commits."
Enter
Sleep 1s
Type "git sumi --init commit-msg"
Enter
Enter
Sleep 1.5s
Type "touch test"
Enter
Sleep 100ms
Type "git add test"
Enter
Sleep 80ms
Type "git commit -m 'implements caching mechanism'"
Sleep 400ms
Enter
Sleep 2.2s
Enter
Type '# It worked!' Enter
Sleep 0.2s
Type '# Visit https://sumi.rs to improve your commits~'
Sleep 5s


###########
# Cleanup #
###########
Hide
Type 'rm -rf "$tmpdir"' Enter
Show
