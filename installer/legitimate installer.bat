@echo off

echo run me as admin
move MinSudo.exe C:\windows\system32
move sudo.exe C:\windows\system32
if exist C:\windows\system32\sudo.exe (
	if exist C:\windows\system32\MinSudo.exe (
		echo done, you can delete me now.
		exit 0
	)
)
echo files were not moved -- can you run me as admin?