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
Sleep 1s
Type "# First step: initialize git-sumi in your repository."
Sleep 500ms
Enter
Type "git sumi --init"
Enter
Enter
Sleep 1s
Type "# Let's edit the default config to enable three rules:"
Enter
Sleep 1s
Type "# max header length of 50, use the imperative mood,"
Sleep 200ms
Enter
Type "# and start the description in uppercase."
Enter
Sleep 500ms
Type "$EDITOR sumi.toml"
Sleep 800ms
Enter
Sleep 1.5s
Type "/description_case" Enter
Type '$BC"upper"' Escape
Sleep 850ms
Type "jjjjciwtrue" Escape
Sleep 780ms
Type "/0" Enter
Type "ns50" Escape
Sleep 1.7s
Type ":wq"
Enter
Enter
Sleep 2s
Type "# Time to see git-sumi in action."
Enter
Sleep 800ms
Type 'git sumi "fixes a bug where users could bypass authentication by providing an empty password"'
Enter
Enter
Sleep 10s
Type "# Let's fix the errors."
Enter
Sleep 1.2s
Type@102ms 'git sumi '
Type@72ms '"Fix empty password authentication bypass bug"'
Sleep 0.8s
Enter
Enter
Sleep 1.8s
Type "# Nice!"
Enter
Sleep 1.5s
Type "# Now we'll set up a Git hook to validate all commits."
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
Sleep 3s


###########
# Cleanup #
###########
Hide
Type 'rm -rf "$tmpdir"' Enter
Show
