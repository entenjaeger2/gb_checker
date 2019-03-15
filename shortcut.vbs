Set wshShell = CreateObject("WScript.Shell")
Set fso = CreateObject("Scripting.FileSystemObject")
Set args = WScript.Arguments

If args.Count < 1 Then
WScript.Echo "File name missing!"
WScript.Quit
End If

target = args(0)
target = fso.GetAbsolutePathName(target)
idxName = InStrRev(target, "\")
idxExt = InStrRev(target, ".")
If idxExt < idxName Then idxExt = Len(target) + 1
directory = Left(target, idxName - 1)
linkName = Mid(target, idxName + 1, idxExt - idxName - 1)
startup = wshShell.SpecialFolders("Startup")

Set link = wshShell.CreateShortcut(startup & "\" & linkName & ".lnk")
link.TargetPath = target
link.WorkingDirectory = directory
link.Save